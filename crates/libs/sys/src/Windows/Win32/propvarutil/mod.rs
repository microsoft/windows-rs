#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn ClearPropVariantArray(rgpropvar : *mut super::PROPVARIANT, cvars : u32));
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut super::VARIANT, cvars : u32));
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromBooleanVector(prgf : *const windows_sys::core::BOOL, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromCLSID(clsid : *const windows_sys::core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromDoubleVector(prgn : *const f64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromFileTime(pftin : *const super::FILETIME, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromFileTimeVector(prgft : *const super::FILETIME, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromGUIDAsString(guid : *const windows_sys::core::GUID, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt16Vector(prgn : *const i16, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt32Vector(prgn : *const i32, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromInt64Vector(prgn : *const i64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromPropVariantVectorElem(propvarin : *const super::PROPVARIANT, ielem : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromResource(hinst : super::HINSTANCE, id : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStrRet(pstrret : *mut super::STRRET, pidl : *const super::ITEMIDLIST, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStringAsVector(psz : windows_sys::core::PCWSTR, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromStringVector(prgsz : *const windows_sys::core::PCWSTR, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt16Vector(prgn : *const u16, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt32Vector(prgn : *const u32, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantFromUInt64Vector(prgn : *const u64, celems : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitPropVariantVectorFromPropVariant(propvarsingle : *const super::PROPVARIANT, ppropvarvector : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const windows_sys::core::BOOL, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const core::ffi::c_void, cb : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const f64, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::FILETIME, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::FILETIME, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const windows_sys::core::GUID, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const i16, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const i32, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const i64, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::HINSTANCE, id : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromStrRet(pstrret : *const super::STRRET, pidl : *const super::ITEMIDLIST, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const windows_sys::core::PCWSTR, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const u16, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const u32, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const u64, celems : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const super::VARIANT, ielem : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantChangeType(ppropvardest : *mut super::PROPVARIANT, propvarsrc : *const super::PROPVARIANT, flags : PROPVAR_CHANGE_FLAGS, vt : super::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantCompareEx(propvar1 : *const super::PROPVARIANT, propvar2 : *const super::PROPVARIANT, unit : PROPVAR_COMPARE_UNIT, flags : PROPVAR_COMPARE_FLAGS) -> i32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetBooleanElem(propvar : *const super::PROPVARIANT, ielem : u32, pfval : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetDoubleElem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetElementCount(propvar : *const super::PROPVARIANT) -> u32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetFileTimeElem(propvar : *const super::PROPVARIANT, ielem : u32, pftval : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt16Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt32Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetInt64Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetStringElem(propvar : *const super::PROPVARIANT, ielem : u32, ppszval : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt16Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt32Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantGetUInt64Elem(propvar : *const super::PROPVARIANT, ielem : u32, pnval : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBSTR(propvar : *const super::PROPVARIANT, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBoolean(propvarin : *const super::PROPVARIANT, pfret : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanVector(propvar : *const super::PROPVARIANT, prgf : *mut windows_sys::core::BOOL, crgf : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanVectorAlloc(propvar : *const super::PROPVARIANT, pprgf : *mut *mut windows_sys::core::BOOL, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBooleanWithDefault(propvarin : *const super::PROPVARIANT, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToBuffer(propvar : *const super::PROPVARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDouble(propvarin : *const super::PROPVARIANT, pdblret : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleVector(propvar : *const super::PROPVARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleVectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToDoubleWithDefault(propvarin : *const super::PROPVARIANT, dbldefault : f64) -> f64);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTime(propvar : *const super::PROPVARIANT, pstfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTimeVector(propvar : *const super::PROPVARIANT, prgft : *mut super::FILETIME, crgft : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToFileTimeVectorAlloc(propvar : *const super::PROPVARIANT, pprgft : *mut *mut super::FILETIME, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToGUID(propvar : *const super::PROPVARIANT, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16(propvarin : *const super::PROPVARIANT, piret : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt16WithDefault(propvarin : *const super::PROPVARIANT, idefault : i16) -> i16);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32(propvarin : *const super::PROPVARIANT, plret : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt32WithDefault(propvarin : *const super::PROPVARIANT, ldefault : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64(propvarin : *const super::PROPVARIANT, pllret : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToInt64WithDefault(propvarin : *const super::PROPVARIANT, lldefault : i64) -> i64);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStrRet(propvar : *const super::PROPVARIANT, pstrret : *mut super::STRRET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToString(propvar : *const super::PROPVARIANT, psz : windows_sys::core::PWSTR, cch : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringAlloc(propvar : *const super::PROPVARIANT, ppszout : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringVector(propvar : *const super::PROPVARIANT, prgsz : *mut windows_sys::core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringVectorAlloc(propvar : *const super::PROPVARIANT, pprgsz : *mut *mut windows_sys::core::PWSTR, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToStringWithDefault(propvarin : *const super::PROPVARIANT, pszdefault : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16(propvarin : *const super::PROPVARIANT, puiret : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16Vector(propvar : *const super::PROPVARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt16WithDefault(propvarin : *const super::PROPVARIANT, uidefault : u16) -> u16);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32(propvarin : *const super::PROPVARIANT, pulret : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32Vector(propvar : *const super::PROPVARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt32WithDefault(propvarin : *const super::PROPVARIANT, uldefault : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64(propvarin : *const super::PROPVARIANT, pullret : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64Vector(propvar : *const super::PROPVARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64VectorAlloc(propvar : *const super::PROPVARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToUInt64WithDefault(propvarin : *const super::PROPVARIANT, ulldefault : u64) -> u64);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToVariant(ppropvar : *const super::PROPVARIANT, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn StgDeserializePropVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbmax : u32, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidl", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn StgSerializePropVariant(ppropvar : *const super::PROPVARIANT, ppprop : *mut *mut super::SERIALIZEDPROPERTYVALUE, pcb : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantCompare(var1 : *const super::VARIANT, var2 : *const super::VARIANT) -> i32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const super::VARIANT, ielem : u32, pfval : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const super::VARIANT, ielem : u32, pnval : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const super::VARIANT) -> u32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const super::VARIANT, ielem : u32, ppszval : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const super::VARIANT, ielem : u32, pnval : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const super::VARIANT, pfret : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const super::VARIANT, prgf : *mut windows_sys::core::BOOL, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const super::VARIANT, pprgf : *mut *mut windows_sys::core::BOOL, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const super::VARIANT, fdefault : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const super::VARIANT, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const super::VARIANT, pwdate : *mut u16, pwtime : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDouble(varin : *const super::VARIANT, pdblret : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const super::VARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const super::VARIANT, dbldefault : f64) -> f64);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const super::VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToGUID(varin : *const super::VARIANT, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16(varin : *const super::VARIANT, piret : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const super::VARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const super::VARIANT, idefault : i16) -> i16);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32(varin : *const super::VARIANT, plret : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const super::VARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const super::VARIANT, ldefault : i32) -> i32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64(varin : *const super::VARIANT, pllret : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const super::VARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const super::VARIANT, lldefault : i64) -> i64);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToPropVariant(pvar : *const super::VARIANT, ppropvar : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shtypes", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStrRet(varin : *const super::VARIANT, pstrret : *mut super::STRRET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToString(varin : *const super::VARIANT, pszbuf : windows_sys::core::PWSTR, cchbuf : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const super::VARIANT, ppszbuf : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringArray(var : *const super::VARIANT, prgsz : *mut windows_sys::core::PWSTR, crgsz : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const super::VARIANT, pprgsz : *mut *mut windows_sys::core::PWSTR, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const super::VARIANT, pszdefault : windows_sys::core::PCWSTR) -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const super::VARIANT, puiret : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const super::VARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const super::VARIANT, uidefault : u16) -> u16);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const super::VARIANT, pulret : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const super::VARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const super::VARIANT, uldefault : u32) -> u32);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const super::VARIANT, pullret : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const super::VARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const super::VARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const super::VARIANT, ulldefault : u64) -> u64);
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
