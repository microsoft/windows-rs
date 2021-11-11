#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ClearPropVariantArray(rgpropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cvars: u32);
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ClearVariantArray(pvars: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, cvars: u32);
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBooleanVector(prgf: *const super::super::super::Foundation::BOOL, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromCLSID(clsid: *const ::windows::runtime::GUID, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromDoubleVector(prgn: *const f64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTime(pftin: *const super::super::super::Foundation::FILETIME, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTimeVector(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt16Vector(prgn: *const i16, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt32Vector(prgn: *const i32, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt64Vector(prgn: *const i64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromPropVariantVectorElem(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn InitPropVariantFromStrRet(pstrret: *mut super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringAsVector(psz: super::super::super::Foundation::PWSTR, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringVector(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt16Vector(prgn: *const u16, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt32Vector(prgn: *const u32, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt64Vector(prgn: *const u64, celems: u32, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantVectorFromPropVariant(propvarsingle: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvarvector: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBooleanArray(prgf: *const super::super::super::Foundation::BOOL, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromDoubleArray(prgn: *const f64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTime(pft: *const super::super::super::Foundation::FILETIME, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTimeArray(prgft: *const super::super::super::Foundation::FILETIME, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromGUIDAsString(guid: *const ::windows::runtime::GUID, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt16Array(prgn: *const i16, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt32Array(prgn: *const i32, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt64Array(prgn: *const i64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromResource(hinst: super::super::super::Foundation::HINSTANCE, id: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn InitVariantFromStrRet(pstrret: *const super::Common::STRRET, pidl: *const super::Common::ITEMIDLIST, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromStringArray(prgsz: *const super::super::super::Foundation::PWSTR, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt16Array(prgn: *const u16, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt32Array(prgn: *const u32, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt64Array(prgn: *const u64, celems: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromVariantArrayElem(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateAdapterFromPropertyStore(pps: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateDelayedMultiplexPropertyStore(flags: GETPROPERTYSTOREFLAGS, pdpsf: ::windows::runtime::RawPtr, rgstoreids: *const u32, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateMemoryPropertyStore(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateMultiplexPropertyStore(prgpunkstores: *const ::windows::runtime::RawPtr, cstores: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreatePropertyChangeArray(rgpropkey: *const PROPERTYKEY, rgflags: *const PKA_FLAGS, rgpropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cchanges: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreatePropertyStoreFromObject(punk: ::windows::runtime::RawPtr, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn PSCreatePropertyStoreFromPropertySetStorage(ppss: ::windows::runtime::RawPtr, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSFormatPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandler(punkitem: ::windows::runtime::RawPtr, freadwrite: super::super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandlerWithCreateObject(punkitem: ::windows::runtime::RawPtr, freadwrite: super::super::super::Foundation::BOOL, punkcreateobject: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY, ppszcanonicalname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetNamedPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionByName(pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionListFromString(pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyKeyFromName(pszname: super::super::super::Foundation::PWSTR, ppropkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSGetPropertySystem(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSLookupPropertyHandlerCLSID(pszfilepath: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_Delete(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBOOL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBSTR(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadDWORD(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadGUID(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadInt(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTS(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::POINTS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPropertyKey(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadRECTL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::RECTL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadSHORT(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStr(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, charactercount: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStrAlloc(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStream(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PSPropertyBag_ReadType(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, var: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, r#type: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadULONGLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadUnknown(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBOOL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBSTR(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteDWORD(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteGUID(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteInt(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTS(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::POINTS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePropertyKey(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const PROPERTYKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteRECTL(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: *const super::super::super::Foundation::RECTL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteSHORT(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStr(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStream(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteULONGLONG(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, value: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteUnknown(propbag: ::windows::runtime::RawPtr, propname: super::super::super::Foundation::PWSTR, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSPropertyKeyFromString(pszstring: super::super::super::Foundation::PWSTR, pkey: *mut PROPERTYKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSRefreshPropertySchema() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSRegisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSSetPropertyValue(pps: ::windows::runtime::RawPtr, ppd: ::windows::runtime::RawPtr, propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSUnregisterPropertySchema(pszpath: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_CloseProperties(hprops: super::super::super::Foundation::HANDLE, flopt: u32) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_GetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *mut ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_OpenProperties(pszapp: super::super::super::Foundation::PWSTR, pszpif: super::super::super::Foundation::PWSTR, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_SetProperties(hprops: super::super::super::Foundation::HANDLE, pszgroup: super::super::super::Foundation::PSTR, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantChangeType(ppropvardest: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvarsrc: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, flags: PROPVAR_CHANGE_FLAGS, vt: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantCompareEx(propvar1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, unit: PROPVAR_COMPARE_UNIT, flags: PROPVAR_COMPARE_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetBooleanElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetDoubleElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetElementCount(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetFileTimeElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pftval: *mut super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt16Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt32Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt64Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetStringElem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt16Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt32Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt64Elem(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBSTR(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBoolean(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgf: *mut super::super::super::Foundation::BOOL, crgf: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBuffer(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDouble(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdblret: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, dbldefault: f64) -> f64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTime(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgft: *mut super::super::super::Foundation::FILETIME, crgft: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgft: *mut *mut super::super::super::Foundation::FILETIME, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToGUID(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, piret: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, idefault: i16) -> i16;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, plret: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ldefault: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pllret: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, lldefault: i64) -> i64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn PropVariantToStrRet(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstrret: *mut super::Common::STRRET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToString(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, psz: super::super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszout: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringWithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, puiret: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, uidefault: u16) -> u16;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pulret: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, uldefault: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pullret: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64Vector(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64VectorAlloc(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64WithDefault(propvarin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ulldefault: u64) -> u64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PropVariantToVariant(ppropvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToWinRTPropertyValue(propvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAddDefaultPropertiesByExt(pszext: super::super::super::Foundation::PWSTR, ppropstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetPropertyStoreForWindow(hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHGetPropertyStoreFromParsingName(pszpath: super::super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn SHPropStgCreate(psstg: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::windows::runtime::RawPtr, pucodepage: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgReadMultiple(pps: ::windows::runtime::RawPtr, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgWriteMultiple(pps: ::windows::runtime::RawPtr, pucodepage: *mut u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propidnamefirst: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantCompare(var1: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetBooleanElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pfval: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetDoubleElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetElementCount(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt16Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt32Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt64Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetStringElem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, ppszval: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt16Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt32Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt64Elem(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ielem: u32, pnval: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBoolean(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pfret: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgf: *mut super::super::super::Foundation::BOOL, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgf: *mut *mut super::super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, fdefault: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBuffer(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDosDateTime(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pwdate: *mut u16, pwtime: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDouble(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pdblret: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut f64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, dbldefault: f64) -> f64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToFileTime(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, stfout: PSTIME_FLAGS, pftout: *mut super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToGUID(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, piret: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, idefault: i16) -> i16;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, plret: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ldefault: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pllret: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut i64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, lldefault: i64) -> i64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn VariantToPropVariant(pvar: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn VariantToStrRet(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pstrret: *mut super::Common::STRRET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToString(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pszbuf: super::super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringAlloc(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppszbuf: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArray(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgsz: *mut super::super::super::Foundation::PWSTR, crgsz: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgsz: *mut *mut super::super::super::Foundation::PWSTR, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringWithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pszdefault: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, puiret: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u16, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, uidefault: u16) -> u16;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pulret: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u32, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, uldefault: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pullret: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64Array(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, prgn: *mut u64, crgn: u32, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64ArrayAlloc(var: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64WithDefault(varin: *const ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ulldefault: u64) -> u64;
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn WinRTPropertyValueToPropVariant(punkpropertyvalue: ::windows::runtime::RawPtr, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
}
