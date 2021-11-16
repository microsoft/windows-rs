#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ClearPropVariantArray(rgpropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, cvars: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ClearVariantArray(pvars: *mut super::super::super::System::Com::VARIANT, cvars: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBooleanVector(prgf: *const super::super::super::Foundation::BOOL, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromCLSID(clsid: *const ::windows_sys::core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTimeVector(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromGUIDAsString(guid: *const ::windows_sys::core::GUID, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromPropVariantVectorElem(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringAsVector(psz: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringVector(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantVectorFromPropVariant(propvarsingle: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarvector: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBooleanArray(prgf: *const super::super::super::Foundation::BOOL, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTimeArray(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromGUIDAsString(guid: *const ::windows_sys::core::GUID, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromStringArray(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromVariantArrayElem(varin: *const super::super::super::System::Com::VARIANT, ielem: u32, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    pub fn PSCreateAdapterFromPropertyStore(pps: IPropertyStore, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: IDelayedPropertyStoreFactory, rgstoreids: *const u32, cstores: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PSCreateMemoryPropertyStore(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::windows_sys::core::IUnknown, cstores: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, cchanges: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PSCreatePropertyStoreFromObject(punk: ::windows_sys::core::IUnknown, grfmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn PSCreatePropertyStoreFromPropertySetStorage(ppss: super::super::super::System::Com::StructuredStorage::IPropertySetStorage, grfmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSFormatPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandler(punkitem: ::windows_sys::core::IUnknown, freadwrite: super::super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandlerWithCreateObject(punkitem: ::windows_sys::core::IUnknown, freadwrite: super::super::super::Foundation::BOOL, punkcreateobject: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    pub fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionByName(pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionListFromString(pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyKeyFromName(pszname: super::super::super::Foundation::PWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
    pub fn PSGetPropertySystem(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSLookupPropertyHandlerCLSID(pszfilepath: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_Delete(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBOOL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBSTR(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadDWORD(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadGUID(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadInt(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadLONG(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTS(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPropertyKey(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadRECTL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::RECTL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadSHORT(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStr(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, charactercount: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStrAlloc(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStream(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PSPropertyBag_ReadType(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, var: *mut super::super::super::System::Com::VARIANT, r#type: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadULONGLONG(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadUnknown(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBOOL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBSTR(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteDWORD(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteGUID(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteInt(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteLONG(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTS(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePropertyKey(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const PROPERTYKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteRECTL(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::RECTL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteSHORT(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStr(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStream(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: super::super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteULONGLONG(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, value: u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteUnknown(propbag: super::super::super::System::Com::StructuredStorage::IPropertyBag, propname: super::super::super::Foundation::PWSTR, punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSPropertyKeyFromString(pszstring: super::super::super::Foundation::PWSTR, pkey: *mut PROPERTYKEY) -> ::windows_sys::core::HRESULT;
    pub fn PSRefreshPropertySchema() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSRegisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSSetPropertyValue(pps: IPropertyStore, ppd: IPropertyDescription, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSUnregisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_CloseProperties(hprops: super::super::super::Foundation::HANDLE, flopt: u32) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_GetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_OpenProperties(pszapp: super::super::super::Foundation::PWSTR, pszpif: super::super::super::Foundation::PWSTR, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_SetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantChangeType(ppropvardest: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvarsrc: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantCompareEx(propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetBooleanElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetDoubleElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetElementCount(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetFileTimeElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pftval: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetStringElem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt16Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt32Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt64Elem(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ielem: u32, pnval: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBSTR(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pbstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBoolean(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBuffer(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDouble(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdblret: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, dbldefault: f64) -> f64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTime(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToGUID(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, piret: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, idefault: i16) -> i16;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, plret: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ldefault: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pllret: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, lldefault: i64) -> i64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn PropVariantToStrRet(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToString(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszout: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringWithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiret: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uidefault: u16) -> u16;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pulret: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, uldefault: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pullret: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64Vector(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64VectorAlloc(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64WithDefault(propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ulldefault: u64) -> u64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PropVariantToVariant(ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToWinRTPropertyValue(propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAddDefaultPropertiesByExt(pszext: super::super::super::Foundation::PWSTR, ppropstore: IPropertyStore) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetPropertyStoreForWindow(hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHGetPropertyStoreFromParsingName(pszpath: super::super::super::Foundation::PWSTR, pbc: super::super::super::System::Com::IBindCtx, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn SHPropStgCreate(psstg: super::super::super::System::Com::StructuredStorage::IPropertySetStorage, fmtid: *const ::windows_sys::core::GUID, pclsid: *const ::windows_sys::core::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut super::super::super::System::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgReadMultiple(pps: super::super::super::System::Com::StructuredStorage::IPropertyStorage, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgWriteMultiple(pps: super::super::super::System::Com::StructuredStorage::IPropertyStorage, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantCompare(var1: *const super::super::super::System::Com::VARIANT, var2: *const super::super::super::System::Com::VARIANT) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetBooleanElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetDoubleElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetElementCount(varin: *const super::super::super::System::Com::VARIANT) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetStringElem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt16Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt32Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt64Elem(var: *const super::super::super::System::Com::VARIANT, ielem: u32, pnval: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBoolean(varin: *const super::super::super::System::Com::VARIANT, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArray(var: *const super::super::super::System::Com::VARIANT, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanWithDefault(varin: *const super::super::super::System::Com::VARIANT, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBuffer(varin: *const super::super::super::System::Com::VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDosDateTime(varin: *const super::super::super::System::Com::VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDouble(varin: *const super::super::super::System::Com::VARIANT, pdblret: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArray(var: *const super::super::super::System::Com::VARIANT, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleWithDefault(varin: *const super::super::super::System::Com::VARIANT, dbldefault: f64) -> f64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToFileTime(varin: *const super::super::super::System::Com::VARIANT, stfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToGUID(varin: *const super::super::super::System::Com::VARIANT, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16(varin: *const super::super::super::System::Com::VARIANT, piret: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, idefault: i16) -> i16;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32(varin: *const super::super::super::System::Com::VARIANT, plret: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, ldefault: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64(varin: *const super::super::super::System::Com::VARIANT, pllret: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, lldefault: i64) -> i64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn VariantToPropVariant(pvar: *const super::super::super::System::Com::VARIANT, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn VariantToStrRet(varin: *const super::super::super::System::Com::VARIANT, pstrret: *mut super::Common::STRRET) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToString(varin: *const super::super::super::System::Com::VARIANT, pszbuf: super::super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringAlloc(varin: *const super::super::super::System::Com::VARIANT, ppszbuf: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArray(var: *const super::super::super::System::Com::VARIANT, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringWithDefault(varin: *const super::super::super::System::Com::VARIANT, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16(varin: *const super::super::super::System::Com::VARIANT, puiret: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16WithDefault(varin: *const super::super::super::System::Com::VARIANT, uidefault: u16) -> u16;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32(varin: *const super::super::super::System::Com::VARIANT, pulret: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32WithDefault(varin: *const super::super::super::System::Com::VARIANT, uldefault: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64(varin: *const super::super::super::System::Com::VARIANT, pullret: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64Array(var: *const super::super::super::System::Com::VARIANT, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64ArrayAlloc(var: *const super::super::super::System::Com::VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64WithDefault(varin: *const super::super::super::System::Com::VARIANT, ulldefault: u64) -> u64;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn WinRTPropertyValueToPropVariant(punkpropertyvalue: ::windows_sys::core::IUnknown, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
}
pub const DPF_NONE: i32 = 0i32;
pub const DPF_MARQUEE: i32 = 1i32;
pub const DPF_MARQUEE_COMPLETE: i32 = 2i32;
pub const DPF_ERROR: i32 = 4i32;
pub const DPF_WARNING: i32 = 8i32;
pub const DPF_STOPPED: i32 = 16i32;
pub const GPS_DEFAULT: i32 = 0i32;
pub const GPS_HANDLERPROPERTIESONLY: i32 = 1i32;
pub const GPS_READWRITE: i32 = 2i32;
pub const GPS_TEMPORARY: i32 = 4i32;
pub const GPS_FASTPROPERTIESONLY: i32 = 8i32;
pub const GPS_OPENSLOWITEM: i32 = 16i32;
pub const GPS_DELAYCREATION: i32 = 32i32;
pub const GPS_BESTEFFORT: i32 = 64i32;
pub const GPS_NO_OPLOCK: i32 = 128i32;
pub const GPS_PREFERQUERYPROPERTIES: i32 = 256i32;
pub const GPS_EXTRINSICPROPERTIES: i32 = 512i32;
pub const GPS_EXTRINSICPROPERTIESONLY: i32 = 1024i32;
pub const GPS_VOLATILEPROPERTIES: i32 = 2048i32;
pub const GPS_VOLATILEPROPERTIESONLY: i32 = 4096i32;
pub const GPS_MASK_VALID: i32 = 8191i32;
#[repr(transparent)]
pub struct ICreateObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICreateObject {}
impl ::core::clone::Clone for ICreateObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDelayedPropertyStoreFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDelayedPropertyStoreFactory {}
impl ::core::clone::Clone for IDelayedPropertyStoreFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitializeWithFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitializeWithFile {}
impl ::core::clone::Clone for IInitializeWithFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitializeWithStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitializeWithStream {}
impl ::core::clone::Clone for IInitializeWithStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INamedPropertyStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INamedPropertyStore {}
impl ::core::clone::Clone for INamedPropertyStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IObjectWithPropertyKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IObjectWithPropertyKey {}
impl ::core::clone::Clone for IObjectWithPropertyKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistSerializedPropStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistSerializedPropStorage {}
impl ::core::clone::Clone for IPersistSerializedPropStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersistSerializedPropStorage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersistSerializedPropStorage2 {}
impl ::core::clone::Clone for IPersistSerializedPropStorage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyChange {}
impl ::core::clone::Clone for IPropertyChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyChangeArray(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyChangeArray {}
impl ::core::clone::Clone for IPropertyChangeArray {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescription {}
impl ::core::clone::Clone for IPropertyDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescription2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescription2 {}
impl ::core::clone::Clone for IPropertyDescription2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescriptionAliasInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescriptionAliasInfo {}
impl ::core::clone::Clone for IPropertyDescriptionAliasInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescriptionList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescriptionList {}
impl ::core::clone::Clone for IPropertyDescriptionList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescriptionRelatedPropertyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescriptionRelatedPropertyInfo {}
impl ::core::clone::Clone for IPropertyDescriptionRelatedPropertyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyDescriptionSearchInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyDescriptionSearchInfo {}
impl ::core::clone::Clone for IPropertyDescriptionSearchInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyEnumType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyEnumType {}
impl ::core::clone::Clone for IPropertyEnumType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyEnumType2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyEnumType2 {}
impl ::core::clone::Clone for IPropertyEnumType2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyEnumTypeList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyEnumTypeList {}
impl ::core::clone::Clone for IPropertyEnumTypeList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyStore {}
impl ::core::clone::Clone for IPropertyStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyStoreCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyStoreCache {}
impl ::core::clone::Clone for IPropertyStoreCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyStoreCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyStoreCapabilities {}
impl ::core::clone::Clone for IPropertyStoreCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyStoreFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyStoreFactory {}
impl ::core::clone::Clone for IPropertyStoreFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySystem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySystem {}
impl ::core::clone::Clone for IPropertySystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySystemChangeNotify(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySystemChangeNotify {}
impl ::core::clone::Clone for IPropertySystemChangeNotify {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyUI {}
impl ::core::clone::Clone for IPropertyUI {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InMemoryPropertyStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2583879698,
    data2: 25347,
    data3: 19998,
    data4: [185, 161, 99, 15, 128, 37, 146, 197],
};
pub const InMemoryPropertyStoreMarshalByValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3570011693,
    data2: 28071,
    data3: 19317,
    data4: [169, 124, 95, 48, 111, 14, 174, 220],
};
pub const PDOPS_RUNNING: i32 = 1i32;
pub const PDOPS_PAUSED: i32 = 2i32;
pub const PDOPS_CANCELLED: i32 = 3i32;
pub const PDOPS_STOPPED: i32 = 4i32;
pub const PDOPS_ERRORS: i32 = 5i32;
pub const PKA_SET: i32 = 0i32;
pub const PKA_APPEND: i32 = 1i32;
pub const PKA_DELETE: i32 = 2i32;
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
pub const PS_NONE: i32 = 0i32;
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: i32 = 1i32;
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: i32 = 2i32;
pub const PS_CREATE_FILE_ACCESSIBLE: i32 = 4i32;
pub const PS_CLOUDFILE_PLACEHOLDER: i32 = 8i32;
pub const PS_DEFAULT: i32 = 7i32;
pub const PS_ALL: i32 = 15i32;
pub const PDAT_DEFAULT: i32 = 0i32;
pub const PDAT_FIRST: i32 = 1i32;
pub const PDAT_SUM: i32 = 2i32;
pub const PDAT_AVERAGE: i32 = 3i32;
pub const PDAT_DATERANGE: i32 = 4i32;
pub const PDAT_UNION: i32 = 5i32;
pub const PDAT_MAX: i32 = 6i32;
pub const PDAT_MIN: i32 = 7i32;
pub const PDCIT_NONE: i32 = 0i32;
pub const PDCIT_ONDISK: i32 = 1i32;
pub const PDCIT_INMEMORY: i32 = 2i32;
pub const PDCIT_ONDEMAND: i32 = 3i32;
pub const PDCIT_ONDISKALL: i32 = 4i32;
pub const PDCIT_ONDISKVECTOR: i32 = 5i32;
pub const PDCOT_NONE: i32 = 0i32;
pub const PDCOT_STRING: i32 = 1i32;
pub const PDCOT_SIZE: i32 = 2i32;
pub const PDCOT_DATETIME: i32 = 3i32;
pub const PDCOT_BOOLEAN: i32 = 4i32;
pub const PDCOT_NUMBER: i32 = 5i32;
pub const PDDT_STRING: i32 = 0i32;
pub const PDDT_NUMBER: i32 = 1i32;
pub const PDDT_BOOLEAN: i32 = 2i32;
pub const PDDT_DATETIME: i32 = 3i32;
pub const PDDT_ENUMERATED: i32 = 4i32;
pub const PDEF_ALL: i32 = 0i32;
pub const PDEF_SYSTEM: i32 = 1i32;
pub const PDEF_NONSYSTEM: i32 = 2i32;
pub const PDEF_VIEWABLE: i32 = 3i32;
pub const PDEF_QUERYABLE: i32 = 4i32;
pub const PDEF_INFULLTEXTQUERY: i32 = 5i32;
pub const PDEF_COLUMN: i32 = 6i32;
pub const PDFF_DEFAULT: i32 = 0i32;
pub const PDFF_PREFIXNAME: i32 = 1i32;
pub const PDFF_FILENAME: i32 = 2i32;
pub const PDFF_ALWAYSKB: i32 = 4i32;
pub const PDFF_RESERVED_RIGHTTOLEFT: i32 = 8i32;
pub const PDFF_SHORTTIME: i32 = 16i32;
pub const PDFF_LONGTIME: i32 = 32i32;
pub const PDFF_HIDETIME: i32 = 64i32;
pub const PDFF_SHORTDATE: i32 = 128i32;
pub const PDFF_LONGDATE: i32 = 256i32;
pub const PDFF_HIDEDATE: i32 = 512i32;
pub const PDFF_RELATIVEDATE: i32 = 1024i32;
pub const PDFF_USEEDITINVITATION: i32 = 2048i32;
pub const PDFF_READONLY: i32 = 4096i32;
pub const PDFF_NOAUTOREADINGORDER: i32 = 8192i32;
pub const PDGR_DISCRETE: i32 = 0i32;
pub const PDGR_ALPHANUMERIC: i32 = 1i32;
pub const PDGR_SIZE: i32 = 2i32;
pub const PDGR_DYNAMIC: i32 = 3i32;
pub const PDGR_DATE: i32 = 4i32;
pub const PDGR_PERCENT: i32 = 5i32;
pub const PDGR_ENUMERATED: i32 = 6i32;
pub const PDRDT_GENERAL: i32 = 0i32;
pub const PDRDT_DATE: i32 = 1i32;
pub const PDRDT_SIZE: i32 = 2i32;
pub const PDRDT_COUNT: i32 = 3i32;
pub const PDRDT_REVISION: i32 = 4i32;
pub const PDRDT_LENGTH: i32 = 5i32;
pub const PDRDT_DURATION: i32 = 6i32;
pub const PDRDT_SPEED: i32 = 7i32;
pub const PDRDT_RATE: i32 = 8i32;
pub const PDRDT_RATING: i32 = 9i32;
pub const PDRDT_PRIORITY: i32 = 10i32;
pub const PDSIF_DEFAULT: i32 = 0i32;
pub const PDSIF_ININVERTEDINDEX: i32 = 1i32;
pub const PDSIF_ISCOLUMN: i32 = 2i32;
pub const PDSIF_ISCOLUMNSPARSE: i32 = 4i32;
pub const PDSIF_ALWAYSINCLUDE: i32 = 8i32;
pub const PDSIF_USEFORTYPEAHEAD: i32 = 16i32;
pub const PDSD_GENERAL: i32 = 0i32;
pub const PDSD_A_Z: i32 = 1i32;
pub const PDSD_LOWEST_HIGHEST: i32 = 2i32;
pub const PDSD_SMALLEST_BIGGEST: i32 = 3i32;
pub const PDSD_OLDEST_NEWEST: i32 = 4i32;
pub const PDTF_DEFAULT: i32 = 0i32;
pub const PDTF_MULTIPLEVALUES: i32 = 1i32;
pub const PDTF_ISINNATE: i32 = 2i32;
pub const PDTF_ISGROUP: i32 = 4i32;
pub const PDTF_CANGROUPBY: i32 = 8i32;
pub const PDTF_CANSTACKBY: i32 = 16i32;
pub const PDTF_ISTREEPROPERTY: i32 = 32i32;
pub const PDTF_INCLUDEINFULLTEXTQUERY: i32 = 64i32;
pub const PDTF_ISVIEWABLE: i32 = 128i32;
pub const PDTF_ISQUERYABLE: i32 = 256i32;
pub const PDTF_CANBEPURGED: i32 = 512i32;
pub const PDTF_SEARCHRAWVALUE: i32 = 1024i32;
pub const PDTF_DONTCOERCEEMPTYSTRINGS: i32 = 2048i32;
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: i32 = 4096i32;
pub const PDTF_ISSYSTEMPROPERTY: i32 = -2147483648i32;
pub const PDTF_MASK_ALL: i32 = -2147475457i32;
pub const PDVF_DEFAULT: i32 = 0i32;
pub const PDVF_CENTERALIGN: i32 = 1i32;
pub const PDVF_RIGHTALIGN: i32 = 2i32;
pub const PDVF_BEGINNEWGROUP: i32 = 4i32;
pub const PDVF_FILLAREA: i32 = 8i32;
pub const PDVF_SORTDESCENDING: i32 = 16i32;
pub const PDVF_SHOWONLYIFPRESENT: i32 = 32i32;
pub const PDVF_SHOWBYDEFAULT: i32 = 64i32;
pub const PDVF_SHOWINPRIMARYLIST: i32 = 128i32;
pub const PDVF_SHOWINSECONDARYLIST: i32 = 256i32;
pub const PDVF_HIDELABEL: i32 = 512i32;
pub const PDVF_HIDDEN: i32 = 2048i32;
pub const PDVF_CANWRAP: i32 = 4096i32;
pub const PDVF_MASK_ALL: i32 = 7167i32;
pub const PET_DISCRETEVALUE: i32 = 0i32;
pub const PET_RANGEDVALUE: i32 = 1i32;
pub const PET_DEFAULTVALUE: i32 = 2i32;
pub const PET_ENDRANGE: i32 = 3i32;
#[repr(C)]
pub struct PROPERTYKEY {
    pub fmtid: ::windows_sys::core::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PUIF_DEFAULT: i32 = 0i32;
pub const PUIF_RIGHTALIGN: i32 = 1i32;
pub const PUIF_NOLABELININFOTIP: i32 = 2i32;
pub const PUIFFDF_DEFAULT: i32 = 0i32;
pub const PUIFFDF_RIGHTTOLEFT: i32 = 1i32;
pub const PUIFFDF_SHORTFORMAT: i32 = 2i32;
pub const PUIFFDF_NOTIME: i32 = 4i32;
pub const PUIFFDF_FRIENDLYDATE: i32 = 8i32;
pub const PUIFNF_DEFAULT: i32 = 0i32;
pub const PUIFNF_MNEMONIC: i32 = 1i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [super::super::super::Foundation::CHAR; 30],
    pub achCmdLine: [super::super::super::Foundation::CHAR; 128],
    pub achWorkDir: [super::super::super::Foundation::CHAR; 64],
    pub wHotKey: u16,
    pub achIconFile: [super::super::super::Foundation::CHAR; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [super::super::super::Foundation::CHAR; 80],
    pub achPIFFile: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPPRG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PVCHF_DEFAULT: i32 = 0i32;
pub const PVCHF_NOVALUEPROP: i32 = 1i32;
pub const PVCHF_ALPHABOOL: i32 = 2i32;
pub const PVCHF_NOUSEROVERRIDE: i32 = 4i32;
pub const PVCHF_LOCALBOOL: i32 = 8i32;
pub const PVCHF_NOHEXSTRING: i32 = 16i32;
pub const PVCF_DEFAULT: i32 = 0i32;
pub const PVCF_TREATEMPTYASGREATERTHAN: i32 = 1i32;
pub const PVCF_USESTRCMP: i32 = 2i32;
pub const PVCF_USESTRCMPC: i32 = 4i32;
pub const PVCF_USESTRCMPI: i32 = 8i32;
pub const PVCF_USESTRCMPIC: i32 = 16i32;
pub const PVCF_DIGITSASNUMBERS_CASESENSITIVE: i32 = 32i32;
pub const PVCU_DEFAULT: i32 = 0i32;
pub const PVCU_SECOND: i32 = 1i32;
pub const PVCU_MINUTE: i32 = 2i32;
pub const PVCU_HOUR: i32 = 3i32;
pub const PVCU_DAY: i32 = 4i32;
pub const PVCU_MONTH: i32 = 5i32;
pub const PVCU_YEAR: i32 = 6i32;
pub const PSC_NORMAL: i32 = 0i32;
pub const PSC_NOTINSOURCE: i32 = 1i32;
pub const PSC_DIRTY: i32 = 2i32;
pub const PSC_READONLY: i32 = 3i32;
pub const PSTF_UTC: i32 = 0i32;
pub const PSTF_LOCAL: i32 = 1i32;
pub const PropertySystem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3096870789, data2: 22702, data3: 20294, data4: [159, 178, 93, 121, 4, 121, 143, 75] };
#[repr(C)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
pub const SESF_NONE: i32 = 0i32;
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: i32 = 1i32;
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: i32 = 2i32;
pub const SESF_AUTHENTICATION_ERROR: i32 = 4i32;
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: i32 = 8i32;
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: i32 = 16i32;
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: i32 = 32i32;
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: i32 = 64i32;
pub const SESF_SERVICE_UNAVAILABLE: i32 = 128i32;
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: i32 = 256i32;
pub const SESF_ALL_FLAGS: i32 = 511i32;
pub const STS_NONE: i32 = 0i32;
pub const STS_NEEDSUPLOAD: i32 = 1i32;
pub const STS_NEEDSDOWNLOAD: i32 = 2i32;
pub const STS_TRANSFERRING: i32 = 4i32;
pub const STS_PAUSED: i32 = 8i32;
pub const STS_HASERROR: i32 = 16i32;
pub const STS_FETCHING_METADATA: i32 = 32i32;
pub const STS_USER_REQUESTED_REFRESH: i32 = 64i32;
pub const STS_HASWARNING: i32 = 128i32;
pub const STS_EXCLUDED: i32 = 256i32;
pub const STS_INCOMPLETE: i32 = 512i32;
pub const STS_PLACEHOLDER_IFEMPTY: i32 = 1024i32;
pub const FPSPS_DEFAULT: i32 = 0i32;
pub const FPSPS_READONLY: i32 = 1i32;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: i32 = 2i32;
