#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearPropVariantArray(rgpropvar: &mut [super::PROPVARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut super::PROPVARIANT, cvars : u32));
    unsafe { ClearPropVariantArray(rgpropvar.as_mut_ptr(), rgpropvar.len().try_into().unwrap()) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [super::VARIANT]) {
    windows_core::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut super::VARIANT, cvars : u32));
    unsafe { ClearVariantArray(pvars.as_mut_ptr(), pvars.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromBooleanVector(prgf: Option<&[windows_core::BOOL]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const windows_core::BOOL, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromBooleanVector(prgf.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgf.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromBuffer(pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromBuffer(pv, cb, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const windows_core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromCLSID(clsid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromDoubleVector(prgn: Option<&[f64]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const f64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromDoubleVector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTime(pftin: *const super::FILETIME) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::FILETIME, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromFileTime(pftin, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromFileTimeVector(prgft: Option<&[super::FILETIME]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::FILETIME, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromFileTimeVector(prgft.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgft.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromGUIDAsString(guid: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const windows_core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromGUIDAsString(guid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt16Vector(prgn: Option<&[i16]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const i16, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt16Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt32Vector(prgn: Option<&[i32]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const i32, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt32Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromInt64Vector(prgn: Option<&[i64]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const i64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromInt64Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const super::PROPVARIANT, ielem : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromPropVariantVectorElem(propvarin, ielem, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromResource(hinst: super::HINSTANCE, id: u32) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::HINSTANCE, id : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromResource(hinst, id, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStrRet(pstrret: *mut super::STRRET, pidl: Option<*const super::ITEMIDLIST>, ppropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStrRet(pstrret : *mut super::STRRET, pidl : *const super::ITEMIDLIST, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { InitPropVariantFromStrRet(pstrret as _, pidl.unwrap_or(core::mem::zeroed()) as _, ppropvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStringAsVector<P0>(psz: P0) -> windows_core::Result<super::PROPVARIANT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : windows_core::PCWSTR, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromStringAsVector(psz.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromStringVector(prgsz: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const windows_core::PCWSTR, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromStringVector(prgsz.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgsz.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt16Vector(prgn: Option<&[u16]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const u16, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt16Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt32Vector(prgn: Option<&[u32]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const u32, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt32Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantFromUInt64Vector(prgn: Option<&[u64]>) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const u64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantFromUInt64Vector(prgn.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgn.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::PROPVARIANT) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const super::PROPVARIANT, ppropvarvector : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitPropVariantVectorFromPropVariant(propvarsingle, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[windows_core::BOOL]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const windows_core::BOOL, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromBooleanArray(prgf.as_ptr(), prgf.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromBuffer(pv, cb, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[f64]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const f64, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromDoubleArray(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::FILETIME) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::FILETIME, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromFileTime(pft, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: Option<&[super::FILETIME]>) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::FILETIME, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromFileTimeArray(prgft.map_or(core::ptr::null(), |slice| slice.as_ptr()), prgft.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const windows_core::GUID) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const windows_core::GUID, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromGUIDAsString(guid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[i16]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const i16, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt16Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[i32]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const i32, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt32Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[i64]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const i64, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromInt64Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromResource(hinst: super::HINSTANCE, id: u32) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::HINSTANCE, id : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromResource(hinst, id, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromStrRet(pstrret: *const super::STRRET, pidl: *const super::ITEMIDLIST) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromStrRet(pstrret : *const super::STRRET, pidl : *const super::ITEMIDLIST, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromStrRet(pstrret, pidl, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[windows_core::PCWSTR]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const windows_core::PCWSTR, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromStringArray(prgsz.as_ptr(), prgsz.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[u16]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const u16, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt16Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[u32]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const u32, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt32Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[u64]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const u64, celems : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromUInt64Array(prgn.as_ptr(), prgn.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const super::VARIANT, ielem: u32) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const super::VARIANT, ielem : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitVariantFromVariantArrayElem(varin, ielem, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantChangeType(ppropvardest: *mut super::PROPVARIANT, propvarsrc: *const super::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: super::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut super::PROPVARIANT, propvarsrc : *const super::PROPVARIANT, flags : PROPVAR_CHANGE_FLAGS, vt : super::VARTYPE) -> windows_core::HRESULT);
    unsafe { PropVariantChangeType(ppropvardest, propvarsrc, flags, vt) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantCompareEx(propvar1: *const super::PROPVARIANT, propvar2: *const super::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const super::PROPVARIANT, propvar2 : *const super::PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
    unsafe { PropVariantCompareEx(propvar1, propvar2, unit, flags) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetBooleanElem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const super::PROPVARIANT, ielem : u32, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetBooleanElem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetDoubleElem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetDoubleElem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetElementCount(propvar: *const super::PROPVARIANT) -> u32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const super::PROPVARIANT) -> u32);
    unsafe { PropVariantGetElementCount(propvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetFileTimeElem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<super::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const super::PROPVARIANT, ielem : u32, pftval : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetFileTimeElem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt16Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt16Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt32Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt32Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetInt64Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetInt64Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetStringElem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const super::PROPVARIANT, ielem : u32, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetStringElem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt16Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt16Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt32Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt32Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantGetUInt64Elem(propvar: *const super::PROPVARIANT, ielem: u32) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantGetUInt64Elem(propvar, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBSTR(propvar: *const super::PROPVARIANT) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const super::PROPVARIANT, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToBSTR(propvar, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBoolean(propvarin: *const super::PROPVARIANT) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const super::PROPVARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToBoolean(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanVector(propvar: *const super::PROPVARIANT, prgf: &mut [windows_core::BOOL], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const super::PROPVARIANT, prgf : *mut windows_core::BOOL, crgf : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVector(propvar, prgf.as_mut_ptr(), prgf.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanVectorAlloc(propvar: *const super::PROPVARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const super::PROPVARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBooleanVectorAlloc(propvar, pprgf as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBooleanWithDefault(propvarin: *const super::PROPVARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const super::PROPVARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PropVariantToBooleanWithDefault(propvarin, fdefault.into()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToBuffer(propvar: *const super::PROPVARIANT, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const super::PROPVARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { PropVariantToBuffer(propvar, pv as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDouble(propvarin: *const super::PROPVARIANT) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const super::PROPVARIANT, pdblret : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToDouble(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleVector(propvar: *const super::PROPVARIANT, prgn: &mut [f64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const super::PROPVARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleVectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToDoubleVectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToDoubleWithDefault(propvarin: *const super::PROPVARIANT, dbldefault: f64) -> f64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const super::PROPVARIANT, dbldefault : f64) -> f64);
    unsafe { PropVariantToDoubleWithDefault(propvarin, dbldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTime(propvar: *const super::PROPVARIANT, pstfout: PSTIME_FLAGS) -> windows_core::Result<super::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const super::PROPVARIANT, pstfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToFileTime(propvar, pstfout, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVector(propvar: *const super::PROPVARIANT, prgft: &mut [super::FILETIME], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const super::PROPVARIANT, prgft : *mut super::FILETIME, crgft : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVector(propvar, prgft.as_mut_ptr(), prgft.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToFileTimeVectorAlloc(propvar: *const super::PROPVARIANT, pprgft: *mut *mut super::FILETIME, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const super::PROPVARIANT, pprgft : *mut *mut super::FILETIME, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToFileTimeVectorAlloc(propvar, pprgft as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToGUID(propvar: *const super::PROPVARIANT) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const super::PROPVARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToGUID(propvar, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16(propvarin: *const super::PROPVARIANT) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const super::PROPVARIANT, piret : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt16(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16Vector(propvar: *const super::PROPVARIANT, prgn: &mut [i16], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt16VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt16WithDefault(propvarin: *const super::PROPVARIANT, idefault: i16) -> i16 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const super::PROPVARIANT, idefault : i16) -> i16);
    unsafe { PropVariantToInt16WithDefault(propvarin, idefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32(propvarin: *const super::PROPVARIANT) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const super::PROPVARIANT, plret : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt32(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32Vector(propvar: *const super::PROPVARIANT, prgn: &mut [i32], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt32VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt32WithDefault(propvarin: *const super::PROPVARIANT, ldefault: i32) -> i32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const super::PROPVARIANT, ldefault : i32) -> i32);
    unsafe { PropVariantToInt32WithDefault(propvarin, ldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64(propvarin: *const super::PROPVARIANT) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const super::PROPVARIANT, pllret : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToInt64(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64Vector(propvar: *const super::PROPVARIANT, prgn: &mut [i64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToInt64VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToInt64WithDefault(propvarin: *const super::PROPVARIANT, lldefault: i64) -> i64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const super::PROPVARIANT, lldefault : i64) -> i64);
    unsafe { PropVariantToInt64WithDefault(propvarin, lldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStrRet(propvar: *const super::PROPVARIANT, pstrret: *mut super::STRRET) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStrRet(propvar : *const super::PROPVARIANT, pstrret : *mut super::STRRET) -> windows_core::HRESULT);
    unsafe { PropVariantToStrRet(propvar, pstrret as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToString(propvar: *const super::PROPVARIANT, psz: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const super::PROPVARIANT, psz : windows_core::PWSTR, cch : u32) -> windows_core::HRESULT);
    unsafe { PropVariantToString(propvar, core::mem::transmute(psz.as_mut_ptr()), psz.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringAlloc(propvar: *const super::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const super::PROPVARIANT, ppszout : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToStringAlloc(propvar, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringVector(propvar: *const super::PROPVARIANT, prgsz: &mut [windows_core::PWSTR], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const super::PROPVARIANT, prgsz : *mut windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVector(propvar, prgsz.as_mut_ptr(), prgsz.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringVectorAlloc(propvar: *const super::PROPVARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const super::PROPVARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToStringVectorAlloc(propvar, pprgsz as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToStringWithDefault<P1>(propvarin: *const super::PROPVARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const super::PROPVARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { PropVariantToStringWithDefault(propvarin, pszdefault.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16(propvarin: *const super::PROPVARIANT) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const super::PROPVARIANT, puiret : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt16(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16Vector(propvar: *const super::PROPVARIANT, prgn: &mut [u16], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt16VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt16WithDefault(propvarin: *const super::PROPVARIANT, uidefault: u16) -> u16 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const super::PROPVARIANT, uidefault : u16) -> u16);
    unsafe { PropVariantToUInt16WithDefault(propvarin, uidefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32(propvarin: *const super::PROPVARIANT) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const super::PROPVARIANT, pulret : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt32(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32Vector(propvar: *const super::PROPVARIANT, prgn: &mut [u32], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt32VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt32WithDefault(propvarin: *const super::PROPVARIANT, uldefault: u32) -> u32 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const super::PROPVARIANT, uldefault : u32) -> u32);
    unsafe { PropVariantToUInt32WithDefault(propvarin, uldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64(propvarin: *const super::PROPVARIANT) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const super::PROPVARIANT, pullret : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToUInt64(propvarin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64Vector(propvar: *const super::PROPVARIANT, prgn: &mut [u64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64Vector(propvar, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64VectorAlloc(propvar: *const super::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { PropVariantToUInt64VectorAlloc(propvar, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToUInt64WithDefault(propvarin: *const super::PROPVARIANT, ulldefault: u64) -> u64 {
    windows_core::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const super::PROPVARIANT, ulldefault : u64) -> u64);
    unsafe { PropVariantToUInt64WithDefault(propvarin, ulldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToVariant(ppropvar: *const super::PROPVARIANT) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const super::PROPVARIANT, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropVariantToVariant(ppropvar, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const super::SERIALIZEDPROPERTYVALUE, cbmax: u32) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbmax : u32, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgDeserializePropVariant(pprop, cbmax, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const super::PROPVARIANT, ppprop: *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const super::PROPVARIANT, ppprop : *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb : *mut u32) -> windows_core::HRESULT);
    unsafe { StgSerializePropVariant(ppropvar, ppprop as _, pcb as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const super::VARIANT, var2: *const super::VARIANT) -> i32 {
    windows_core::link!("propsys.dll" "system" fn VariantCompare(var1 : *const super::VARIANT, var2 : *const super::VARIANT) -> i32);
    unsafe { VariantCompare(var1, var2) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const super::VARIANT, ielem : u32, pfval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetBooleanElem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const super::VARIANT, ielem : u32, pnval : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetDoubleElem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const super::VARIANT) -> u32 {
    windows_core::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const super::VARIANT) -> u32);
    unsafe { VariantGetElementCount(varin) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt16Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt32Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetInt64Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const super::VARIANT, ielem : u32, ppszval : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetStringElem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt16Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt32Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const super::VARIANT, ielem: u32) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantGetUInt64Elem(var, ielem, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const super::VARIANT) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const super::VARIANT, pfret : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToBoolean(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const super::VARIANT, prgf: &mut [windows_core::BOOL], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const super::VARIANT, prgf : *mut windows_core::BOOL, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArray(var, prgf.as_mut_ptr(), prgf.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const super::VARIANT, pprgf: *mut *mut windows_core::BOOL, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const super::VARIANT, pprgf : *mut *mut windows_core::BOOL, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToBooleanArrayAlloc(var, pprgf as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault(varin: *const super::VARIANT, fdefault: bool) -> windows_core::BOOL {
    windows_core::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const super::VARIANT, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { VariantToBooleanWithDefault(varin, fdefault.into()) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const super::VARIANT, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const super::VARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { VariantToBuffer(varin, pv as _, cb) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const super::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const super::VARIANT, pwdate : *mut u16, pwtime : *mut u16) -> windows_core::HRESULT);
    unsafe { VariantToDosDateTime(varin, pwdate as _, pwtime as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const super::VARIANT) -> windows_core::Result<f64> {
    windows_core::link!("propsys.dll" "system" fn VariantToDouble(varin : *const super::VARIANT, pdblret : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToDouble(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const super::VARIANT, prgn: &mut [f64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const super::VARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArray(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToDoubleArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const super::VARIANT, dbldefault: f64) -> f64 {
    windows_core::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const super::VARIANT, dbldefault : f64) -> f64);
    unsafe { VariantToDoubleWithDefault(varin, dbldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const super::VARIANT, stfout: PSTIME_FLAGS) -> windows_core::Result<super::FILETIME> {
    windows_core::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const super::VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToFileTime(varin, stfout, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const super::VARIANT) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("propsys.dll" "system" fn VariantToGUID(varin : *const super::VARIANT, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToGUID(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const super::VARIANT) -> windows_core::Result<i16> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16(varin : *const super::VARIANT, piret : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt16(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const super::VARIANT, prgn: &mut [i16], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const super::VARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt16Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt16ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const super::VARIANT, idefault: i16) -> i16 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const super::VARIANT, idefault : i16) -> i16);
    unsafe { VariantToInt16WithDefault(varin, idefault) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const super::VARIANT) -> windows_core::Result<i32> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32(varin : *const super::VARIANT, plret : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt32(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const super::VARIANT, prgn: &mut [i32], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const super::VARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt32Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt32ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const super::VARIANT, ldefault: i32) -> i32 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const super::VARIANT, ldefault : i32) -> i32);
    unsafe { VariantToInt32WithDefault(varin, ldefault) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const super::VARIANT) -> windows_core::Result<i64> {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64(varin : *const super::VARIANT, pllret : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToInt64(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const super::VARIANT, prgn: &mut [i64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const super::VARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt64Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToInt64ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const super::VARIANT, lldefault: i64) -> i64 {
    windows_core::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const super::VARIANT, lldefault : i64) -> i64);
    unsafe { VariantToInt64WithDefault(varin, lldefault) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToPropVariant(pvar: *const super::VARIANT) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const super::VARIANT, ppropvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToPropVariant(pvar, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStrRet(varin: *const super::VARIANT, pstrret: *mut super::STRRET) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStrRet(varin : *const super::VARIANT, pstrret : *mut super::STRRET) -> windows_core::HRESULT);
    unsafe { VariantToStrRet(varin, pstrret as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToString(varin: *const super::VARIANT, pszbuf: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToString(varin : *const super::VARIANT, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::HRESULT);
    unsafe { VariantToString(varin, core::mem::transmute(pszbuf.as_mut_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const super::VARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const super::VARIANT, ppszbuf : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToStringAlloc(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const super::VARIANT, prgsz: &mut [windows_core::PWSTR], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArray(var : *const super::VARIANT, prgsz : *mut windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToStringArray(var, prgsz.as_mut_ptr(), prgsz.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const super::VARIANT, pprgsz: *mut *mut windows_core::PWSTR, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const super::VARIANT, pprgsz : *mut *mut windows_core::PWSTR, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToStringArrayAlloc(var, pprgsz as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<P1>(varin: *const super::VARIANT, pszdefault: P1) -> windows_core::PCWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const super::VARIANT, pszdefault : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { VariantToStringWithDefault(varin, pszdefault.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const super::VARIANT) -> windows_core::Result<u16> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const super::VARIANT, puiret : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt16(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const super::VARIANT, prgn: &mut [u16], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const super::VARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt16Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt16ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const super::VARIANT, uidefault: u16) -> u16 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const super::VARIANT, uidefault : u16) -> u16);
    unsafe { VariantToUInt16WithDefault(varin, uidefault) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const super::VARIANT) -> windows_core::Result<u32> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const super::VARIANT, pulret : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt32(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const super::VARIANT, prgn: &mut [u32], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const super::VARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt32Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt32ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const super::VARIANT, uldefault: u32) -> u32 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const super::VARIANT, uldefault : u32) -> u32);
    unsafe { VariantToUInt32WithDefault(varin, uldefault) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const super::VARIANT) -> windows_core::Result<u64> {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const super::VARIANT, pullret : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantToUInt64(varin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const super::VARIANT, prgn: &mut [u64], pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const super::VARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt64Array(var, prgn.as_mut_ptr(), prgn.len().try_into().unwrap(), pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const super::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_core::HRESULT);
    unsafe { VariantToUInt64ArrayAlloc(var, pprgn as _, pcelem as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const super::VARIANT, ulldefault: u64) -> u64 {
    windows_core::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const super::VARIANT, ulldefault : u64) -> u64);
    unsafe { VariantToUInt64WithDefault(varin, ulldefault) }
}
pub const DPF_ERROR: DRAWPROGRESSFLAGS = 4;
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = 1;
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = 2;
pub const DPF_NONE: DRAWPROGRESSFLAGS = 0;
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = 16;
pub const DPF_WARNING: DRAWPROGRESSFLAGS = 8;
pub type DRAWPROGRESSFLAGS = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_CHANGE_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPVAR_COMPARE_FLAGS(pub i32);
pub type PROPVAR_COMPARE_UNIT = i32;
pub const PSTF_LOCAL: tagPSTIME_FLAGS = 1;
pub const PSTF_UTC: tagPSTIME_FLAGS = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSTIME_FLAGS(pub i32);
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
