#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [super::PROPVARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut super::PROPVARIANT, cvars : super::UINT));
    unsafe { ClearPropVariantArray(rgpropvar.as_mut_ptr(), rgpropvar.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [super::VARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut super::VARIANT, cvars : super::UINT));
    unsafe { ClearVariantArray(pvars.as_mut_ptr(), pvars.len().try_into().unwrap()) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: Option<&[windows_core::BOOL]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const windows_core::BOOL, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromBooleanVector(prgf.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgf.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const core::ffi::c_void, cb: super::UINT, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const core::ffi::c_void, cb : super::UINT, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromBuffer(pv, cb, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const super::IID, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const super::IID, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromCLSID(clsid, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: Option<&[super::DOUBLE]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const super::DOUBLE, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromDoubleVector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::FILETIME, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::FILETIME, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromFileTime(pftin, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: Option<&[super::FILETIME]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::FILETIME, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromFileTimeVector(prgft.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgft.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const windows_core::GUID, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const windows_core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromGUIDAsString(guid, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: Option<&[super::SHORT]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const super::SHORT, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromInt16Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: Option<&[super::LONG]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const super::LONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromInt32Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: Option<&[super::LONGLONG]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const super::LONGLONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromInt64Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::PROPVARIANT, ielem: super::ULONG, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const super::PROPVARIANT, ielem : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromPropVariantVectorElem(propvarin, ielem, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromResource(hinst: super::HINSTANCE, id: super::UINT, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::HINSTANCE, id : super::UINT, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromResource(hinst, id, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::STRRET, pidl: Option<super::LPCITEMIDLIST>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStrRet(pstrret : *mut super::STRRET, pidl : super::LPCITEMIDLIST, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromStrRet(pstrret as _, pidl.unwrap_or(core::mem::zeroed()) as _, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<P0>(psz: P0, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : windows_core::PCWSTR, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromStringAsVector(psz.param().abi(), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: Option<&[windows_core::PCWSTR]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const windows_core::PCWSTR, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromStringVector(prgsz.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgsz.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: Option<&[super::USHORT]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const super::USHORT, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromUInt16Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: Option<&[super::ULONG]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const super::ULONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromUInt32Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: Option<&[super::ULONGLONG]>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const super::ULONGLONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromUInt64Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::PROPVARIANT, ppropvarvector: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const super::PROPVARIANT, ppropvarvector : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantVectorFromPropVariant(propvarsingle, ppropvarvector) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[windows_core::BOOL], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const windows_core::BOOL, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromBooleanArray(prgf.as_ptr(), prgf.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const core::ffi::c_void, cb: super::UINT, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const core::ffi::c_void, cb : super::UINT, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromBuffer(pv, cb, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[super::DOUBLE], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const super::DOUBLE, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromDoubleArray(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::FILETIME, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::FILETIME, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromFileTime(pft, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: Option<&[super::FILETIME]>, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::FILETIME, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromFileTimeArray(prgft.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgft.map_or(0, |slice| slice.len().try_into().unwrap()), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const windows_core::GUID, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const windows_core::GUID, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromGUIDAsString(guid, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[super::SHORT], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const super::SHORT, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromInt16Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[super::LONG], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const super::LONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromInt32Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[super::LONGLONG], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const super::LONGLONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromInt64Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromResource(hinst: super::HINSTANCE, id: super::UINT, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::HINSTANCE, id : super::UINT, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromResource(hinst, id, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::STRRET, pidl: super::LPCITEMIDLIST, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromStrRet(pstrret : *const super::STRRET, pidl : super::LPCITEMIDLIST, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromStrRet(pstrret, pidl, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[windows_core::PCWSTR], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const windows_core::PCWSTR, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromStringArray(prgsz.as_ptr(), prgsz.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[super::USHORT], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const super::USHORT, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromUInt16Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[super::ULONG], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const super::ULONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromUInt32Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[super::ULONGLONG], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const super::ULONGLONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromUInt64Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::VARIANT, ielem: super::ULONG, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const super::VARIANT, ielem : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { InitVariantFromVariantArrayElem(varin, ielem, pvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::PROPVARIANT, propvarsrc: *const super::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: super::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut super::PROPVARIANT, propvarsrc : *const super::PROPVARIANT, flags : PROPVAR_CHANGE_FLAGS, vt : super::VARTYPE) -> windows_core::HRESULT);
    unsafe { PropVariantChangeType(ppropvardest, propvarsrc, flags, vt) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::PROPVARIANT, propvar2: *const super::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const super::PROPVARIANT, propvar2 : *const super::PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
    unsafe { PropVariantCompareEx(propvar1, propvar2, unit, flags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pfval: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { PropVariantGetBooleanElem(propvar, ielem, pfval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { PropVariantGetDoubleElem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::PROPVARIANT) -> super::ULONG {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const super::PROPVARIANT) -> super::ULONG);
    unsafe { PropVariantGetElementCount(propvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pftval: *mut super::FILETIME) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pftval : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe { PropVariantGetFileTimeElem(propvar, ielem, pftval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { PropVariantGetInt16Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { PropVariantGetInt32Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::LONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::LONGLONG) -> windows_core::HRESULT);
    unsafe { PropVariantGetInt64Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, ppszval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { PropVariantGetStringElem(propvar, ielem, ppszval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { PropVariantGetUInt16Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantGetUInt32Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::PROPVARIANT, ielem: super::ULONG, pnval: *mut super::ULONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::ULONGLONG) -> windows_core::HRESULT);
    unsafe { PropVariantGetUInt64Elem(propvar, ielem, pnval as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::PROPVARIANT, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const super::PROPVARIANT, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PropVariantToBSTR(propvar, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::PROPVARIANT, pfret: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const super::PROPVARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { PropVariantToBoolean(propvarin, pfret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::PROPVARIANT, prgf: *mut windows_core::BOOL, crgf: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const super::PROPVARIANT, prgf : *mut windows_core::BOOL, crgf : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVector(propvar, prgf as _, crgf, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::PROPVARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const super::PROPVARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVectorAlloc(propvar, pprgf as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault(propvarin: *const super::PROPVARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const super::PROPVARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PropVariantToBooleanWithDefault(propvarin, fdefault.into()) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::PROPVARIANT, pv: *mut core::ffi::c_void, cb: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const super::PROPVARIANT, pv : *mut core::ffi::c_void, cb : super::UINT) -> windows_core::HRESULT);
    unsafe { PropVariantToBuffer(propvar, pv as _, cb) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::PROPVARIANT, pdblret: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const super::PROPVARIANT, pdblret : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { PropVariantToDouble(propvarin, pdblret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::PROPVARIANT, prgn: *mut super::DOUBLE, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const super::PROPVARIANT, prgn : *mut super::DOUBLE, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::DOUBLE, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::DOUBLE, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::PROPVARIANT, dbldefault: super::DOUBLE) -> super::DOUBLE {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const super::PROPVARIANT, dbldefault : super::DOUBLE) -> super::DOUBLE);
    unsafe { PropVariantToDoubleWithDefault(propvarin, dbldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::PROPVARIANT, pstfout: PSTIME_FLAGS, pftout: *mut super::FILETIME) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const super::PROPVARIANT, pstfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTime(propvar, pstfout, pftout as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::PROPVARIANT, prgft: *mut super::FILETIME, crgft: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const super::PROPVARIANT, prgft : *mut super::FILETIME, crgft : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVector(propvar, prgft as _, crgft, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::PROPVARIANT, pprgft: *mut *mut super::FILETIME, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const super::PROPVARIANT, pprgft : *mut *mut super::FILETIME, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVectorAlloc(propvar, pprgft as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::PROPVARIANT, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const super::PROPVARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { PropVariantToGUID(propvar, pguid as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::PROPVARIANT, piret: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const super::PROPVARIANT, piret : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16(propvarin, piret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::SHORT, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::SHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::SHORT, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::SHORT, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::PROPVARIANT, idefault: super::SHORT) -> super::SHORT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const super::PROPVARIANT, idefault : super::SHORT) -> super::SHORT);
    unsafe { PropVariantToInt16WithDefault(propvarin, idefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::PROPVARIANT, plret: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const super::PROPVARIANT, plret : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32(propvarin, plret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::LONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::LONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::LONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::LONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::PROPVARIANT, ldefault: super::LONG) -> super::LONG {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const super::PROPVARIANT, ldefault : super::LONG) -> super::LONG);
    unsafe { PropVariantToInt32WithDefault(propvarin, ldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::PROPVARIANT, pllret: *mut super::LONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const super::PROPVARIANT, pllret : *mut super::LONGLONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64(propvarin, pllret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::LONGLONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::LONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::LONGLONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::LONGLONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::PROPVARIANT, lldefault: super::LONGLONG) -> super::LONGLONG {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const super::PROPVARIANT, lldefault : super::LONGLONG) -> super::LONGLONG);
    unsafe { PropVariantToInt64WithDefault(propvarin, lldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::PROPVARIANT, pstrret: *mut super::STRRET) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStrRet(propvar : *const super::PROPVARIANT, pstrret : *mut super::STRRET) -> windows_core::HRESULT);
    unsafe { PropVariantToStrRet(propvar, pstrret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::PROPVARIANT, psz: windows_core::PWSTR, cch: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const super::PROPVARIANT, psz : windows_core::PWSTR, cch : super::UINT) -> windows_core::HRESULT);
    unsafe { PropVariantToString(propvar, psz, cch) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::PROPVARIANT, ppszout: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const super::PROPVARIANT, ppszout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { PropVariantToStringAlloc(propvar, ppszout as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::PROPVARIANT, prgsz: *mut windows_core::PWSTR, crgsz: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const super::PROPVARIANT, prgsz : *mut windows_core::PWSTR, crgsz : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVector(propvar, prgsz as _, crgsz, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::PROPVARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const super::PROPVARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVectorAlloc(propvar, pprgsz as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<P1>(propvarin: *const super::PROPVARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const super::PROPVARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { PropVariantToStringWithDefault(propvarin, pszdefault.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::PROPVARIANT, puiret: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const super::PROPVARIANT, puiret : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16(propvarin, puiret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::USHORT, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::USHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::USHORT, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::USHORT, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::PROPVARIANT, uidefault: super::USHORT) -> super::USHORT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const super::PROPVARIANT, uidefault : super::USHORT) -> super::USHORT);
    unsafe { PropVariantToUInt16WithDefault(propvarin, uidefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::PROPVARIANT, pulret: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const super::PROPVARIANT, pulret : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32(propvarin, pulret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::ULONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::ULONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::PROPVARIANT, uldefault: super::ULONG) -> super::ULONG {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const super::PROPVARIANT, uldefault : super::ULONG) -> super::ULONG);
    unsafe { PropVariantToUInt32WithDefault(propvarin, uldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::PROPVARIANT, pullret: *mut super::ULONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const super::PROPVARIANT, pullret : *mut super::ULONGLONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64(propvarin, pullret as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::PROPVARIANT, prgn: *mut super::ULONGLONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::ULONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64Vector(propvar, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut super::ULONGLONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::ULONGLONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::PROPVARIANT, ulldefault: super::ULONGLONG) -> super::ULONGLONG {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const super::PROPVARIANT, ulldefault : super::ULONGLONG) -> super::ULONGLONG);
    unsafe { PropVariantToUInt64WithDefault(propvarin, ulldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::PROPVARIANT, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const super::PROPVARIANT, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantToVariant(ppropvar, pvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const super::SERIALIZEDPROPERTYVALUE, cbmax: super::ULONG, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbmax : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { StgDeserializePropVariant(pprop, cbmax, ppropvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const super::PROPVARIANT, ppprop: *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const super::PROPVARIANT, ppprop : *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { StgSerializePropVariant(ppropvar, ppprop as _, pcb as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::VARIANT, var2: *const super::VARIANT) -> i32 {
    windows_core::link!("propsys.dll" "system" fn VariantCompare(var1 : *const super::VARIANT, var2 : *const super::VARIANT) -> i32);
    unsafe { VariantCompare(var1, var2) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::VARIANT, ielem: super::ULONG, pfval: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const super::VARIANT, ielem : super::ULONG, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { VariantGetBooleanElem(var, ielem, pfval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VariantGetDoubleElem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::VARIANT) -> super::ULONG {
    windows_core::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const super::VARIANT) -> super::ULONG);
    unsafe { VariantGetElementCount(varin) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VariantGetInt16Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VariantGetInt32Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::LONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::LONGLONG) -> windows_core::HRESULT);
    unsafe { VariantGetInt64Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::VARIANT, ielem: super::ULONG, ppszval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const super::VARIANT, ielem : super::ULONG, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { VariantGetStringElem(var, ielem, ppszval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VariantGetUInt16Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantGetUInt32Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::VARIANT, ielem: super::ULONG, pnval: *mut super::ULONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::ULONGLONG) -> windows_core::HRESULT);
    unsafe { VariantGetUInt64Elem(var, ielem, pnval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::VARIANT, pfret: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const super::VARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { VariantToBoolean(varin, pfret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::VARIANT, prgf: *mut windows_core::BOOL, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const super::VARIANT, prgf : *mut windows_core::BOOL, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArray(var, prgf as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::VARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const super::VARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArrayAlloc(var, pprgf as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault(varin: *const super::VARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const super::VARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { VariantToBooleanWithDefault(varin, fdefault.into()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::VARIANT, pv: *mut core::ffi::c_void, cb: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const super::VARIANT, pv : *mut core::ffi::c_void, cb : super::UINT) -> windows_core::HRESULT);
    unsafe { VariantToBuffer(varin, pv as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::VARIANT, pwdate: *mut super::WORD, pwtime: *mut super::WORD) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const super::VARIANT, pwdate : *mut super::WORD, pwtime : *mut super::WORD) -> windows_core::HRESULT);
    unsafe { VariantToDosDateTime(varin, pwdate as _, pwtime as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::VARIANT, pdblret: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDouble(varin : *const super::VARIANT, pdblret : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VariantToDouble(varin, pdblret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::VARIANT, prgn: *mut super::DOUBLE, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const super::VARIANT, prgn : *mut super::DOUBLE, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArray(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::DOUBLE, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::DOUBLE, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::VARIANT, dbldefault: super::DOUBLE) -> super::DOUBLE {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const super::VARIANT, dbldefault : super::DOUBLE) -> super::DOUBLE);
    unsafe { VariantToDoubleWithDefault(varin, dbldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::VARIANT, stfout: PSTIME_FLAGS, pftout: *mut super::FILETIME) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const super::VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe { VariantToFileTime(varin, stfout, pftout as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::VARIANT, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToGUID(varin : *const super::VARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { VariantToGUID(varin, pguid as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::VARIANT, piret: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16(varin : *const super::VARIANT, piret : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VariantToInt16(varin, piret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::VARIANT, prgn: *mut super::SHORT, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const super::VARIANT, prgn : *mut super::SHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt16Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::SHORT, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::SHORT, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt16ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::VARIANT, idefault: super::SHORT) -> super::SHORT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const super::VARIANT, idefault : super::SHORT) -> super::SHORT);
    unsafe { VariantToInt16WithDefault(varin, idefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::VARIANT, plret: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32(varin : *const super::VARIANT, plret : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VariantToInt32(varin, plret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::VARIANT, prgn: *mut super::LONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const super::VARIANT, prgn : *mut super::LONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt32Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::LONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::LONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt32ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::VARIANT, ldefault: super::LONG) -> super::LONG {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const super::VARIANT, ldefault : super::LONG) -> super::LONG);
    unsafe { VariantToInt32WithDefault(varin, ldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::VARIANT, pllret: *mut super::LONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64(varin : *const super::VARIANT, pllret : *mut super::LONGLONG) -> windows_core::HRESULT);
    unsafe { VariantToInt64(varin, pllret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::VARIANT, prgn: *mut super::LONGLONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const super::VARIANT, prgn : *mut super::LONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt64Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::LONGLONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::LONGLONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToInt64ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::VARIANT, lldefault: super::LONGLONG) -> super::LONGLONG {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const super::VARIANT, lldefault : super::LONGLONG) -> super::LONGLONG);
    unsafe { VariantToInt64WithDefault(varin, lldefault) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::VARIANT, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const super::VARIANT, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { VariantToPropVariant(pvar, ppropvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::VARIANT, pstrret: *mut super::STRRET) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStrRet(varin : *const super::VARIANT, pstrret : *mut super::STRRET) -> windows_core::HRESULT);
    unsafe { VariantToStrRet(varin, pstrret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToString(varin: *const super::VARIANT, pszbuf: windows_core::PWSTR, cchbuf: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToString(varin : *const super::VARIANT, pszbuf : windows_core::PWSTR, cchbuf : super::UINT) -> windows_core::HRESULT);
    unsafe { VariantToString(varin, pszbuf, cchbuf) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::VARIANT, ppszbuf: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const super::VARIANT, ppszbuf : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { VariantToStringAlloc(varin, ppszbuf as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::VARIANT, prgsz: *mut windows_core::PWSTR, crgsz: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArray(var : *const super::VARIANT, prgsz : *mut windows_core::PWSTR, crgsz : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToStringArray(var, prgsz as _, crgsz, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::VARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const super::VARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToStringArrayAlloc(var, pprgsz as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<P1>(varin: *const super::VARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const super::VARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { VariantToStringWithDefault(varin, pszdefault.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::VARIANT, puiret: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const super::VARIANT, puiret : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VariantToUInt16(varin, puiret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::VARIANT, prgn: *mut super::USHORT, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const super::VARIANT, prgn : *mut super::USHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt16Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::USHORT, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::USHORT, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt16ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::VARIANT, uidefault: super::USHORT) -> super::USHORT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const super::VARIANT, uidefault : super::USHORT) -> super::USHORT);
    unsafe { VariantToUInt16WithDefault(varin, uidefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::VARIANT, pulret: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const super::VARIANT, pulret : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt32(varin, pulret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::VARIANT, prgn: *mut super::ULONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const super::VARIANT, prgn : *mut super::ULONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt32Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt32ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::VARIANT, uldefault: super::ULONG) -> super::ULONG {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const super::VARIANT, uldefault : super::ULONG) -> super::ULONG);
    unsafe { VariantToUInt32WithDefault(varin, uldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::VARIANT, pullret: *mut super::ULONGLONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const super::VARIANT, pullret : *mut super::ULONGLONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt64(varin, pullret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::VARIANT, prgn: *mut super::ULONGLONG, crgn: super::ULONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const super::VARIANT, prgn : *mut super::ULONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt64Array(var, prgn as _, crgn, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut super::ULONGLONG, pcelem: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::ULONGLONG, pcelem : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VariantToUInt64ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::VARIANT, ulldefault: super::ULONGLONG) -> super::ULONGLONG {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const super::VARIANT, ulldefault : super::ULONGLONG) -> super::ULONGLONG);
    unsafe { VariantToUInt64WithDefault(varin, ulldefault) }
}
pub const DPF_ERROR: DRAWPROGRESSFLAGS = 4;
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = 1;
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = 2;
pub const DPF_NONE: DRAWPROGRESSFLAGS = 0;
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = 16;
pub const DPF_WARNING: DRAWPROGRESSFLAGS = 8;
pub type DRAWPROGRESSFLAGS = i32;
pub type PROPVAR_CHANGE_FLAGS = i32;
pub type PROPVAR_COMPARE_FLAGS = i32;
pub type PROPVAR_COMPARE_UNIT = i32;
pub const PSTF_LOCAL: tagPSTIME_FLAGS = 1;
pub const PSTF_UTC: tagPSTIME_FLAGS = 0;
pub type PSTIME_FLAGS = i32;
pub const PVCF_DEFAULT: tagPROPVAR_COMPARE_FLAGS = 0;
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: tagPROPVAR_COMPARE_FLAGS = 32;
pub const PVCF_TREATEMPTYASGREATERTHAN: tagPROPVAR_COMPARE_FLAGS = 1;
pub const PVCF_USESTRCMP: tagPROPVAR_COMPARE_FLAGS = 2;
pub const PVCF_USESTRCMPC: tagPROPVAR_COMPARE_FLAGS = 4;
pub const PVCF_USESTRCMPI: tagPROPVAR_COMPARE_FLAGS = 8;
pub const PVCF_USESTRCMPIC: tagPROPVAR_COMPARE_FLAGS = 16;
pub const PVCHF_ALPHABOOL: tagPROPVAR_CHANGE_FLAGS = 2;
pub const PVCHF_DEFAULT: tagPROPVAR_CHANGE_FLAGS = 0;
pub const PVCHF_LOCALBOOL: tagPROPVAR_CHANGE_FLAGS = 8;
pub const PVCHF_NOHEXSTRING: tagPROPVAR_CHANGE_FLAGS = 16;
pub const PVCHF_NOUSEROVERRIDE: tagPROPVAR_CHANGE_FLAGS = 4;
pub const PVCHF_NOVALUEPROP: tagPROPVAR_CHANGE_FLAGS = 1;
pub const PVCU_DAY: PROPVAR_COMPARE_UNIT = 4;
pub const PVCU_DEFAULT: PROPVAR_COMPARE_UNIT = 0;
pub const PVCU_HOUR: PROPVAR_COMPARE_UNIT = 3;
pub const PVCU_MINUTE: PROPVAR_COMPARE_UNIT = 2;
pub const PVCU_MONTH: PROPVAR_COMPARE_UNIT = 5;
pub const PVCU_SECOND: PROPVAR_COMPARE_UNIT = 1;
pub const PVCU_YEAR: PROPVAR_COMPARE_UNIT = 6;
pub type tagPROPVAR_CHANGE_FLAGS = i32;
pub type tagPROPVAR_COMPARE_FLAGS = i32;
pub type tagPSTIME_FLAGS = i32;
