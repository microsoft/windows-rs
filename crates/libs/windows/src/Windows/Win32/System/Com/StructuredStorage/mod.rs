#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [::windows_core::PROPVARIANT]) {
    ::windows_targets::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, cvars : u32));
    ClearPropVariantArray(::core::mem::transmute(rgpropvar.as_ptr()), rgpropvar.len().try_into().unwrap())
}
#[inline]
pub unsafe fn CoGetInstanceFromFile<P0, P1>(pserverinfo: ::core::option::Option<*const super::COSERVERINFO>, pclsid: ::core::option::Option<*const ::windows_core::GUID>, punkouter: P0, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: P1, presults: &mut [super::MULTI_QI]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super:: COSERVERINFO, pclsid : *const ::windows_core::GUID, punkouter : * mut::core::ffi::c_void, dwclsctx : super:: CLSCTX, grfmode : u32, pwszname : ::windows_core::PCWSTR, dwcount : u32, presults : *mut super:: MULTI_QI) -> ::windows_core::HRESULT);
    CoGetInstanceFromFile(::core::mem::transmute(pserverinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), punkouter.into_param().abi(), dwclsctx, grfmode, pwszname.into_param().abi(), presults.len().try_into().unwrap(), ::core::mem::transmute(presults.as_ptr())).ok()
}
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<P0, P1>(pserverinfo: ::core::option::Option<*const super::COSERVERINFO>, pclsid: ::core::option::Option<*const ::windows_core::GUID>, punkouter: P0, dwclsctx: super::CLSCTX, pstg: P1, presults: &mut [super::MULTI_QI]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    P1: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super:: COSERVERINFO, pclsid : *const ::windows_core::GUID, punkouter : * mut::core::ffi::c_void, dwclsctx : super:: CLSCTX, pstg : * mut::core::ffi::c_void, dwcount : u32, presults : *mut super:: MULTI_QI) -> ::windows_core::HRESULT);
    CoGetInstanceFromIStorage(::core::mem::transmute(pserverinfo.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), punkouter.into_param().abi(), dwclsctx, pstg.into_param().abi(), presults.len().try_into().unwrap(), ::core::mem::transmute(presults.as_ptr())).ok()
}
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<P0, T>(pstm: P0) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<super::IStream>,
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("ole32.dll" "system" fn CoGetInterfaceAndReleaseStream(pstm : * mut::core::ffi::c_void, iid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    CoGetInterfaceAndReleaseStream(pstm.into_param().abi(), &T::IID, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal<P0, P1>(hglobal: P0, fdeleteonrelease: P1) -> ::windows_core::Result<ILockBytes>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::super::super::Foundation:: HGLOBAL, fdeleteonrelease : super::super::super::Foundation:: BOOL, pplkbyt : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateILockBytesOnHGlobal(hglobal.into_param().abi(), fdeleteonrelease.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateStreamOnHGlobal<P0, P1>(hglobal: P0, fdeleteonrelease: P1) -> ::windows_core::Result<super::IStream>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("ole32.dll" "system" fn CreateStreamOnHGlobal(hglobal : super::super::super::Foundation:: HGLOBAL, fdeleteonrelease : super::super::super::Foundation:: BOOL, ppstm : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateStreamOnHGlobal(hglobal.into_param().abi(), fdeleteonrelease.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const ::windows_core::GUID, oszname: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    ::windows_targets::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const ::windows_core::GUID, oszname : ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    FmtIdToPropStgName(pfmtid, ::core::mem::transmute(oszname)).ok()
}
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [::windows_core::PROPVARIANT]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("ole32.dll" "system" fn FreePropVariantArray(cvariants : u32, rgvars : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    FreePropVariantArray(rgvars.len().try_into().unwrap(), ::core::mem::transmute(rgvars.as_ptr())).ok()
}
#[inline]
pub unsafe fn GetConvertStg<P0>(pstg: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn GetConvertStg(pstg : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    GetConvertStg(pstg.into_param().abi()).ok()
}
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<P0>(plkbyt: P0) -> ::windows_core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: ::windows_core::IntoParam<ILockBytes>,
{
    ::windows_targets::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : * mut::core::ffi::c_void, phglobal : *mut super::super::super::Foundation:: HGLOBAL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetHGlobalFromILockBytes(plkbyt.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetHGlobalFromStream<P0>(pstm: P0) -> ::windows_core::Result<super::super::super::Foundation::HGLOBAL>
where
    P0: ::windows_core::IntoParam<super::IStream>,
{
    ::windows_targets::link!("ole32.dll" "system" fn GetHGlobalFromStream(pstm : * mut::core::ffi::c_void, phglobal : *mut super::super::super::Foundation:: HGLOBAL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetHGlobalFromStream(pstm.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: ::core::option::Option<&[super::super::super::Foundation::BOOL]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const super::super::super::Foundation:: BOOL, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromBooleanVector(::core::mem::transmute(prgf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const ::core::ffi::c_void, cb : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromBuffer(pv, cb, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const ::windows_core::GUID, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromCLSID(clsid, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: ::core::option::Option<&[f64]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const f64, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromDoubleVector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::super::super::Foundation:: FILETIME, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromFileTime(pftin, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: ::core::option::Option<&[super::super::super::Foundation::FILETIME]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::super::super::Foundation:: FILETIME, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromFileTimeVector(::core::mem::transmute(prgft.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const ::windows_core::GUID, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromGUIDAsString(guid, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: ::core::option::Option<&[i16]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const i16, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromInt16Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: ::core::option::Option<&[i32]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const i32, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromInt32Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: ::core::option::Option<&[i64]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const i64, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromInt64Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromPropVariantVectorElem(::core::mem::transmute(propvarin), ielem, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromResource<P0>(hinst: P0, id: u32) -> ::windows_core::Result<::windows_core::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HINSTANCE>,
{
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::super::super::Foundation:: HINSTANCE, id : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromResource(hinst.into_param().abi(), id, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<P0>(psz: P0) -> ::windows_core::Result<::windows_core::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : ::windows_core::PCWSTR, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromStringAsVector(psz.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: ::core::option::Option<&[::windows_core::PCWSTR]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const ::windows_core::PCWSTR, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromStringVector(::core::mem::transmute(prgsz.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgsz.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: ::core::option::Option<&[u16]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const u16, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromUInt16Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: ::core::option::Option<&[u32]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const u32, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromUInt32Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: ::core::option::Option<&[u64]>) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const u64, celems : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantFromUInt64Vector(::core::mem::transmute(prgn.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ppropvarvector : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitPropVariantVectorFromPropVariant(::core::mem::transmute(propvarsingle), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<P0>(pstg: P0) -> ::windows_core::Result<OLESTREAM>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : * mut::core::ffi::c_void, lpolestream : *mut OLESTREAM) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    OleConvertIStorageToOLESTREAM(pstg.into_param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<P0>(pstg: P0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *const super::STGMEDIUM) -> ::windows_core::Result<OLESTREAM>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : * mut::core::ffi::c_void, cfformat : u16, lwidth : i32, lheight : i32, dwsize : u32, pmedium : *const super:: STGMEDIUM, polestm : *mut OLESTREAM) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    OleConvertIStorageToOLESTREAMEx(pstg.into_param().abi(), cfformat, lwidth, lheight, dwsize, pmedium, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<P0>(lpolestream: *const OLESTREAM, pstg: P0, ptd: *const super::DVTARGETDEVICE) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : *const OLESTREAM, pstg : * mut::core::ffi::c_void, ptd : *const super:: DVTARGETDEVICE) -> ::windows_core::HRESULT);
    OleConvertOLESTREAMToIStorage(lpolestream, pstg.into_param().abi(), ptd).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<P0>(polestm: *const OLESTREAM, pstg: P0, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : *const OLESTREAM, pstg : * mut::core::ffi::c_void, pcfformat : *mut u16, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super:: STGMEDIUM) -> ::windows_core::HRESULT);
    OleConvertOLESTREAMToIStorageEx(polestm, pstg.into_param().abi(), pcfformat, plwwidth, plheight, pdwsize, pmedium).ok()
}
#[inline]
pub unsafe fn PropStgNameToFmtId<P0>(oszname: P0) -> ::windows_core::Result<::windows_core::GUID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : ::windows_core::PCWSTR, pfmtid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropStgNameToFmtId(oszname.into_param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut ::windows_core::PROPVARIANT, propvarsrc: *const ::windows_core::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: super::super::Variant::VARENUM) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, propvarsrc : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, flags : PROPVAR_CHANGE_FLAGS, vt : super::super::Variant:: VARENUM) -> ::windows_core::HRESULT);
    PropVariantChangeType(::core::mem::transmute(ppropvardest), ::core::mem::transmute(propvarsrc), flags, vt).ok()
}
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut ::windows_core::PROPVARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    PropVariantClear(::core::mem::transmute(pvar)).ok()
}
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const ::windows_core::PROPVARIANT, propvar2: *const ::windows_core::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, propvar2 : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
    PropVariantCompareEx(::core::mem::transmute(propvar1), ::core::mem::transmute(propvar2), unit, flags)
}
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut ::windows_core::PROPVARIANT, pvarsrc: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pvarsrc : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    PropVariantCopy(::core::mem::transmute(pvardest), ::core::mem::transmute(pvarsrc)).ok()
}
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pfval : *mut super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetBooleanElem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<f64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut f64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetDoubleElem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const ::windows_core::PROPVARIANT) -> u32 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> u32);
    PropVariantGetElementCount(::core::mem::transmute(propvar))
}
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<super::super::super::Foundation::FILETIME> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pftval : *mut super::super::super::Foundation:: FILETIME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetFileTimeElem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i16> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut i16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetInt16Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i32> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetInt32Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<i64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut i64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetInt64Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, ppszval : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetStringElem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u16> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut u16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetUInt16Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u32> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetUInt32Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const ::windows_core::PROPVARIANT, ielem: u32) -> ::windows_core::Result<u64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ielem : u32, pnval : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantGetUInt64Elem(::core::mem::transmute(propvar), ielem, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<::windows_core::BSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pbstrout : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToBSTR(::core::mem::transmute(propvar), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<super::super::super::Foundation::BOOL> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pfret : *mut super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToBoolean(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const ::windows_core::PROPVARIANT, prgf: &mut [super::super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgf : *mut super::super::super::Foundation:: BOOL, crgf : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToBooleanVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgf.as_ptr()), prgf.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgf : *mut *mut super::super::super::Foundation:: BOOL, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToBooleanVectorAlloc(::core::mem::transmute(propvar), pprgf, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault<P0>(propvarin: *const ::windows_core::PROPVARIANT, fdefault: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, fdefault : super::super::super::Foundation:: BOOL) -> super::super::super::Foundation:: BOOL);
    PropVariantToBooleanWithDefault(::core::mem::transmute(propvarin), fdefault.into_param().abi())
}
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const ::windows_core::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pv : *mut ::core::ffi::c_void, cb : u32) -> ::windows_core::HRESULT);
    PropVariantToBuffer(::core::mem::transmute(propvar), pv, cb).ok()
}
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<f64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pdblret : *mut f64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToDouble(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToDoubleVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut f64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToDoubleVectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const ::windows_core::PROPVARIANT, dbldefault: f64) -> f64 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, dbldefault : f64) -> f64);
    PropVariantToDoubleWithDefault(::core::mem::transmute(propvarin), dbldefault)
}
#[cfg(feature = "Win32_System_Variant")]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const ::windows_core::PROPVARIANT, pstfout: super::super::Variant::PSTIME_FLAGS) -> ::windows_core::Result<super::super::super::Foundation::FILETIME> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pstfout : super::super::Variant:: PSTIME_FLAGS, pftout : *mut super::super::super::Foundation:: FILETIME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToFileTime(::core::mem::transmute(propvar), pstfout, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const ::windows_core::PROPVARIANT, prgft: &mut [super::super::super::Foundation::FILETIME], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgft : *mut super::super::super::Foundation:: FILETIME, crgft : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToFileTimeVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgft.as_ptr()), prgft.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgft : *mut *mut super::super::super::Foundation:: FILETIME, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToFileTimeVectorAlloc(::core::mem::transmute(propvar), pprgft, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<::windows_core::GUID> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pguid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToGUID(::core::mem::transmute(propvar), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<i16> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, piret : *mut i16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToInt16(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut i16, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt16VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const ::windows_core::PROPVARIANT, idefault: i16) -> i16 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, idefault : i16) -> i16);
    PropVariantToInt16WithDefault(::core::mem::transmute(propvarin), idefault)
}
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<i32> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, plret : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToInt32(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut i32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt32VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const ::windows_core::PROPVARIANT, ldefault: i32) -> i32 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ldefault : i32) -> i32);
    PropVariantToInt32WithDefault(::core::mem::transmute(propvarin), ldefault)
}
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<i64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pllret : *mut i64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToInt64(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut i64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToInt64VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const ::windows_core::PROPVARIANT, lldefault: i64) -> i64 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, lldefault : i64) -> i64);
    PropVariantToInt64WithDefault(::core::mem::transmute(propvarin), lldefault)
}
#[inline]
pub unsafe fn PropVariantToString(propvar: *const ::windows_core::PROPVARIANT, psz: &mut [u16]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, psz : ::windows_core::PWSTR, cch : u32) -> ::windows_core::HRESULT);
    PropVariantToString(::core::mem::transmute(propvar), ::core::mem::transmute(psz.as_ptr()), psz.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ppszout : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToStringAlloc(::core::mem::transmute(propvar), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const ::windows_core::PROPVARIANT, prgsz: &mut [::windows_core::PWSTR], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgsz : *mut ::windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToStringVector(::core::mem::transmute(propvar), ::core::mem::transmute(prgsz.as_ptr()), prgsz.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgsz : *mut *mut ::windows_core::PWSTR, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToStringVectorAlloc(::core::mem::transmute(propvar), pprgsz, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToStringWithDefault<P0>(propvarin: *const ::windows_core::PROPVARIANT, pszdefault: P0) -> ::windows_core::PCWSTR
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pszdefault : ::windows_core::PCWSTR) -> ::windows_core::PCWSTR);
    PropVariantToStringWithDefault(::core::mem::transmute(propvarin), pszdefault.into_param().abi())
}
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<u16> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, puiret : *mut u16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToUInt16(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt16Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut u16, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt16VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const ::windows_core::PROPVARIANT, uidefault: u16) -> u16 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, uidefault : u16) -> u16);
    PropVariantToUInt16WithDefault(::core::mem::transmute(propvarin), uidefault)
}
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<u32> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pulret : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToUInt32(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt32Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt32VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const ::windows_core::PROPVARIANT, uldefault: u32) -> u32 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, uldefault : u32) -> u32);
    PropVariantToUInt32WithDefault(::core::mem::transmute(propvarin), uldefault)
}
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<u64> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pullret : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToUInt64(::core::mem::transmute(propvarin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const ::windows_core::PROPVARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt64Vector(::core::mem::transmute(propvar), ::core::mem::transmute(prgn.as_ptr()), prgn.len().try_into().unwrap(), pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const ::windows_core::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pprgn : *mut *mut u64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToUInt64VectorAlloc(::core::mem::transmute(propvar), pprgn, pcelem).ok()
}
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const ::windows_core::PROPVARIANT, ulldefault: u64) -> u64 {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ulldefault : u64) -> u64);
    PropVariantToUInt64WithDefault(::core::mem::transmute(propvarin), ulldefault)
}
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<::windows_core::VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pvar : *mut ::std::mem::MaybeUninit <::windows_core::VARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PropVariantToVariant(::core::mem::transmute(ppropvar), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T>(propvar: *const ::windows_core::PROPVARIANT) -> ::windows_core::Result<T>
where
    T: ::windows_core::Interface,
{
    ::windows_targets::link!("propsys.dll" "system" fn PropVariantToWinRTPropertyValue(propvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    PropVariantToWinRTPropertyValue(::core::mem::transmute(propvar), &T::IID, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn ReadClassStg<P0>(pstg: P0) -> ::windows_core::Result<::windows_core::GUID>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn ReadClassStg(pstg : * mut::core::ffi::c_void, pclsid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    ReadClassStg(pstg.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ReadClassStm<P0>(pstm: P0) -> ::windows_core::Result<::windows_core::GUID>
where
    P0: ::windows_core::IntoParam<super::IStream>,
{
    ::windows_targets::link!("ole32.dll" "system" fn ReadClassStm(pstm : * mut::core::ffi::c_void, pclsid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    ReadClassStm(pstm.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ReadFmtUserTypeStg<P0>(pstg: P0, pcf: *mut u16, lplpszusertype: ::core::option::Option<*mut ::windows_core::PWSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : * mut::core::ffi::c_void, pcf : *mut u16, lplpszusertype : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    ReadFmtUserTypeStg(pstg.into_param().abi(), pcf, ::core::mem::transmute(lplpszusertype.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetConvertStg<P0, P1>(pstg: P0, fconvert: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("ole32.dll" "system" fn SetConvertStg(pstg : * mut::core::ffi::c_void, fconvert : super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    SetConvertStg(pstg.into_param().abi(), fconvert.into_param().abi()).ok()
}
#[inline]
pub unsafe fn StgConvertPropertyToVariant<P0>(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut ::windows_core::PROPVARIANT, pma: P0) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<IMemoryAllocator>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgConvertPropertyToVariant(pprop : *const SERIALIZEDPROPERTYVALUE, codepage : u16, pvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, pma : * mut::core::ffi::c_void) -> super::super::super::Foundation:: BOOLEAN);
    StgConvertPropertyToVariant(pprop, codepage, ::core::mem::transmute(pvar), pma.into_param().abi())
}
#[inline]
pub unsafe fn StgConvertVariantToProperty<P0>(pvar: *const ::windows_core::PROPVARIANT, codepage: u16, pprop: ::core::option::Option<*mut SERIALIZEDPROPERTYVALUE>, pcb: *mut u32, pid: u32, freserved: P0, pcindirect: ::core::option::Option<*mut u32>) -> *mut SERIALIZEDPROPERTYVALUE
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgConvertVariantToProperty(pvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, codepage : u16, pprop : *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32, pid : u32, freserved : super::super::super::Foundation:: BOOLEAN, pcindirect : *mut u32) -> *mut SERIALIZEDPROPERTYVALUE);
    StgConvertVariantToProperty(::core::mem::transmute(pvar), codepage, ::core::mem::transmute(pprop.unwrap_or(::std::ptr::null_mut())), pcb, pid, freserved.into_param().abi(), ::core::mem::transmute(pcindirect.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn StgCreateDocfile<P0>(pwcsname: P0, grfmode: super::STGM, reserved: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : ::windows_core::PCWSTR, grfmode : super:: STGM, reserved : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgCreateDocfile(pwcsname.into_param().abi(), grfmode, reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<P0>(plkbyt: P0, grfmode: super::STGM, reserved: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<ILockBytes>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : * mut::core::ffi::c_void, grfmode : super:: STGM, reserved : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgCreateDocfileOnILockBytes(plkbyt.into_param().abi(), grfmode, reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgCreatePropSetStg<P0>(pstorage: P0, dwreserved: u32) -> ::windows_core::Result<IPropertySetStorage>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : * mut::core::ffi::c_void, dwreserved : u32, pppropsetstg : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgCreatePropSetStg(pstorage.into_param().abi(), dwreserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgCreatePropStg<P0>(punk: P0, fmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, dwreserved: u32) -> ::windows_core::Result<IPropertyStorage>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgCreatePropStg(punk : * mut::core::ffi::c_void, fmtid : *const ::windows_core::GUID, pclsid : *const ::windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgCreatePropStg(punk.into_param().abi(), fmtid, pclsid, grfflags, dwreserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgCreateStorageEx<P0, P1>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: ::core::option::Option<*mut STGOPTIONS>, psecuritydescriptor: P1, riid: *const ::windows_core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : ::windows_core::PCWSTR, grfmode : super:: STGM, stgfmt : STGFMT, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR, riid : *const ::windows_core::GUID, ppobjectopen : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    StgCreateStorageEx(pwcsname.into_param().abi(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions.unwrap_or(::std::ptr::null_mut())), psecuritydescriptor.into_param().abi(), riid, ppobjectopen).ok()
}
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const SERIALIZEDPROPERTYVALUE, cbmax : u32, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgDeserializePropVariant(pprop, cbmax, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<P0>(pwcsname: P0) -> ::windows_core::Result<IFillLockBytes>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : ::windows_core::PCWSTR, ppflb : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgGetIFillLockBytesOnFile(pwcsname.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<P0>(pilb: P0) -> ::windows_core::Result<IFillLockBytes>
where
    P0: ::windows_core::IntoParam<ILockBytes>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : * mut::core::ffi::c_void, ppflb : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgGetIFillLockBytesOnILockBytes(pilb.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgIsStorageFile<P0>(pwcsname: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    StgIsStorageFile(pwcsname.into_param().abi()).ok()
}
#[inline]
pub unsafe fn StgIsStorageILockBytes<P0>(plkbyt: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<ILockBytes>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    StgIsStorageILockBytes(plkbyt.into_param().abi()).ok()
}
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<P0>(pflb: P0, grfmode: u32, asyncflags: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<IFillLockBytes>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : * mut::core::ffi::c_void, grfmode : u32, asyncflags : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgOpenAsyncDocfileOnIFillLockBytes(pflb.into_param().abi(), grfmode, asyncflags, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgOpenLayoutDocfile<P0>(pwcsdfname: P0, grfmode: u32, reserved: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : ::windows_core::PCWSTR, grfmode : u32, reserved : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgOpenLayoutDocfile(pwcsdfname.into_param().abi(), grfmode, reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgOpenPropStg<P0>(punk: P0, fmtid: *const ::windows_core::GUID, grfflags: u32, dwreserved: u32) -> ::windows_core::Result<IPropertyStorage>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgOpenPropStg(punk : * mut::core::ffi::c_void, fmtid : *const ::windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgOpenPropStg(punk.into_param().abi(), fmtid, grfflags, dwreserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgOpenStorage<P0, P1>(pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: ::core::option::Option<*const *const u16>, reserved: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : ::windows_core::PCWSTR, pstgpriority : * mut::core::ffi::c_void, grfmode : super:: STGM, snbexclude : *const *const u16, reserved : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgOpenStorage(pwcsname.into_param().abi(), pstgpriority.into_param().abi(), grfmode, ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgOpenStorageEx<P0, P1>(pwcsname: P0, grfmode: super::STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: ::core::option::Option<*mut STGOPTIONS>, psecuritydescriptor: P1, riid: *const ::windows_core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : ::windows_core::PCWSTR, grfmode : super:: STGM, stgfmt : STGFMT, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::super::super::Security:: PSECURITY_DESCRIPTOR, riid : *const ::windows_core::GUID, ppobjectopen : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    StgOpenStorageEx(pwcsname.into_param().abi(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions.unwrap_or(::std::ptr::null_mut())), psecuritydescriptor.into_param().abi(), riid, ppobjectopen).ok()
}
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: ::core::option::Option<*const *const u16>, reserved: u32) -> ::windows_core::Result<IStorage>
where
    P0: ::windows_core::IntoParam<ILockBytes>,
    P1: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : * mut::core::ffi::c_void, pstgpriority : * mut::core::ffi::c_void, grfmode : super:: STGM, snbexclude : *const *const u16, reserved : u32, ppstgopen : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    StgOpenStorageOnILockBytes(plkbyt.into_param().abi(), pstgpriority.into_param().abi(), grfmode, ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32 {
    ::windows_targets::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const SERIALIZEDPROPERTYVALUE, cbprop : u32, codepage : u16, breserved : u8) -> u32);
    StgPropertyLengthAsVariant(pprop, cbprop, codepage, breserved)
}
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const ::windows_core::PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >, ppprop : *mut *mut SERIALIZEDPROPERTYVALUE, pcb : *mut u32) -> ::windows_core::HRESULT);
    StgSerializePropVariant(::core::mem::transmute(ppropvar), ppprop, pcb).ok()
}
#[inline]
pub unsafe fn StgSetTimes<P0>(lpszname: P0, pctime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>, patime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>, pmtime: ::core::option::Option<*const super::super::super::Foundation::FILETIME>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn StgSetTimes(lpszname : ::windows_core::PCWSTR, pctime : *const super::super::super::Foundation:: FILETIME, patime : *const super::super::super::Foundation:: FILETIME, pmtime : *const super::super::super::Foundation:: FILETIME) -> ::windows_core::HRESULT);
    StgSetTimes(lpszname.into_param().abi(), ::core::mem::transmute(pctime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(patime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pmtime.unwrap_or(::std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const ::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::PROPVARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const ::std::mem::MaybeUninit <::windows_core::VARIANT >, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToPropVariant(::core::mem::transmute(pvar), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<P0>(punkpropertyvalue: P0) -> ::windows_core::Result<::windows_core::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("propsys.dll" "system" fn WinRTPropertyValueToPropVariant(punkpropertyvalue : * mut::core::ffi::c_void, ppropvar : *mut ::std::mem::MaybeUninit <::windows_core::PROPVARIANT >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    WinRTPropertyValueToPropVariant(punkpropertyvalue.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn WriteClassStg<P0>(pstg: P0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
{
    ::windows_targets::link!("ole32.dll" "system" fn WriteClassStg(pstg : * mut::core::ffi::c_void, rclsid : *const ::windows_core::GUID) -> ::windows_core::HRESULT);
    WriteClassStg(pstg.into_param().abi(), rclsid).ok()
}
#[inline]
pub unsafe fn WriteClassStm<P0>(pstm: P0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::IStream>,
{
    ::windows_targets::link!("ole32.dll" "system" fn WriteClassStm(pstm : * mut::core::ffi::c_void, rclsid : *const ::windows_core::GUID) -> ::windows_core::HRESULT);
    WriteClassStm(pstm.into_param().abi(), rclsid).ok()
}
#[inline]
pub unsafe fn WriteFmtUserTypeStg<P0, P1>(pstg: P0, cf: u16, lpszusertype: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IStorage>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : * mut::core::ffi::c_void, cf : u16, lpszusertype : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    WriteFmtUserTypeStg(pstg.into_param().abi(), cf, lpszusertype.into_param().abi()).ok()
}
::windows_core::imp::com_interface!(IDirectWriterLock, IDirectWriterLock_Vtbl, 0x0e6d4d92_6738_11cf_9608_00aa00680db4);
::windows_core::imp::interface_hierarchy!(IDirectWriterLock, ::windows_core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WaitForWriteAccess)(::windows_core::Interface::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseWriteAccess)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HaveWriteAccess(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HaveWriteAccess)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectWriterLock_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub WaitForWriteAccess: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSTATPROPSETSTG, IEnumSTATPROPSETSTG_Vtbl, 0x0000013b_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IEnumSTATPROPSETSTG, ::windows_core::IUnknown);
impl IEnumSTATPROPSETSTG {
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSETSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut STATPROPSETSTG, *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSTATPROPSTG, IEnumSTATPROPSTG_Vtbl, 0x00000139_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IEnumSTATPROPSTG, ::windows_core::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub Next: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut STATPROPSTG, *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEnumSTATSTG, IEnumSTATSTG_Vtbl, 0x0000000d_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IEnumSTATSTG, ::windows_core::IUnknown);
impl IEnumSTATSTG {
    pub unsafe fn Next(&self, rgelt: &mut [super::STATSTG], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumSTATSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSTATSTG_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::STATSTG, *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFillLockBytes, IFillLockBytes_Vtbl, 0x99caf010_415e_11cf_8814_00aa00b569f5);
::windows_core::imp::interface_hierarchy!(IFillLockBytes, ::windows_core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FillAppend)(::windows_core::Interface::as_raw(self), pv, cb, &mut result__).map(|| result__)
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FillAt)(::windows_core::Interface::as_raw(self), uloffset, pv, cb, &mut result__).map(|| result__)
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFillSize)(::windows_core::Interface::as_raw(self), ulsize).ok()
    }
    pub unsafe fn Terminate<P0>(&self, bcanceled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self), bcanceled.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFillLockBytes_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FillAppend: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, u32, *mut u32) -> ::windows_core::HRESULT,
    pub FillAt: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64, *const ::core::ffi::c_void, u32, *mut u32) -> ::windows_core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ILayoutStorage, ILayoutStorage_Vtbl, 0x0e6d4d90_6738_11cf_9608_00aa00680db4);
::windows_core::imp::interface_hierarchy!(ILayoutStorage, ::windows_core::IUnknown);
impl ILayoutStorage {
    pub unsafe fn LayoutScript(&self, pstoragelayout: &[super::StorageLayout], glfinterleavedflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LayoutScript)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pstoragelayout.as_ptr()), pstoragelayout.len().try_into().unwrap(), glfinterleavedflag).ok()
    }
    pub unsafe fn BeginMonitor(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginMonitor)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndMonitor(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndMonitor)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReLayoutDocfile<P0>(&self, pwcsnewdfname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ReLayoutDocfile)(::windows_core::Interface::as_raw(self), pwcsnewdfname.into_param().abi()).ok()
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<P0>(&self, pilockbytes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ILockBytes>,
    {
        (::windows_core::Interface::vtable(self).ReLayoutDocfileOnILockBytes)(::windows_core::Interface::as_raw(self), pilockbytes.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct ILayoutStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LayoutScript: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const super::StorageLayout, u32, u32) -> ::windows_core::HRESULT,
    pub BeginMonitor: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReLayoutDocfile: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ILockBytes, ILockBytes_Vtbl, 0x0000000a_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(ILockBytes, ::windows_core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadAt)(::windows_core::Interface::as_raw(self), uloffset, pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteAt)(::windows_core::Interface::as_raw(self), uloffset, pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Flush)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSize)(::windows_core::Interface::as_raw(self), cb).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockRegion)(::windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stat)(::windows_core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
}
#[repr(C)]
pub struct ILockBytes_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReadAt: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64, *mut ::core::ffi::c_void, u32, *mut u32) -> ::windows_core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64, *const ::core::ffi::c_void, u32, *mut u32) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64) -> ::windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64, u64, u32) -> ::windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(*mut ::core::ffi::c_void, u64, u64, u32) -> ::windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::STATSTG, u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::interface!(IMemoryAllocator, IMemoryAllocator_Vtbl);
impl IMemoryAllocator {
    pub unsafe fn Allocate(&self, cbsize: u32) -> *mut ::core::ffi::c_void {
        (::windows_core::Interface::vtable(self).Allocate)(::windows_core::Interface::as_raw(self), cbsize)
    }
    pub unsafe fn Free(&self, pv: *mut ::core::ffi::c_void) {
        (::windows_core::Interface::vtable(self).Free)(::windows_core::Interface::as_raw(self), pv)
    }
}
#[repr(C)]
pub struct IMemoryAllocator_Vtbl {
    pub Allocate: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> *mut ::core::ffi::c_void,
    pub Free: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void),
}
::windows_core::imp::com_interface!(IPersistStorage, IPersistStorage_Vtbl, 0x0000010a_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IPersistStorage, ::windows_core::IUnknown, super::IPersist);
impl IPersistStorage {
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn InitNew<P0>(&self, pstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStorage>,
    {
        (::windows_core::Interface::vtable(self).InitNew)(::windows_core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
    pub unsafe fn Load<P0>(&self, pstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStorage>,
    {
        (::windows_core::Interface::vtable(self).Load)(::windows_core::Interface::as_raw(self), pstg.into_param().abi()).ok()
    }
    pub unsafe fn Save<P0, P1>(&self, pstgsave: P0, fsameasload: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStorage>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pstgsave.into_param().abi(), fsameasload.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<P0>(&self, pstgnew: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStorage>,
    {
        (::windows_core::Interface::vtable(self).SaveCompleted)(::windows_core::Interface::as_raw(self), pstgnew.into_param().abi()).ok()
    }
    pub unsafe fn HandsOffStorage(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandsOffStorage)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPersistStorage_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IPropertyBag, IPropertyBag_Vtbl, 0x55272a00_42cb_11ce_8135_00aa004bb851);
::windows_core::imp::interface_hierarchy!(IPropertyBag, ::windows_core::IUnknown);
impl IPropertyBag {
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut ::windows_core::VARIANT, perrorlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const ::windows_core::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
#[repr(C)]
pub struct IPropertyBag_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IPropertyBag2, IPropertyBag2_Vtbl, 0x22f55882_280b_11d0_a8a9_00a0c90c2004);
::windows_core::imp::interface_hierarchy!(IPropertyBag2, ::windows_core::IUnknown);
impl IPropertyBag2 {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn Read<P0>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: P0, pvarvalue: *mut ::windows_core::VARIANT, phrerror: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), cproperties, ppropbag, perrlog.into_param().abi(), ::core::mem::transmute(pvarvalue), phrerror).ok()
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const ::windows_core::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), cproperties, ppropbag, ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn CountProperties(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CountProperties)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, ppropbag: &mut [PROPBAG2], pcproperties: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyInfo)(::windows_core::Interface::as_raw(self), iproperty, ppropbag.len().try_into().unwrap(), ::core::mem::transmute(ppropbag.as_ptr()), pcproperties).ok()
    }
    pub unsafe fn LoadObject<P0, P1, P2>(&self, pstrname: P0, dwhint: u32, punkobject: P1, perrlog: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<super::IErrorLog>,
    {
        (::windows_core::Interface::vtable(self).LoadObject)(::windows_core::Interface::as_raw(self), pstrname.into_param().abi(), dwhint, punkobject.into_param().abi(), perrlog.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPropertyBag2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub Read: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const PROPBAG2, *mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    Read: usize,
    #[cfg(feature = "Win32_System_Variant")]
    pub Write: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const PROPBAG2, *const ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Variant")]
    pub GetPropertyInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, u32, *mut PROPBAG2, *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    GetPropertyInfo: usize,
    pub LoadObject: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, u32, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IPropertySetStorage, IPropertySetStorage_Vtbl, 0x0000013a_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IPropertySetStorage, ::windows_core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, grfmode: u32) -> ::windows_core::Result<IPropertyStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), rfmtid, pclsid, grfflags, grfmode, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Open(&self, rfmtid: *const ::windows_core::GUID, grfmode: u32) -> ::windows_core::Result<IPropertyStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), rfmtid, grfmode, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete(&self, rfmtid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), rfmtid).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows_core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enum)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, *const ::windows_core::GUID, u32, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IPropertyStorage, IPropertyStorage_Vtbl, 0x00000138_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IPropertyStorage, ::windows_core::IUnknown);
impl IPropertyStorage {
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut ::windows_core::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadMultiple)(::windows_core::Interface::as_raw(self), cpspec, rgpspec, ::core::mem::transmute(rgpropvar)).ok()
    }
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const ::windows_core::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteMultiple)(::windows_core::Interface::as_raw(self), cpspec, rgpspec, ::core::mem::transmute(rgpropvar), propidnamefirst).ok()
    }
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[PROPSPEC]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMultiple)(::windows_core::Interface::as_raw(self), rgpspec.len().try_into().unwrap(), ::core::mem::transmute(rgpspec.as_ptr())).ok()
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadPropertyNames)(::windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok()
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WritePropertyNames)(::windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok()
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyNames)(::windows_core::Interface::as_raw(self), rgpropid.len().try_into().unwrap(), ::core::mem::transmute(rgpropid.as_ptr())).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows_core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enum)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTimes)(::windows_core::Interface::as_raw(self), pctime, patime, pmtime).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClass)(::windows_core::Interface::as_raw(self), clsid).ok()
    }
    pub unsafe fn Stat(&self, pstatpsstg: *mut STATPROPSETSTG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stat)(::windows_core::Interface::as_raw(self), pstatpsstg).ok()
    }
}
#[repr(C)]
pub struct IPropertyStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReadMultiple: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const PROPSPEC, *mut ::std::mem::MaybeUninit<::windows_core::PROPVARIANT>) -> ::windows_core::HRESULT,
    pub WriteMultiple: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const PROPSPEC, *const ::std::mem::MaybeUninit<::windows_core::PROPVARIANT>, u32) -> ::windows_core::HRESULT,
    pub DeleteMultiple: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const PROPSPEC) -> ::windows_core::HRESULT,
    pub ReadPropertyNames: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u32, *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u32, *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u32) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTimes: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut STATPROPSETSTG) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IRootStorage, IRootStorage_Vtbl, 0x00000012_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IRootStorage, ::windows_core::IUnknown);
impl IRootStorage {
    pub unsafe fn SwitchToFile<P0>(&self, pszfile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SwitchToFile)(::windows_core::Interface::as_raw(self), pszfile.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRootStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SwitchToFile: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IStorage, IStorage_Vtbl, 0x0000000b_0000_0000_c000_000000000046);
::windows_core::imp::interface_hierarchy!(IStorage, ::windows_core::IUnknown);
impl IStorage {
    pub unsafe fn CreateStream<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows_core::Result<super::IStream>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStream)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), grfmode, reserved1, reserved2, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenStream<P0>(&self, pwcsname: P0, reserved1: ::core::option::Option<*const ::core::ffi::c_void>, grfmode: super::STGM, reserved2: u32) -> ::windows_core::Result<super::IStream>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenStream)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), ::core::mem::transmute(reserved1.unwrap_or(::std::ptr::null())), grfmode, reserved2, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateStorage<P0>(&self, pwcsname: P0, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows_core::Result<IStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStorage)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), grfmode, reserved1, reserved2, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenStorage<P0, P1>(&self, pwcsname: P0, pstgpriority: P1, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows_core::Result<IStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IStorage>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenStorage)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), pstgpriority.into_param().abi(), grfmode, snbexclude, reserved, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyTo<P0>(&self, rgiidexclude: ::core::option::Option<&[::windows_core::GUID]>, snbexclude: ::core::option::Option<*const *const u16>, pstgdest: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IStorage>,
    {
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), rgiidexclude.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(rgiidexclude.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(snbexclude.unwrap_or(::std::ptr::null())), pstgdest.into_param().abi()).ok()
    }
    pub unsafe fn MoveElementTo<P0, P1, P2>(&self, pwcsname: P0, pstgdest: P1, pwcsnewname: P2, grfflags: STGMOVE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IStorage>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).MoveElementTo)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), pstgdest.into_param().abi(), pwcsnewname.into_param().abi(), grfflags.0 as _).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revert)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumElements(&self, reserved1: u32, reserved2: ::core::option::Option<*const ::core::ffi::c_void>, reserved3: u32) -> ::windows_core::Result<IEnumSTATSTG> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumElements)(::windows_core::Interface::as_raw(self), reserved1, ::core::mem::transmute(reserved2.unwrap_or(::std::ptr::null())), reserved3, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DestroyElement<P0>(&self, pwcsname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DestroyElement)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi()).ok()
    }
    pub unsafe fn RenameElement<P0, P1>(&self, pwcsoldname: P0, pwcsnewname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RenameElement)(::windows_core::Interface::as_raw(self), pwcsoldname.into_param().abi(), pwcsnewname.into_param().abi()).ok()
    }
    pub unsafe fn SetElementTimes<P0>(&self, pwcsname: P0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetElementTimes)(::windows_core::Interface::as_raw(self), pwcsname.into_param().abi(), pctime, patime, pmtime).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClass)(::windows_core::Interface::as_raw(self), clsid).ok()
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStateBits)(::windows_core::Interface::as_raw(self), grfstatebits, grfmask).ok()
    }
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stat)(::windows_core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
}
#[repr(C)]
pub struct IStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, super::STGM, u32, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *const ::core::ffi::c_void, super::STGM, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, super::STGM, u32, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *mut ::core::ffi::c_void, super::STGM, *const *const u16, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const ::windows_core::GUID, *const *const u16, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveElementTo: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *mut ::core::ffi::c_void, ::windows_core::PCWSTR, u32) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DestroyElement: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetElementTimes: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME, *const super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, u32) -> ::windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::STATSTG, u32) -> ::windows_core::HRESULT,
}
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
pub const CWCSTORAGENAME: u32 = 32u32;
pub const PIDDI_THUMBNAIL: i32 = 2i32;
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
pub const PIDDSI_CATEGORY: u32 = 2u32;
pub const PIDDSI_COMPANY: u32 = 15u32;
pub const PIDDSI_DOCPARTS: u32 = 13u32;
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
pub const PIDDSI_LINECOUNT: u32 = 5u32;
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
pub const PIDDSI_MANAGER: u32 = 14u32;
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
pub const PIDDSI_PARCOUNT: u32 = 6u32;
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
pub const PIDDSI_SCALE: u32 = 11u32;
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
pub const PIDMSI_EDITOR: i32 = 2i32;
pub const PIDMSI_OWNER: i32 = 8i32;
pub const PIDMSI_PRODUCTION: i32 = 10i32;
pub const PIDMSI_PROJECT: i32 = 6i32;
pub const PIDMSI_RATING: i32 = 9i32;
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
pub const PIDMSI_SOURCE: i32 = 4i32;
pub const PIDMSI_STATUS: i32 = 7i32;
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(3i32);
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(5i32);
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(8i32);
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(4i32);
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(1i32);
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(0i32);
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(32767i32);
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(2i32);
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(7i32);
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(6i32);
pub const PIDMSI_SUPPLIER: i32 = 3i32;
pub const PIDSI_APPNAME: i32 = 18i32;
pub const PIDSI_AUTHOR: i32 = 4i32;
pub const PIDSI_CHARCOUNT: i32 = 16i32;
pub const PIDSI_COMMENTS: i32 = 6i32;
pub const PIDSI_CREATE_DTM: i32 = 12i32;
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
pub const PIDSI_EDITTIME: i32 = 10i32;
pub const PIDSI_KEYWORDS: i32 = 5i32;
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
pub const PIDSI_LASTPRINTED: i32 = 11i32;
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
pub const PIDSI_PAGECOUNT: i32 = 14i32;
pub const PIDSI_REVNUMBER: i32 = 9i32;
pub const PIDSI_SUBJECT: i32 = 3i32;
pub const PIDSI_TEMPLATE: i32 = 7i32;
pub const PIDSI_THUMBNAIL: i32 = 17i32;
pub const PIDSI_TITLE: i32 = 2i32;
pub const PIDSI_WORDCOUNT: i32 = 15i32;
pub const PID_BEHAVIOR: u32 = 2147483651u32;
pub const PID_CODEPAGE: u32 = 1u32;
pub const PID_DICTIONARY: u32 = 0u32;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
pub const PID_FIRST_USABLE: u32 = 2u32;
pub const PID_ILLEGAL: u32 = 4294967295u32;
pub const PID_LOCALE: u32 = 2147483648u32;
pub const PID_MAX_READONLY: u32 = 3221225471u32;
pub const PID_MIN_READONLY: u32 = 2147483648u32;
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
pub const PID_SECURITY: u32 = 2147483650u32;
pub const PROPSETFLAG_ANSI: u32 = 2u32;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
pub const PRSPEC_INVALID: u32 = 4294967295u32;
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = PROPSPEC_KIND(0u32);
pub const PRSPEC_PROPID: PROPSPEC_KIND = PROPSPEC_KIND(1u32);
pub const PVCF_DEFAULT: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(0i32);
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(32i32);
pub const PVCF_TREATEMPTYASGREATERTHAN: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(1i32);
pub const PVCF_USESTRCMP: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(2i32);
pub const PVCF_USESTRCMPC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(4i32);
pub const PVCF_USESTRCMPI: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(8i32);
pub const PVCF_USESTRCMPIC: PROPVAR_COMPARE_FLAGS = PROPVAR_COMPARE_FLAGS(16i32);
pub const PVCHF_ALPHABOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(2i32);
pub const PVCHF_DEFAULT: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(0i32);
pub const PVCHF_LOCALBOOL: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(8i32);
pub const PVCHF_NOHEXSTRING: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(16i32);
pub const PVCHF_NOUSEROVERRIDE: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(4i32);
pub const PVCHF_NOVALUEPROP: PROPVAR_CHANGE_FLAGS = PROPVAR_CHANGE_FLAGS(1i32);
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(4i32);
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(0i32);
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(3i32);
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(2i32);
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(5i32);
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(1i32);
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = PROPVAR_COMPARE_UNIT(6i32);
pub const STGFMT_ANY: STGFMT = STGFMT(4u32);
pub const STGFMT_DOCFILE: STGFMT = STGFMT(5u32);
pub const STGFMT_DOCUMENT: STGFMT = STGFMT(0u32);
pub const STGFMT_FILE: STGFMT = STGFMT(3u32);
pub const STGFMT_NATIVE: STGFMT = STGFMT(1u32);
pub const STGFMT_STORAGE: STGFMT = STGFMT(0u32);
pub const STGMOVE_COPY: STGMOVE = STGMOVE(1i32);
pub const STGMOVE_MOVE: STGMOVE = STGMOVE(0i32);
pub const STGMOVE_SHALLOWCOPY: STGMOVE = STGMOVE(2i32);
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct PIDMSI_STATUS_VALUE(pub i32);
impl ::windows_core::TypeKind for PIDMSI_STATUS_VALUE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PIDMSI_STATUS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDMSI_STATUS_VALUE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct PROPSPEC_KIND(pub u32);
impl ::windows_core::TypeKind for PROPSPEC_KIND {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPSPEC_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPSPEC_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
impl ::windows_core::TypeKind for PROPVAR_CHANGE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl PROPVAR_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
impl ::windows_core::TypeKind for PROPVAR_COMPARE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_FLAGS").field(&self.0).finish()
    }
}
impl PROPVAR_COMPARE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPVAR_COMPARE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPVAR_COMPARE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPVAR_COMPARE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct PROPVAR_COMPARE_UNIT(pub i32);
impl ::windows_core::TypeKind for PROPVAR_COMPARE_UNIT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPVAR_COMPARE_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPVAR_COMPARE_UNIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct STGFMT(pub u32);
impl ::windows_core::TypeKind for STGFMT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for STGFMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGFMT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct STGMOVE(pub i32);
impl ::windows_core::TypeKind for STGMOVE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for STGMOVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGMOVE").field(&self.0).finish()
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for BSTRBLOB {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::VARIANT_BOOL,
}
impl ::core::marker::Copy for CABOOL {}
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABOOL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows_core::TypeKind for CABOOL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CABOOL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CABOOL {}
impl ::core::default::Default for CABOOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_core::BSTR,
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
impl ::windows_core::TypeKind for CABSTR {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CABSTRBLOB {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CAC {
    pub cElems: u32,
    pub pElems: ::windows_core::PSTR,
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
impl ::windows_core::TypeKind for CAC {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CACLIPDATA {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows_core::GUID,
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
impl ::windows_core::TypeKind for CACLSID {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CACY {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CADATE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CADBL {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
impl ::core::marker::Copy for CAFILETIME {}
impl ::core::clone::Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAFILETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAFILETIME").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows_core::TypeKind for CAFILETIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CAFILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAFILETIME {}
impl ::core::default::Default for CAFILETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for CAFLT {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAH {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAI {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAL {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_core::PSTR,
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
impl ::windows_core::TypeKind for CALPSTR {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows_core::PWSTR,
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
impl ::windows_core::TypeKind for CALPWSTR {
    type TypeKind = ::windows_core::CopyType;
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
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut ::windows_core::PROPVARIANT,
}
impl ::core::marker::Copy for CAPROPVARIANT {}
impl ::core::clone::Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPROPVARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPROPVARIANT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::windows_core::TypeKind for CAPROPVARIANT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CAPROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAPROPVARIANT {}
impl ::core::default::Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for CASCODE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAUB {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAUH {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAUI {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CAUL {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for CLIPDATA {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for OLESTREAM {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for OLESTREAMVTBL {
    type TypeKind = ::windows_core::CopyType;
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
#[cfg(feature = "Win32_System_Variant")]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: super::super::Variant::VARENUM,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: ::windows_core::PWSTR,
    pub clsid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::marker::Copy for PROPBAG2 {}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::fmt::Debug for PROPBAG2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPBAG2").field("dwType", &self.dwType).field("vt", &self.vt).field("cfType", &self.cfType).field("dwHint", &self.dwHint).field("pstrName", &self.pstrName).field("clsid", &self.clsid).finish()
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::windows_core::TypeKind for PROPBAG2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::cmp::PartialEq for PROPBAG2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.vt == other.vt && self.cfType == other.cfType && self.dwHint == other.dwHint && self.pstrName == other.pstrName && self.clsid == other.clsid
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::cmp::Eq for PROPBAG2 {}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::default::Default for PROPBAG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for PROPSPEC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for PROPSPEC_0 {}
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PROPSPEC_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for RemSNB {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for SERIALIZEDPROPERTYVALUE {
    type TypeKind = ::windows_core::CopyType;
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
pub struct STATPROPSETSTG {
    pub fmtid: ::windows_core::GUID,
    pub clsid: ::windows_core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
impl ::core::marker::Copy for STATPROPSETSTG {}
impl ::core::clone::Clone for STATPROPSETSTG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STATPROPSETSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSETSTG").field("fmtid", &self.fmtid).field("clsid", &self.clsid).field("grfFlags", &self.grfFlags).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("dwOSVersion", &self.dwOSVersion).finish()
    }
}
impl ::windows_core::TypeKind for STATPROPSETSTG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.clsid == other.clsid && self.grfFlags == other.grfFlags && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.dwOSVersion == other.dwOSVersion
    }
}
impl ::core::cmp::Eq for STATPROPSETSTG {}
impl ::core::default::Default for STATPROPSETSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub struct STATPROPSTG {
    pub lpwstrName: ::windows_core::PWSTR,
    pub propid: u32,
    pub vt: super::super::Variant::VARENUM,
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::marker::Copy for STATPROPSTG {}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::fmt::Debug for STATPROPSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSTG").field("lpwstrName", &self.lpwstrName).field("propid", &self.propid).field("vt", &self.vt).finish()
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::windows_core::TypeKind for STATPROPSTG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::cmp::PartialEq for STATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        self.lpwstrName == other.lpwstrName && self.propid == other.propid && self.vt == other.vt
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::cmp::Eq for STATPROPSTG {}
#[cfg(feature = "Win32_System_Variant")]
impl ::core::default::Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: ::windows_core::PCWSTR,
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
impl ::windows_core::TypeKind for STGOPTIONS {
    type TypeKind = ::windows_core::CopyType;
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
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows_core::GUID,
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
impl ::windows_core::TypeKind for VERSIONEDSTREAM {
    type TypeKind = ::windows_core::CopyType;
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
