#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut super::PROPVARIANT, cvars : super::UINT));
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut super::VARIANT, cvars : super::UINT));
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const windows_sys::core::BOOL, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const core::ffi::c_void, cb : super::UINT, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const super::IID, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const super::DOUBLE, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::FILETIME, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::FILETIME, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const windows_sys::core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const super::SHORT, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const super::LONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const super::LONGLONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const super::PROPVARIANT, ielem : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::HINSTANCE, id : super::UINT, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStrRet(pstrret : *mut super::STRRET, pidl : super::LPCITEMIDLIST, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : windows_sys::core::PCWSTR, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const windows_sys::core::PCWSTR, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const super::USHORT, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const super::ULONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const super::ULONGLONG, celems : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const super::PROPVARIANT, ppropvarvector : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const windows_sys::core::BOOL, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const core::ffi::c_void, cb : super::UINT, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const super::DOUBLE, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::FILETIME, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::FILETIME, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const windows_sys::core::GUID, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const super::SHORT, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const super::LONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const super::LONGLONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::HINSTANCE, id : super::UINT, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromStrRet(pstrret : *const super::STRRET, pidl : super::LPCITEMIDLIST, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const windows_sys::core::PCWSTR, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const super::USHORT, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const super::ULONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const super::ULONGLONG, celems : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const super::VARIANT, ielem : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut super::PROPVARIANT, propvarsrc : *const super::PROPVARIANT, flags : PROPVAR_CHANGE_FLAGS, vt : super::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const super::PROPVARIANT, propvar2 : *const super::PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pfval : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const super::PROPVARIANT) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pftval : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::LONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, ppszval : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const super::PROPVARIANT, ielem : super::ULONG, pnval : *mut super::ULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const super::PROPVARIANT, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const super::PROPVARIANT, pfret : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const super::PROPVARIANT, prgf : *mut windows_sys::core::BOOL, crgf : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const super::PROPVARIANT, pprgf : *mut *mut windows_sys::core::BOOL, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const super::PROPVARIANT, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const super::PROPVARIANT, pv : *mut core::ffi::c_void, cb : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const super::PROPVARIANT, pdblret : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const super::PROPVARIANT, prgn : *mut super::DOUBLE, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::DOUBLE, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const super::PROPVARIANT, dbldefault : super::DOUBLE) -> super::DOUBLE);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const super::PROPVARIANT, pstfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const super::PROPVARIANT, prgft : *mut super::FILETIME, crgft : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const super::PROPVARIANT, pprgft : *mut *mut super::FILETIME, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const super::PROPVARIANT, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const super::PROPVARIANT, piret : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::SHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::SHORT, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const super::PROPVARIANT, idefault : super::SHORT) -> super::SHORT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const super::PROPVARIANT, plret : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::LONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::LONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const super::PROPVARIANT, ldefault : super::LONG) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const super::PROPVARIANT, pllret : *mut super::LONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::LONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::LONGLONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const super::PROPVARIANT, lldefault : super::LONGLONG) -> super::LONGLONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStrRet(propvar : *const super::PROPVARIANT, pstrret : *mut super::STRRET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const super::PROPVARIANT, psz : windows_sys::core::PWSTR, cch : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const super::PROPVARIANT, ppszout : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const super::PROPVARIANT, prgsz : *mut windows_sys::core::PWSTR, crgsz : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const super::PROPVARIANT, pprgsz : *mut *mut windows_sys::core::PWSTR, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const super::PROPVARIANT, pszdefault : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const super::PROPVARIANT, puiret : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::USHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::USHORT, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const super::PROPVARIANT, uidefault : super::USHORT) -> super::USHORT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const super::PROPVARIANT, pulret : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::ULONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const super::PROPVARIANT, uldefault : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const super::PROPVARIANT, pullret : *mut super::ULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut super::ULONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut super::ULONGLONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const super::PROPVARIANT, ulldefault : super::ULONGLONG) -> super::ULONGLONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const super::PROPVARIANT, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbmax : super::ULONG, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const super::PROPVARIANT, ppprop : *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantCompare(var1 : *const super::VARIANT, var2 : *const super::VARIANT) -> i32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const super::VARIANT, ielem : super::ULONG, pfval : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const super::VARIANT) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::LONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const super::VARIANT, ielem : super::ULONG, ppszval : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const super::VARIANT, ielem : super::ULONG, pnval : *mut super::ULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const super::VARIANT, pfret : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const super::VARIANT, prgf : *mut windows_sys::core::BOOL, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const super::VARIANT, pprgf : *mut *mut windows_sys::core::BOOL, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const super::VARIANT, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const super::VARIANT, pv : *mut core::ffi::c_void, cb : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const super::VARIANT, pwdate : *mut super::WORD, pwtime : *mut super::WORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDouble(varin : *const super::VARIANT, pdblret : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const super::VARIANT, prgn : *mut super::DOUBLE, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::DOUBLE, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const super::VARIANT, dbldefault : super::DOUBLE) -> super::DOUBLE);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const super::VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToGUID(varin : *const super::VARIANT, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16(varin : *const super::VARIANT, piret : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const super::VARIANT, prgn : *mut super::SHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::SHORT, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const super::VARIANT, idefault : super::SHORT) -> super::SHORT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32(varin : *const super::VARIANT, plret : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const super::VARIANT, prgn : *mut super::LONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::LONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const super::VARIANT, ldefault : super::LONG) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64(varin : *const super::VARIANT, pllret : *mut super::LONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const super::VARIANT, prgn : *mut super::LONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::LONGLONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const super::VARIANT, lldefault : super::LONGLONG) -> super::LONGLONG);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const super::VARIANT, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "shtypes", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStrRet(varin : *const super::VARIANT, pstrret : *mut super::STRRET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToString(varin : *const super::VARIANT, pszbuf : windows_sys::core::PWSTR, cchbuf : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const super::VARIANT, ppszbuf : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringArray(var : *const super::VARIANT, prgsz : *mut windows_sys::core::PWSTR, crgsz : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const super::VARIANT, pprgsz : *mut *mut windows_sys::core::PWSTR, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const super::VARIANT, pszdefault : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const super::VARIANT, puiret : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const super::VARIANT, prgn : *mut super::USHORT, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::USHORT, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const super::VARIANT, uidefault : super::USHORT) -> super::USHORT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const super::VARIANT, pulret : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const super::VARIANT, prgn : *mut super::ULONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const super::VARIANT, uldefault : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const super::VARIANT, pullret : *mut super::ULONGLONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const super::VARIANT, prgn : *mut super::ULONGLONG, crgn : super::ULONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut super::ULONGLONG, pcelem : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const super::VARIANT, ulldefault : super::ULONGLONG) -> super::ULONGLONG);
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
