#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSCoerceToCanonicalValue(key : *const super::wtypes::PROPERTYKEY, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSCreateAdapterFromPropertyStore(pps : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSCreateDelayedMultiplexPropertyStore(flags : GETPROPERTYSTOREFLAGS, pdpsf : *mut core::ffi::c_void, rgstoreids : *const u32, cstores : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSCreateMemoryPropertyStore(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSCreateMultiplexPropertyStore(prgpunkstores : *const *mut core::ffi::c_void, cstores : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSCreatePropertyChangeArray(rgpropkey : *const super::wtypes::PROPERTYKEY, rgflags : *const PKA_FLAGS, rgpropvar : *const super::propidlbase::PROPVARIANT, cchanges : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromObject(punk : *mut core::ffi::c_void, grfmode : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_propidlbase")]
windows_link::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromPropertySetStorage(ppss : *mut core::ffi::c_void, grfmode : u32, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSCreateSimplePropertyChange(flags : PKA_FLAGS, key : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSEnumeratePropertyDescriptions(filteron : PROPDESC_ENUMFILTER, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSFormatForDisplay(propkey : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, pdfflags : PROPDESC_FORMAT_FLAGS, pwsztext : windows_sys::core::PWSTR, cchtext : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSFormatForDisplayAlloc(key : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSFormatPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSGetImageReferenceForValue(propkey : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, ppszimageres : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSGetItemPropertyHandler(punkitem : *mut core::ffi::c_void, freadwrite : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSGetItemPropertyHandlerWithCreateObject(punkitem : *mut core::ffi::c_void, freadwrite : windows_sys::core::BOOL, punkcreateobject : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("propsys.dll" "system" fn PSGetNameFromPropertyKey(propkey : *const super::wtypes::PROPERTYKEY, ppszcanonicalname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSGetNamedPropertyFromPropertyStorage(psps : *const SERIALIZEDPROPSTORAGE, cb : u32, pszname : windows_sys::core::PCWSTR, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("propsys.dll" "system" fn PSGetPropertyDescription(propkey : *const super::wtypes::PROPERTYKEY, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSGetPropertyDescriptionByName(pszcanonicalname : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSGetPropertyDescriptionListFromString(pszproplist : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSGetPropertyFromPropertyStorage(psps : *const SERIALIZEDPROPSTORAGE, cb : u32, rpkey : *const super::wtypes::PROPERTYKEY, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("propsys.dll" "system" fn PSGetPropertyKeyFromName(pszname : windows_sys::core::PCWSTR, ppropkey : *mut super::wtypes::PROPERTYKEY) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSGetPropertySystem(riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSGetPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSLookupPropertyHandlerCLSID(pszfilepath : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_Delete(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadBOOL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadBSTR(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadDWORD(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadGUID(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadInt(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadLONG(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut super::windef::POINTL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTS(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut super::windef::POINTS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadPropertyKey(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut super::wtypes::PROPERTYKEY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadRECTL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut super::windef::RECTL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadSHORT(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadStr(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : windows_sys::core::PWSTR, charactercount : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadStrAlloc(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadStream(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadType(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, var : *mut super::oaidl::VARIANT, r#type : super::wtypes::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadULONGLONG(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_ReadUnknown(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteBOOL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteBSTR(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteDWORD(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteGUID(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteInt(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteLONG(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *const super::windef::POINTL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTS(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *const super::windef::POINTS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WritePropertyKey(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *const super::wtypes::PROPERTYKEY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_windef"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteRECTL(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *const super::windef::RECTL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteSHORT(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteStr(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase"))]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteStream(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteULONGLONG(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, value : u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("propsys.dll" "system" fn PSPropertyBag_WriteUnknown(propbag : *mut core::ffi::c_void, propname : windows_sys::core::PCWSTR, punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("propsys.dll" "system" fn PSPropertyKeyFromString(pszstring : windows_sys::core::PCWSTR, pkey : *mut super::wtypes::PROPERTYKEY) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSRefreshPropertySchema() -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSRegisterPropertySchema(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PSSetPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, propvar : *const super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("propsys.dll" "system" fn PSStringFromPropertyKey(pkey : *const super::wtypes::PROPERTYKEY, psz : windows_sys::core::PWSTR, cch : u32) -> windows_sys::core::HRESULT);
windows_link::link!("propsys.dll" "system" fn PSUnregisterPropertySchema(pszpath : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn PropVariantToWinRTPropertyValue(propvar : *const super::propidlbase::PROPVARIANT, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("propsys.dll" "system" fn WinRTPropertyValueToPropVariant(punkpropertyvalue : *mut core::ffi::c_void, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_sys::core::HRESULT);
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = 0;
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = 1;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = 2;
pub type GETPROPERTYSTOREFLAGS = u32;
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = 64;
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = 0;
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = 32;
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = 512;
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1024;
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 8;
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 1;
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = 8191;
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = 128;
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = 16;
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = 256;
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = 2;
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = 4;
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = 2048;
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = 4096;
pub const GUIDSTRING_MAX: u32 = 39;
pub const InMemoryPropertyStore: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
pub type PCUSERIALIZEDPROPSTORAGE = *const SERIALIZEDPROPSTORAGE;
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = 3;
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = 4;
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = 0;
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = 1;
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = 6;
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = 7;
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = 2;
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = 5;
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = 2;
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = 0;
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = 3;
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = 1;
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = 4;
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = 5;
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = 4;
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = 3;
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = 0;
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = 5;
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = 2;
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = 1;
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = 2;
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = 3;
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = 4;
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = 1;
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = 0;
pub const PDEF_ALL: PROPDESC_ENUMFILTER = 0;
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = 6;
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = 5;
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = 2;
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = 4;
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = 1;
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = 3;
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = 4;
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = 0;
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = 2;
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = 512;
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = 64;
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = 256;
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = 32;
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = 8192;
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = 1;
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = 4096;
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = 1024;
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = 8;
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = 128;
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = 16;
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = 2048;
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = 1;
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = 4;
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = 0;
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = 3;
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = 6;
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = 5;
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = 2;
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = 3;
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 1;
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 6;
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = 0;
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = 5;
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = 10;
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 8;
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = 9;
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = 4;
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = 2;
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = 7;
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = 1;
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = 0;
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = 2;
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = 4;
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = 3;
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = 8;
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = 0;
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = 1;
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = 2;
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = 4;
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = 16;
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = 4096;
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = 512;
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = 8;
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = 16;
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = 0;
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = 2048;
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = 64;
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = 4;
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = 2;
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = 256;
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = 2147483648;
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = 32;
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = 128;
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = 2147491839;
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = 1;
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = 1024;
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = 4;
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = 4096;
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = 1;
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = 0;
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = 8;
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = 2048;
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = 512;
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = 7167;
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = 2;
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = 64;
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = 128;
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = 256;
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = 32;
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = 16;
pub type PERSIST_SPROPSTORE_FLAGS = i32;
pub const PET_DEFAULTVALUE: PROPENUMTYPE = 2;
pub const PET_DISCRETEVALUE: PROPENUMTYPE = 0;
pub const PET_ENDRANGE: PROPENUMTYPE = 3;
pub const PET_RANGEDVALUE: PROPENUMTYPE = 1;
pub const PKA_APPEND: PKA_FLAGS = 1;
pub const PKA_DELETE: PKA_FLAGS = 2;
pub type PKA_FLAGS = i32;
pub const PKA_SET: PKA_FLAGS = 0;
pub const PKEYSTR_MAX: u32 = 50;
pub const PKEY_PIDSTR_MAX: u32 = 10;
pub type PROPDESC_AGGREGATION_TYPE = i32;
pub type PROPDESC_COLUMNINDEX_TYPE = i32;
pub type PROPDESC_CONDITION_TYPE = i32;
pub type PROPDESC_DISPLAYTYPE = i32;
pub type PROPDESC_ENUMFILTER = i32;
pub type PROPDESC_FORMAT_FLAGS = u32;
pub type PROPDESC_GROUPING_RANGE = i32;
pub type PROPDESC_RELATIVEDESCRIPTION_TYPE = i32;
pub type PROPDESC_SEARCHINFO_FLAGS = u32;
pub type PROPDESC_SORTDESCRIPTION = i32;
pub type PROPDESC_TYPE_FLAGS = u32;
pub type PROPDESC_VIEW_FLAGS = u32;
pub type PROPENUMTYPE = i32;
pub const PSC_DIRTY: PSC_STATE = 2;
pub const PSC_NORMAL: PSC_STATE = 0;
pub const PSC_NOTINSOURCE: PSC_STATE = 1;
pub const PSC_READONLY: PSC_STATE = 3;
pub type PSC_STATE = i32;
pub type PUSERIALIZEDPROPSTORAGE = *mut SERIALIZEDPROPSTORAGE;
pub const PropertySystem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
pub type _PERSIST_SPROPSTORE_FLAGS = i32;
