#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSCoerceToCanonicalValue(key : *const PROPERTYKEY, ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    PSCoerceToCanonicalValue(key, ppropvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<P0>(pps: P0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IPropertyStore>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSCreateAdapterFromPropertyStore(pps : * mut::core::ffi::c_void, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreateAdapterFromPropertyStore(pps.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<P0>(flags: GETPROPERTYSTOREFLAGS, pdpsf: P0, rgstoreids: &[u32], riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IDelayedPropertyStoreFactory>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSCreateDelayedMultiplexPropertyStore(flags : GETPROPERTYSTOREFLAGS, pdpsf : * mut::core::ffi::c_void, rgstoreids : *const u32, cstores : u32, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreateDelayedMultiplexPropertyStore(flags, pdpsf.into_param().abi(), ::core::mem::transmute(rgstoreids.as_ptr()), rgstoreids.len() as _, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSCreateMemoryPropertyStore(riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreateMemoryPropertyStore(riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore(prgpunkstores: &[::core::option::Option<::windows_core::IUnknown>], riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSCreateMultiplexPropertyStore(prgpunkstores : *const * mut::core::ffi::c_void, cstores : u32, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreateMultiplexPropertyStore(::core::mem::transmute(prgpunkstores.as_ptr()), prgpunkstores.len() as _, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray(rgpropkey: ::core::option::Option<*const PROPERTYKEY>, rgflags: ::core::option::Option<*const PKA_FLAGS>, rgpropvar: ::core::option::Option<*const super::super::super::System::Com::StructuredStorage::PROPVARIANT>, cchanges: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyChangeArray(rgpropkey : *const PROPERTYKEY, rgflags : *const PKA_FLAGS, rgpropvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT, cchanges : u32, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreatePropertyChangeArray(::core::mem::transmute(rgpropkey.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgflags.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgpropvar.unwrap_or(::std::ptr::null())), cchanges, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<P0>(punk: P0, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromObject(punk : * mut::core::ffi::c_void, grfmode : u32, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreatePropertyStoreFromObject(punk.into_param().abi(), grfmode, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<P0>(ppss: P0, grfmode: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromPropertySetStorage(ppss : * mut::core::ffi::c_void, grfmode : u32, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreatePropertyStoreFromPropertySetStorage(ppss.into_param().abi(), grfmode, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange(flags: PKA_FLAGS, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSCreateSimplePropertyChange(flags : PKA_FLAGS, key : *const PROPERTYKEY, propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSCreateSimplePropertyChange(flags, key, propvar, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions(filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSEnumeratePropertyDescriptions(filteron : PROPDESC_ENUMFILTER, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSEnumeratePropertyDescriptions(filteron, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSFormatForDisplay(propkey : *const PROPERTYKEY, propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT, pdfflags : PROPDESC_FORMAT_FLAGS, pwsztext : ::windows_core::PWSTR, cchtext : u32) -> ::windows_core::HRESULT);
    PSFormatForDisplay(propkey, propvar, pdfflags, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PSFormatForDisplayAlloc(key : *const PROPERTYKEY, propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSFormatForDisplayAlloc(key, propvar, pdff, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSFormatPropertyValue<P0, P1>(pps: P0, ppd: P1, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<IPropertyStore>,
    P1: ::windows_core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSFormatPropertyValue(pps : * mut::core::ffi::c_void, ppd : * mut::core::ffi::c_void, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSFormatPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), pdff, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PSGetImageReferenceForValue(propkey : *const PROPERTYKEY, propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT, ppszimageres : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSGetImageReferenceForValue(propkey, propvar, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandler<P0, P1>(punkitem: P0, freadwrite: P1, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetItemPropertyHandler(punkitem : * mut::core::ffi::c_void, freadwrite : super::super::super::Foundation:: BOOL, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetItemPropertyHandler(punkitem.into_param().abi(), freadwrite.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<P0, P1, P2>(punkitem: P0, freadwrite: P1, punkcreateobject: P2, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    P2: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetItemPropertyHandlerWithCreateObject(punkitem : * mut::core::ffi::c_void, freadwrite : super::super::super::Foundation:: BOOL, punkcreateobject : * mut::core::ffi::c_void, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetItemPropertyHandlerWithCreateObject(punkitem.into_param().abi(), freadwrite.into_param().abi(), punkcreateobject.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const PROPERTYKEY) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn PSGetNameFromPropertyKey(propkey : *const PROPERTYKEY, ppszcanonicalname : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSGetNameFromPropertyKey(propkey, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<P0, P1>(psps: P0, cb: u32, pszname: P1) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<PCUSERIALIZEDPROPSTORAGE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetNamedPropertyFromPropertyStorage(psps : PCUSERIALIZEDPROPSTORAGE, cb : u32, pszname : ::windows_core::PCWSTR, ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSGetNamedPropertyFromPropertyStorage(psps.into_param().abi(), cb, pszname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescription(propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescription(propkey : *const PROPERTYKEY, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetPropertyDescription(propkey, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<P0>(pszcanonicalname: P0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescriptionByName(pszcanonicalname : ::windows_core::PCWSTR, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetPropertyDescriptionByName(pszcanonicalname.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<P0>(pszproplist: P0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyDescriptionListFromString(pszproplist : ::windows_core::PCWSTR, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetPropertyDescriptionListFromString(pszproplist.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage<P0>(psps: P0, cb: u32, rpkey: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<PCUSERIALIZEDPROPSTORAGE>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyFromPropertyStorage(psps : PCUSERIALIZEDPROPSTORAGE, cb : u32, rpkey : *const PROPERTYKEY, ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSGetPropertyFromPropertyStorage(psps.into_param().abi(), cb, rpkey, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<P0>(pszname: P0, ppropkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyKeyFromName(pszname : ::windows_core::PCWSTR, ppropkey : *mut PROPERTYKEY) -> ::windows_core::HRESULT);
    PSGetPropertyKeyFromName(pszname.into_param().abi(), ppropkey).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSGetPropertySystem(riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertySystem(riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSGetPropertySystem(riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSGetPropertyValue<P0, P1>(pps: P0, ppd: P1) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
where
    P0: ::windows_core::IntoParam<IPropertyStore>,
    P1: ::windows_core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSGetPropertyValue(pps : * mut::core::ffi::c_void, ppd : * mut::core::ffi::c_void, ppropvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSGetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<P0>(pszfilepath: P0) -> ::windows_core::Result<::windows_core::GUID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSLookupPropertyHandlerCLSID(pszfilepath : ::windows_core::PCWSTR, pclsid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSLookupPropertyHandlerCLSID(pszfilepath.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_Delete<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_Delete(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    PSPropertyBag_Delete(propbag.into_param().abi(), propname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<super::super::super::Foundation::BOOL>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadBOOL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadBOOL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<::windows_core::BSTR>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadBSTR(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadBSTR(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<u32>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadDWORD(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadDWORD(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<::windows_core::GUID>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadGUID(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadGUID(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<i32>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadInt(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadInt(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<i32>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadLONG(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<super::super::super::Foundation::POINTL>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut super::super::super::Foundation:: POINTL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadPOINTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<super::super::super::Foundation::POINTS>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTS(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut super::super::super::Foundation:: POINTS) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadPOINTS(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<P0, P1>(propbag: P0, propname: P1, value: *mut PROPERTYKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadPropertyKey(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut PROPERTYKEY) -> ::windows_core::HRESULT);
    PSPropertyBag_ReadPropertyKey(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<super::super::super::Foundation::RECTL>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadRECTL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut super::super::super::Foundation:: RECTL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadRECTL(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<i16>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadSHORT(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut i16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadSHORT(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<P0, P1>(propbag: P0, propname: P1, value: &mut [u16]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStr(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : ::windows_core::PWSTR, charactercount : i32) -> ::windows_core::HRESULT);
    PSPropertyBag_ReadStr(propbag.into_param().abi(), propname.into_param().abi(), ::core::mem::transmute(value.as_ptr()), value.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStrAlloc(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadStrAlloc(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<super::super::super::System::Com::IStream>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadStream(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadStream(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<P0, P1>(propbag: P0, propname: P1, var: *mut super::super::super::System::Variant::VARIANT, r#type: super::super::super::System::Variant::VARENUM) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadType(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, var : *mut super::super::super::System::Variant:: VARIANT, r#type : super::super::super::System::Variant:: VARENUM) -> ::windows_core::HRESULT);
    PSPropertyBag_ReadType(propbag.into_param().abi(), propname.into_param().abi(), var, r#type).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<P0, P1>(propbag: P0, propname: P1) -> ::windows_core::Result<u64>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadULONGLONG(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PSPropertyBag_ReadULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<P0, P1>(propbag: P0, propname: P1, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_ReadUnknown(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSPropertyBag_ReadUnknown(propbag.into_param().abi(), propname.into_param().abi(), riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteBOOL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : super::super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteBOOL(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::BSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteBSTR(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteBSTR(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<P0, P1>(propbag: P0, propname: P1, value: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteDWORD(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : u32) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteDWORD(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<P0, P1>(propbag: P0, propname: P1, value: *const ::windows_core::GUID) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteGUID(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *const ::windows_core::GUID) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteGUID(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<P0, P1>(propbag: P0, propname: P1, value: i32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteInt(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : i32) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteInt(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<P0, P1>(propbag: P0, propname: P1, value: i32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteLONG(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : i32) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteLONG(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTL) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *const super::super::super::Foundation:: POINTL) -> ::windows_core::HRESULT);
    PSPropertyBag_WritePOINTL(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::POINTS) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTS(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *const super::super::super::Foundation:: POINTS) -> ::windows_core::HRESULT);
    PSPropertyBag_WritePOINTS(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<P0, P1>(propbag: P0, propname: P1, value: *const PROPERTYKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WritePropertyKey(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *const PROPERTYKEY) -> ::windows_core::HRESULT);
    PSPropertyBag_WritePropertyKey(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<P0, P1>(propbag: P0, propname: P1, value: *const super::super::super::Foundation::RECTL) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteRECTL(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : *const super::super::super::Foundation:: RECTL) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteRECTL(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<P0, P1>(propbag: P0, propname: P1, value: i16) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteSHORT(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : i16) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteSHORT(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteStr(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteStr(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteStream(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteStream(propbag.into_param().abi(), propname.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<P0, P1>(propbag: P0, propname: P1, value: u64) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteULONGLONG(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, value : u64) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteULONGLONG(propbag.into_param().abi(), propname.into_param().abi(), value).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<P0, P1, P2>(propbag: P0, propname: P1, punk: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyBag>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyBag_WriteUnknown(propbag : * mut::core::ffi::c_void, propname : ::windows_core::PCWSTR, punk : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    PSPropertyBag_WriteUnknown(propbag.into_param().abi(), propname.into_param().abi(), punk.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSPropertyKeyFromString<P0>(pszstring: P0, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSPropertyKeyFromString(pszstring : ::windows_core::PCWSTR, pkey : *mut PROPERTYKEY) -> ::windows_core::HRESULT);
    PSPropertyKeyFromString(pszstring.into_param().abi(), pkey).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSRefreshPropertySchema() -> ::windows_core::HRESULT);
    PSRefreshPropertySchema().ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSRegisterPropertySchema<P0>(pszpath: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSRegisterPropertySchema(pszpath : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    PSRegisterPropertySchema(pszpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PSSetPropertyValue<P0, P1>(pps: P0, ppd: P1, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IPropertyStore>,
    P1: ::windows_core::IntoParam<IPropertyDescription>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSSetPropertyValue(pps : * mut::core::ffi::c_void, ppd : * mut::core::ffi::c_void, propvar : *const super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    PSSetPropertyValue(pps.into_param().abi(), ppd.into_param().abi(), propvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const PROPERTYKEY, psz: &mut [u16]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn PSStringFromPropertyKey(pkey : *const PROPERTYKEY, psz : ::windows_core::PWSTR, cch : u32) -> ::windows_core::HRESULT);
    PSStringFromPropertyKey(pkey, ::core::mem::transmute(psz.as_ptr()), psz.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn PSUnregisterPropertySchema<P0>(pszpath: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn PSUnregisterPropertySchema(pszpath : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    PSUnregisterPropertySchema(pszpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_CloseProperties<P0>(hprops: P0, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("shell32.dll" "system" fn PifMgr_CloseProperties(hprops : super::super::super::Foundation:: HANDLE, flopt : u32) -> super::super::super::Foundation:: HANDLE);
    PifMgr_CloseProperties(hprops.into_param().abi(), flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_GetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: ::core::option::Option<*mut ::core::ffi::c_void>, cbprops: i32, flopt: u32) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("shell32.dll" "system" fn PifMgr_GetProperties(hprops : super::super::super::Foundation:: HANDLE, pszgroup : ::windows_core::PCSTR, lpprops : *mut ::core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    PifMgr_GetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), ::core::mem::transmute(lpprops.unwrap_or(::std::ptr::null_mut())), cbprops, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_OpenProperties<P0, P1>(pszapp: P0, pszpif: P1, hinf: u32, flopt: u32) -> super::super::super::Foundation::HANDLE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("shell32.dll" "system" fn PifMgr_OpenProperties(pszapp : ::windows_core::PCWSTR, pszpif : ::windows_core::PCWSTR, hinf : u32, flopt : u32) -> super::super::super::Foundation:: HANDLE);
    PifMgr_OpenProperties(pszapp.into_param().abi(), pszpif.into_param().abi(), hinf, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PifMgr_SetProperties<P0, P1>(hprops: P0, pszgroup: P1, lpprops: *const ::core::ffi::c_void, cbprops: i32, flopt: u32) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("shell32.dll" "system" fn PifMgr_SetProperties(hprops : super::super::super::Foundation:: HANDLE, pszgroup : ::windows_core::PCSTR, lpprops : *const ::core::ffi::c_void, cbprops : i32, flopt : u32) -> i32);
    PifMgr_SetProperties(hprops.into_param().abi(), pszgroup.into_param().abi(), lpprops, cbprops, flopt)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[inline]
pub unsafe fn SHAddDefaultPropertiesByExt<P0, P1>(pszext: P0, ppropstore: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<IPropertyStore>,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHAddDefaultPropertiesByExt(pszext : ::windows_core::PCWSTR, ppropstore : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    SHAddDefaultPropertiesByExt(pszext.into_param().abi(), ppropstore.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SHGetPropertyStoreForWindow<P0, T>(hwnd: P0) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreForWindow(hwnd : super::super::super::Foundation:: HWND, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    SHGetPropertyStoreForWindow(hwnd.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_UI_Shell_Common\"`*"]
#[cfg(feature = "Win32_UI_Shell_Common")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromIDList(pidl: *const super::Common::ITEMIDLIST, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreFromIDList(pidl : *const super::Common:: ITEMIDLIST, flags : GETPROPERTYSTOREFLAGS, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    SHGetPropertyStoreFromIDList(pidl, flags, riid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SHGetPropertyStoreFromParsingName<P0, P1, T>(pszpath: P0, pbc: P1, flags: GETPROPERTYSTOREFLAGS) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IBindCtx>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHGetPropertyStoreFromParsingName(pszpath : ::windows_core::PCWSTR, pbc : * mut::core::ffi::c_void, flags : GETPROPERTYSTOREFLAGS, riid : *const ::windows_core::GUID, ppv : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    SHGetPropertyStoreFromParsingName(pszpath.into_param().abi(), pbc.into_param().abi(), flags, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn SHPropStgCreate<P0>(psstg: P0, fmtid: *const ::windows_core::GUID, pclsid: ::core::option::Option<*const ::windows_core::GUID>, grfflags: u32, grfmode: u32, dwdisposition: u32, ppstg: *mut ::core::option::Option<super::super::super::System::Com::StructuredStorage::IPropertyStorage>, pucodepage: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertySetStorage>,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHPropStgCreate(psstg : * mut::core::ffi::c_void, fmtid : *const ::windows_core::GUID, pclsid : *const ::windows_core::GUID, grfflags : u32, grfmode : u32, dwdisposition : u32, ppstg : *mut * mut::core::ffi::c_void, pucodepage : *mut u32) -> ::windows_core::HRESULT);
    SHPropStgCreate(psstg.into_param().abi(), fmtid, ::core::mem::transmute(pclsid.unwrap_or(::std::ptr::null())), grfflags, grfmode, dwdisposition, ::core::mem::transmute(ppstg), ::core::mem::transmute(pucodepage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SHPropStgReadMultiple<P0>(pps: P0, ucodepage: u32, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHPropStgReadMultiple(pps : * mut::core::ffi::c_void, ucodepage : u32, cpspec : u32, rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC, rgvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT) -> ::windows_core::HRESULT);
    SHPropStgReadMultiple(pps.into_param().abi(), ucodepage, cpspec, rgpspec, rgvar).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SHPropStgWriteMultiple<P0>(pps: P0, pucodepage: ::core::option::Option<*mut u32>, cpspec: u32, rgpspec: *const super::super::super::System::Com::StructuredStorage::PROPSPEC, rgvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IPropertyStorage>,
{
    ::windows_targets::link!("shell32.dll" "system" fn SHPropStgWriteMultiple(pps : * mut::core::ffi::c_void, pucodepage : *mut u32, cpspec : u32, rgpspec : *const super::super::super::System::Com::StructuredStorage:: PROPSPEC, rgvar : *mut super::super::super::System::Com::StructuredStorage:: PROPVARIANT, propidnamefirst : u32) -> ::windows_core::HRESULT);
    SHPropStgWriteMultiple(pps.into_param().abi(), ::core::mem::transmute(pucodepage.unwrap_or(::std::ptr::null_mut())), cpspec, rgpspec, rgvar, propidnamefirst).ok()
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct ICreateObject(::windows_core::IUnknown);
impl ICreateObject {
    pub unsafe fn CreateObject<P0, T>(&self, clsid: *const ::windows_core::GUID, punkouter: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateObject)(::windows_core::Interface::as_raw(self), clsid, punkouter.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ICreateObject, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for ICreateObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateObject {}
impl ::core::fmt::Debug for ICreateObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateObject {
    type Vtable = ICreateObject_Vtbl;
}
impl ::core::clone::Clone for ICreateObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICreateObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75121952_e0d0_43e5_9380_1d80483acf72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IDelayedPropertyStoreFactory(::windows_core::IUnknown);
impl IDelayedPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P0, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetPropertyStore)(::windows_core::Interface::as_raw(self), flags, punkfactory.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetPropertyStoreForKeys)(::windows_core::Interface::as_raw(self), rgkeys, ckeys, flags, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDelayedPropertyStore<T>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetDelayedPropertyStore)(::windows_core::Interface::as_raw(self), flags, dwstoreid, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDelayedPropertyStoreFactory, ::windows_core::IUnknown, IPropertyStoreFactory);
impl ::core::cmp::PartialEq for IDelayedPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDelayedPropertyStoreFactory {}
impl ::core::fmt::Debug for IDelayedPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDelayedPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDelayedPropertyStoreFactory {
    type Vtable = IDelayedPropertyStoreFactory_Vtbl;
}
impl ::core::clone::Clone for IDelayedPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDelayedPropertyStoreFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40d4577f_e237_4bdb_bd69_58f089431b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_Vtbl {
    pub base__: IPropertyStoreFactory_Vtbl,
    pub GetDelayedPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IInitializeWithFile(::windows_core::IUnknown);
impl IInitializeWithFile {
    pub unsafe fn Initialize<P0>(&self, pszfilepath: P0, grfmode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pszfilepath.into_param().abi(), grfmode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInitializeWithFile, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IInitializeWithFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithFile {}
impl ::core::fmt::Debug for IInitializeWithFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithFile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitializeWithFile {
    type Vtable = IInitializeWithFile_Vtbl;
}
impl ::core::clone::Clone for IInitializeWithFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInitializeWithFile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7d14566_0509_4cce_a71f_0a554233bd9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IInitializeWithStream(::windows_core::IUnknown);
impl IInitializeWithStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pstream: P0, grfmode: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), grfmode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IInitializeWithStream, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IInitializeWithStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithStream {}
impl ::core::fmt::Debug for IInitializeWithStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithStream").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInitializeWithStream {
    type Vtable = IInitializeWithStream_Vtbl;
}
impl ::core::clone::Clone for IInitializeWithStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInitializeWithStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct INamedPropertyStore(::windows_core::IUnknown);
impl INamedPropertyStore {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetNamedValue<P0>(&self, pszname: P0) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNamedValue)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetNamedValue<P0>(&self, pszname: P0, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNamedValue)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), propvar).ok()
    }
    pub unsafe fn GetNameCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNameCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNameAt(&self, iprop: u32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNameAt)(::windows_core::Interface::as_raw(self), iprop, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(INamedPropertyStore, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for INamedPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedPropertyStore {}
impl ::core::fmt::Debug for INamedPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for INamedPropertyStore {
    type Vtable = INamedPropertyStore_Vtbl;
}
impl ::core::clone::Clone for INamedPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INamedPropertyStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71604b0f_97b0_4764_8577_2f13e98a1422);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetNamedValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetNamedValue: usize,
    pub GetNameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetNameAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IObjectWithPropertyKey(::windows_core::IUnknown);
impl IObjectWithPropertyKey {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyKey)(::windows_core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IObjectWithPropertyKey, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IObjectWithPropertyKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithPropertyKey {}
impl ::core::fmt::Debug for IObjectWithPropertyKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithPropertyKey").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectWithPropertyKey {
    type Vtable = IObjectWithPropertyKey_Vtbl;
}
impl ::core::clone::Clone for IObjectWithPropertyKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IObjectWithPropertyKey {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPersistSerializedPropStorage(::windows_core::IUnknown);
impl IPersistSerializedPropStorage {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetPropertyStorage<P0>(&self, psps: P0, cb: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PCUSERIALIZEDPROPSTORAGE>,
    {
        (::windows_core::Interface::vtable(self).SetPropertyStorage)(::windows_core::Interface::as_raw(self), psps.into_param().abi(), cb).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyStorage)(::windows_core::Interface::as_raw(self), ppsps, pcb).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistSerializedPropStorage {
    type Vtable = IPersistSerializedPropStorage_Vtbl;
}
impl ::core::clone::Clone for IPersistSerializedPropStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPersistSerializedPropStorage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe318ad57_0aa0_450f_aca5_6fab7103d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub SetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::HRESULT,
    pub GetPropertyStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPersistSerializedPropStorage2(::windows_core::IUnknown);
impl IPersistSerializedPropStorage2 {
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetPropertyStorage<P0>(&self, psps: P0, cb: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PCUSERIALIZEDPROPSTORAGE>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPropertyStorage)(::windows_core::Interface::as_raw(self), psps.into_param().abi(), cb).ok()
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyStorage)(::windows_core::Interface::as_raw(self), ppsps, pcb).ok()
    }
    pub unsafe fn GetPropertyStorageSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyStorageSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyStorageBuffer)(::windows_core::Interface::as_raw(self), psps, cb, pcbwritten).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage2, ::windows_core::IUnknown, IPersistSerializedPropStorage);
impl ::core::cmp::PartialEq for IPersistSerializedPropStorage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistSerializedPropStorage2 {}
impl ::core::fmt::Debug for IPersistSerializedPropStorage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistSerializedPropStorage2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPersistSerializedPropStorage2 {
    type Vtable = IPersistSerializedPropStorage2_Vtbl;
}
impl ::core::clone::Clone for IPersistSerializedPropStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPersistSerializedPropStorage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77effa68_4f98_4366_ba72_573b3d880571);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_Vtbl {
    pub base__: IPersistSerializedPropStorage_Vtbl,
    pub GetPropertyStorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropertyStorageBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyChange(::windows_core::IUnknown);
impl IPropertyChange {
    pub unsafe fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPropertyKey)(::windows_core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ApplyToPropVariant)(::windows_core::Interface::as_raw(self), propvarin, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyChange, ::windows_core::IUnknown, IObjectWithPropertyKey);
impl ::core::cmp::PartialEq for IPropertyChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChange {}
impl ::core::fmt::Debug for IPropertyChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyChange {
    type Vtable = IPropertyChange_Vtbl;
}
impl ::core::clone::Clone for IPropertyChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf917bc8a_1bba_4478_a245_1bde03eb9431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_Vtbl {
    pub base__: IObjectWithPropertyKey_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub ApplyToPropVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    ApplyToPropVariant: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyChangeArray(::windows_core::IUnknown);
impl IPropertyChangeArray {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, iindex: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), iindex, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, iindex: u32, ppropchange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IPropertyChange>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), iindex, ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, ppropchange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IPropertyChange>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn AppendOrReplace<P0>(&self, ppropchange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IPropertyChange>,
    {
        (::windows_core::Interface::vtable(self).AppendOrReplace)(::windows_core::Interface::as_raw(self), ppropchange.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, iindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), iindex).ok()
    }
    pub unsafe fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsKeyInArray)(::windows_core::Interface::as_raw(self), key).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyChangeArray, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyChangeArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyChangeArray {}
impl ::core::fmt::Debug for IPropertyChangeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyChangeArray").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyChangeArray {
    type Vtable = IPropertyChangeArray_Vtbl;
}
impl ::core::clone::Clone for IPropertyChangeArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyChangeArray {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x380f5cad_1b5e_42f2_805d_637fd392d31e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppendOrReplace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows_core::HRESULT,
    pub IsKeyInArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescription(::windows_core::IUnknown);
impl IPropertyDescription {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCanonicalName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEditInvitation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTypeFlags)(::windows_core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetViewFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGroupingRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRelativeDescription)(::windows_core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAggregationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConditionType)(::windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsValueCanonical)(::windows_core::Interface::as_raw(self), propvar).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescription, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription {}
impl ::core::fmt::Debug for IPropertyDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescription {
    type Vtable = IPropertyDescription_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f79d558_3e96_4549_a1d1_7d75d2288814);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPropertyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEditInvitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszinvite: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows_core::HRESULT,
    pub GetViewFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows_core::HRESULT,
    pub GetDefaultColumnWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows_core::HRESULT,
    pub GetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows_core::HRESULT,
    pub GetColumnState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetGroupingRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows_core::HRESULT,
    pub GetRelativeDescriptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetRelativeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetRelativeDescription: usize,
    pub GetSortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSortDescriptionLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSortDescriptionLabel: usize,
    pub GetAggregationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Search_Common")]
    pub GetConditionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Search_Common"))]
    GetConditionType: usize,
    pub GetEnumTypeList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub CoerceToCanonicalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    CoerceToCanonicalValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub IsValueCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    IsValueCanonical: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescription2(::windows_core::IUnknown);
impl IPropertyDescription2 {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), propvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImageReferenceForValue)(::windows_core::Interface::as_raw(self), propvar, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescription2, ::windows_core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescription2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescription2 {}
impl ::core::fmt::Debug for IPropertyDescription2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescription2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescription2 {
    type Vtable = IPropertyDescription2_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescription2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescription2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d2eded_5062_400e_b107_5dae79fe57a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetImageReferenceForValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetImageReferenceForValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionAliasInfo(::windows_core::IUnknown);
impl IPropertyDescriptionAliasInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetSortByAlias<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetSortByAlias)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdditionalSortByAliases<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetAdditionalSortByAliases)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescriptionAliasInfo, ::windows_core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionAliasInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionAliasInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionAliasInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionAliasInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionAliasInfo {
    type Vtable = IPropertyDescriptionAliasInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionAliasInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescriptionAliasInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSortByAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAdditionalSortByAliases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionList(::windows_core::IUnknown);
impl IPropertyDescriptionList {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, ielem: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), ielem, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescriptionList, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyDescriptionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionList {}
impl ::core::fmt::Debug for IPropertyDescriptionList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionList {
    type Vtable = IPropertyDescriptionList_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescriptionList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionRelatedPropertyInfo(::windows_core::IUnknown);
impl IPropertyDescriptionRelatedPropertyInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetRelatedProperty<P0, T>(&self, pszrelationshipname: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetRelatedProperty)(::windows_core::Interface::as_raw(self), pszrelationshipname.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescriptionRelatedPropertyInfo, ::windows_core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionRelatedPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionRelatedPropertyInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionRelatedPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionRelatedPropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionRelatedPropertyInfo {
    type Vtable = IPropertyDescriptionRelatedPropertyInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionRelatedPropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescriptionRelatedPropertyInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetRelatedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrelationshipname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyDescriptionSearchInfo(::windows_core::IUnknown);
impl IPropertyDescriptionSearchInfo {
    pub unsafe fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPropertyKey)(::windows_core::Interface::as_raw(self), pkey).ok()
    }
    pub unsafe fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyType(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEditInvitation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTypeFlags)(::windows_core::Interface::as_raw(self), mask, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultColumnWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumnState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGroupingRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescriptionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRelativeDescription)(::windows_core::Interface::as_raw(self), propvar1, propvar2, ppszdesc1, ppszdesc2).ok()
    }
    pub unsafe fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSortDescriptionLabel<P0>(&self, fdescending: P0) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSortDescriptionLabel)(::windows_core::Interface::as_raw(self), fdescending.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAggregationType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetConditionType)(::windows_core::Interface::as_raw(self), pcontype, popdefault).ok()
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.GetEnumTypeList)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CoerceToCanonicalValue)(::windows_core::Interface::as_raw(self), ppropvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FormatForDisplay)(::windows_core::Interface::as_raw(self), propvar, pdfflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsValueCanonical)(::windows_core::Interface::as_raw(self), propvar).ok()
    }
    pub unsafe fn GetSearchInfoFlags(&self) -> ::windows_core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSearchInfoFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumnIndexType(&self) -> ::windows_core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnIndexType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProjectionString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProjectionString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxSize(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyDescriptionSearchInfo, ::windows_core::IUnknown, IPropertyDescription);
impl ::core::cmp::PartialEq for IPropertyDescriptionSearchInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyDescriptionSearchInfo {}
impl ::core::fmt::Debug for IPropertyDescriptionSearchInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyDescriptionSearchInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyDescriptionSearchInfo {
    type Vtable = IPropertyDescriptionSearchInfo_Vtbl;
}
impl ::core::clone::Clone for IPropertyDescriptionSearchInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyDescriptionSearchInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x078f91bd_29a2_440f_924e_46a291524520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSearchInfoFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows_core::HRESULT,
    pub GetColumnIndexType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows_core::HRESULT,
    pub GetProjectionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszprojection: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumType(::windows_core::IUnknown);
impl IPropertyEnumType {
    pub unsafe fn GetEnumType(&self) -> ::windows_core::Result<PROPENUMTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRangeMinValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRangeSetValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyEnumType, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyEnumType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType {}
impl ::core::fmt::Debug for IPropertyEnumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumType {
    type Vtable = IPropertyEnumType_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyEnumType {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetEnumType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetRangeMinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetRangeMinValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetRangeSetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetRangeSetValue: usize,
    pub GetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumType2(::windows_core::IUnknown);
impl IPropertyEnumType2 {
    pub unsafe fn GetEnumType(&self) -> ::windows_core::Result<PROPENUMTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnumType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRangeMinValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRangeMinValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRangeSetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRangeSetValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayText(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDisplayText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImageReference(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImageReference)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyEnumType2, ::windows_core::IUnknown, IPropertyEnumType);
impl ::core::cmp::PartialEq for IPropertyEnumType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumType2 {}
impl ::core::fmt::Debug for IPropertyEnumType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumType2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumType2 {
    type Vtable = IPropertyEnumType2_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyEnumType2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_Vtbl {
    pub base__: IPropertyEnumType_Vtbl,
    pub GetImageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyEnumTypeList(::windows_core::IUnknown);
impl IPropertyEnumTypeList {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, itype: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), itype, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetConditionAt<T>(&self, nindex: u32) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetConditionAt)(::windows_core::Interface::as_raw(self), nindex, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindMatchingIndex)(::windows_core::Interface::as_raw(self), propvarcmp, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyEnumTypeList, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyEnumTypeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyEnumTypeList {}
impl ::core::fmt::Debug for IPropertyEnumTypeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyEnumTypeList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyEnumTypeList {
    type Vtable = IPropertyEnumTypeList_Vtbl;
}
impl ::core::clone::Clone for IPropertyEnumTypeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyEnumTypeList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConditionAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub FindMatchingIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    FindMatchingIndex: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStore(::windows_core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), key, propvar).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyStore, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStore {}
impl ::core::fmt::Debug for IPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStore {
    type Vtable = IPropertyStore_Vtbl;
}
impl ::core::clone::Clone for IPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreCache(::windows_core::IUnknown);
impl IPropertyStoreCache {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAt)(::windows_core::Interface::as_raw(self), iprop, pkey).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValue)(::windows_core::Interface::as_raw(self), key, propvar).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetState(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<PSC_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetState)(::windows_core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValueAndState)(::windows_core::Interface::as_raw(self), key, ppropvar, pstate).ok()
    }
    pub unsafe fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetState)(::windows_core::Interface::as_raw(self), key, state).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueAndState)(::windows_core::Interface::as_raw(self), key, ppropvar, state).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyStoreCache, ::windows_core::IUnknown, IPropertyStore);
impl ::core::cmp::PartialEq for IPropertyStoreCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCache {}
impl ::core::fmt::Debug for IPropertyStoreCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCache").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreCache {
    type Vtable = IPropertyStoreCache_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyStoreCache {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3017056d_9a91_4e90_937d_746c72abbf4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_Vtbl {
    pub base__: IPropertyStore_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetValueAndState: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetValueAndState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetValueAndState: usize,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreCapabilities(::windows_core::IUnknown);
impl IPropertyStoreCapabilities {
    pub unsafe fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPropertyWritable)(::windows_core::Interface::as_raw(self), key).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyStoreCapabilities, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStoreCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCapabilities {}
impl ::core::fmt::Debug for IPropertyStoreCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreCapabilities {
    type Vtable = IPropertyStoreCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyStoreCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e2d566_186e_4d49_bf41_6909ead56acc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsPropertyWritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreFactory(::windows_core::IUnknown);
impl IPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P0, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPropertyStore)(::windows_core::Interface::as_raw(self), flags, punkfactory.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPropertyStoreForKeys)(::windows_core::Interface::as_raw(self), rgkeys, ckeys, flags, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyStoreFactory, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyStoreFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreFactory {}
impl ::core::fmt::Debug for IPropertyStoreFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyStoreFactory {
    type Vtable = IPropertyStoreFactory_Vtbl;
}
impl ::core::clone::Clone for IPropertyStoreFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyStoreFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyStoreForKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertySystem(::windows_core::IUnknown);
impl IPropertySystem {
    pub unsafe fn GetPropertyDescription<T>(&self, propkey: *const PROPERTYKEY) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPropertyDescription)(::windows_core::Interface::as_raw(self), propkey, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyDescriptionByName<P0, T>(&self, pszcanonicalname: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPropertyDescriptionByName)(::windows_core::Interface::as_raw(self), pszcanonicalname.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyDescriptionListFromString<P0, T>(&self, pszproplist: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetPropertyDescriptionListFromString)(::windows_core::Interface::as_raw(self), pszproplist.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumeratePropertyDescriptions<T>(&self, filteron: PROPDESC_ENUMFILTER) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).EnumeratePropertyDescriptions)(::windows_core::Interface::as_raw(self), filteron, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), key, propvar, pdff, ::core::mem::transmute(psztext.as_ptr()), psztext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FormatForDisplayAlloc)(::windows_core::Interface::as_raw(self), key, propvar, pdff, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterPropertySchema<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterPropertySchema)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertySchema<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterPropertySchema)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn RefreshPropertySchema(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshPropertySchema)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertySystem, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystem {}
impl ::core::fmt::Debug for IPropertySystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertySystem {
    type Vtable = IPropertySystem_Vtbl;
}
impl ::core::clone::Clone for IPropertySystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertySystem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyDescriptionByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcanonicalname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyDescriptionListFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszproplist: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumeratePropertyDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub FormatForDisplayAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    FormatForDisplayAlloc: usize,
    pub RegisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub RefreshPropertySchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertySystemChangeNotify(::windows_core::IUnknown);
impl IPropertySystemChangeNotify {
    pub unsafe fn SchemaRefreshed(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SchemaRefreshed)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertySystemChangeNotify, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySystemChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySystemChangeNotify {}
impl ::core::fmt::Debug for IPropertySystemChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySystemChangeNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertySystemChangeNotify {
    type Vtable = IPropertySystemChangeNotify_Vtbl;
}
impl ::core::clone::Clone for IPropertySystemChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertySystemChangeNotify {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa955fd9_38be_4879_a6ce_824cf52d609f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SchemaRefreshed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
pub struct IPropertyUI(::windows_core::IUnknown);
impl IPropertyUI {
    pub unsafe fn ParsePropertyName<P0>(&self, pszname: P0, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ParsePropertyName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi(), pfmtid, ppid, pcheaten).ok()
    }
    pub unsafe fn GetCannonicalName(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCannonicalName)(::windows_core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDisplayName(&self, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), fmtid, pid, flags, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetPropertyDescription(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyDescription)(::windows_core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetDefaultWidth(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultWidth)(::windows_core::Interface::as_raw(self), fmtid, pid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<PROPERTYUI_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), fmtid, pid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn FormatForDisplay(&self, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FormatForDisplay)(::windows_core::Interface::as_raw(self), fmtid, pid, ppropvar, puiff, ::core::mem::transmute(pwsztext.as_ptr()), pwsztext.len() as _).ok()
    }
    pub unsafe fn GetHelpInfo(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: &mut [u16], puhelpid: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHelpInfo)(::windows_core::Interface::as_raw(self), fmtid, pid, ::core::mem::transmute(pwszhelpfile.as_ptr()), pwszhelpfile.len() as _, puhelpid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPropertyUI, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPropertyUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyUI {}
impl ::core::fmt::Debug for IPropertyUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPropertyUI {
    type Vtable = IPropertyUI_Vtbl;
}
impl ::core::clone::Clone for IPropertyUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPropertyUI {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x757a7d9f_919a_4118_99d7_dbb208c8cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyUI_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ParsePropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::HRESULT,
    pub GetCannonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetPropertyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    pub GetDefaultWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub FormatForDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    FormatForDisplay: usize,
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_DEFAULT: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_READONLY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: _PERSIST_SPROPSTORE_FLAGS = _PERSIST_SPROPSTORE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_BESTEFFORT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_DEFAULT: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_DELAYCREATION: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_EXTRINSICPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_EXTRINSICPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_FASTPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_HANDLERPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_MASK_VALID: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(8191i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_NO_OPLOCK: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_OPENSLOWITEM: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_PREFERQUERYPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_READWRITE: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_TEMPORARY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_VOLATILEPROPERTIES: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const GPS_VOLATILEPROPERTIESONLY: GETPROPERTYSTOREFLAGS = GETPROPERTYSTOREFLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const InMemoryPropertyStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const InMemoryPropertyStoreMarshalByValue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_AVERAGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_DATERANGE: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_DEFAULT: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_FIRST: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_MAX: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_MIN: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_SUM: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDAT_UNION: PROPDESC_AGGREGATION_TYPE = PROPDESC_AGGREGATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_INMEMORY: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_NONE: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDEMAND: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISK: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISKALL: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCIT_ONDISKVECTOR: PROPDESC_COLUMNINDEX_TYPE = PROPDESC_COLUMNINDEX_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_BOOLEAN: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_DATETIME: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_NONE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_NUMBER: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_SIZE: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDCOT_STRING: PROPDESC_CONDITION_TYPE = PROPDESC_CONDITION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_BOOLEAN: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_DATETIME: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_ENUMERATED: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_NUMBER: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDDT_STRING: PROPDESC_DISPLAYTYPE = PROPDESC_DISPLAYTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_ALL: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_COLUMN: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_INFULLTEXTQUERY: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_NONSYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_QUERYABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_SYSTEM: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDEF_VIEWABLE: PROPDESC_ENUMFILTER = PROPDESC_ENUMFILTER(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_ALWAYSKB: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_DEFAULT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_FILENAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_HIDEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_HIDETIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_LONGDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_LONGTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_NOAUTOREADINGORDER: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_PREFIXNAME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_READONLY: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_RELATIVEDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_RESERVED_RIGHTTOLEFT: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_SHORTDATE: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_SHORTTIME: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDFF_USEEDITINVITATION: PROPDESC_FORMAT_FLAGS = PROPDESC_FORMAT_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_ALPHANUMERIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DATE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DISCRETE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_DYNAMIC: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_ENUMERATED: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_PERCENT: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDGR_SIZE: PROPDESC_GROUPING_RANGE = PROPDESC_GROUPING_RANGE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_CANCELLED: PDOPSTATUS = PDOPSTATUS(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_ERRORS: PDOPSTATUS = PDOPSTATUS(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_PAUSED: PDOPSTATUS = PDOPSTATUS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_RUNNING: PDOPSTATUS = PDOPSTATUS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDOPS_STOPPED: PDOPSTATUS = PDOPSTATUS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_COUNT: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_DATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_DURATION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_GENERAL: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_LENGTH: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_PRIORITY: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_RATE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_RATING: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_REVISION: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_SIZE: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDRDT_SPEED: PROPDESC_RELATIVEDESCRIPTION_TYPE = PROPDESC_RELATIVEDESCRIPTION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_A_Z: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_GENERAL: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_LOWEST_HIGHEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_OLDEST_NEWEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSD_SMALLEST_BIGGEST: PROPDESC_SORTDESCRIPTION = PROPDESC_SORTDESCRIPTION(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ALWAYSINCLUDE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_DEFAULT: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ININVERTEDINDEX: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ISCOLUMN: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_ISCOLUMNSPARSE: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDSIF_USEFORTYPEAHEAD: PROPDESC_SEARCHINFO_FLAGS = PROPDESC_SEARCHINFO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ALWAYSINSUPPLEMENTALSTORE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANBEPURGED: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANGROUPBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_CANSTACKBY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_DEFAULT: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_DONTCOERCEEMPTYSTRINGS: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_INCLUDEINFULLTEXTQUERY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISGROUP: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISINNATE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISQUERYABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISSYSTEMPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISTREEPROPERTY: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_ISVIEWABLE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_MASK_ALL: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(2147491839u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_MULTIPLEVALUES: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDTF_SEARCHRAWVALUE: PROPDESC_TYPE_FLAGS = PROPDESC_TYPE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_BEGINNEWGROUP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_CANWRAP: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_CENTERALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_DEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_FILLAREA: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_HIDDEN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_HIDELABEL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_MASK_ALL: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(7167i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_RIGHTALIGN: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWBYDEFAULT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWINPRIMARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWINSECONDARYLIST: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SHOWONLYIFPRESENT: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PDVF_SORTDESCENDING: PROPDESC_VIEW_FLAGS = PROPDESC_VIEW_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_DEFAULTVALUE: PROPENUMTYPE = PROPENUMTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_DISCRETEVALUE: PROPENUMTYPE = PROPENUMTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_ENDRANGE: PROPENUMTYPE = PROPENUMTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PET_RANGEDVALUE: PROPENUMTYPE = PROPENUMTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_APPEND: PKA_FLAGS = PKA_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_DELETE: PKA_FLAGS = PKA_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKA_SET: PKA_FLAGS = PKA_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PKEY_PIDSTR_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_DIRTY: PSC_STATE = PSC_STATE(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_NORMAL: PSC_STATE = PSC_STATE(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_NOTINSOURCE: PSC_STATE = PSC_STATE(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PSC_READONLY: PSC_STATE = PSC_STATE(3i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_ALL: PLACEHOLDER_STATES = PLACEHOLDER_STATES(15i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_CLOUDFILE_PLACEHOLDER: PLACEHOLDER_STATES = PLACEHOLDER_STATES(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_CREATE_FILE_ACCESSIBLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_DEFAULT: PLACEHOLDER_STATES = PLACEHOLDER_STATES(7i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_FULL_PRIMARY_STREAM_AVAILABLE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_MARKED_FOR_OFFLINE_AVAILABILITY: PLACEHOLDER_STATES = PLACEHOLDER_STATES(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PS_NONE: PLACEHOLDER_STATES = PLACEHOLDER_STATES(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_DEFAULT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_FRIENDLYDATE: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_NOTIME: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_RIGHTTOLEFT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFFDF_SHORTFORMAT: PROPERTYUI_FORMAT_FLAGS = PROPERTYUI_FORMAT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFNF_DEFAULT: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIFNF_MNEMONIC: PROPERTYUI_NAME_FLAGS = PROPERTYUI_NAME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_DEFAULT: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_NOLABELININFOTIP: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PUIF_RIGHTALIGN: PROPERTYUI_FLAGS = PROPERTYUI_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const PropertySystem: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_ALL_FLAGS: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(511i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_AUTHENTICATION_ERROR: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_NONE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_CLIENT_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_DISK_SPACE_FULL: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_METERED_NETWORK: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_SERVICE_POLICY: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_PAUSED_DUE_TO_USER_REQUEST: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_QUOTA_EXCEEDED_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_QUOTA_NEARING_LIMIT: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const SESF_SERVICE_UNAVAILABLE: SYNC_ENGINE_STATE_FLAGS = SYNC_ENGINE_STATE_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_EXCLUDED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(256i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_FETCHING_METADATA: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(32i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_HASERROR: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(16i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_HASWARNING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(128i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_INCOMPLETE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(512i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NEEDSDOWNLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NEEDSUPLOAD: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_NONE: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_PAUSED: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(8i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_PLACEHOLDER_IFEMPTY: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(1024i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_TRANSFERRING: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub const STS_USER_REQUESTED_REFRESH: SYNC_TRANSFER_STATUS = SYNC_TRANSFER_STATUS(64i32);
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GETPROPERTYSTOREFLAGS(pub i32);
impl ::core::marker::Copy for GETPROPERTYSTOREFLAGS {}
impl ::core::clone::Clone for GETPROPERTYSTOREFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GETPROPERTYSTOREFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GETPROPERTYSTOREFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GETPROPERTYSTOREFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETPROPERTYSTOREFLAGS").field(&self.0).finish()
    }
}
impl GETPROPERTYSTOREFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GETPROPERTYSTOREFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GETPROPERTYSTOREFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GETPROPERTYSTOREFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PDOPSTATUS(pub i32);
impl ::core::marker::Copy for PDOPSTATUS {}
impl ::core::clone::Clone for PDOPSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PDOPSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PDOPSTATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PDOPSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PDOPSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PKA_FLAGS(pub i32);
impl ::core::marker::Copy for PKA_FLAGS {}
impl ::core::clone::Clone for PKA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PKA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PKA_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PKA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PKA_FLAGS").field(&self.0).finish()
    }
}
impl PKA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PKA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PKA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PKA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PKA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PKA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PLACEHOLDER_STATES(pub i32);
impl ::core::marker::Copy for PLACEHOLDER_STATES {}
impl ::core::clone::Clone for PLACEHOLDER_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PLACEHOLDER_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PLACEHOLDER_STATES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PLACEHOLDER_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLACEHOLDER_STATES").field(&self.0).finish()
    }
}
impl PLACEHOLDER_STATES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PLACEHOLDER_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PLACEHOLDER_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PLACEHOLDER_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PLACEHOLDER_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_AGGREGATION_TYPE(pub i32);
impl ::core::marker::Copy for PROPDESC_AGGREGATION_TYPE {}
impl ::core::clone::Clone for PROPDESC_AGGREGATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_AGGREGATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_AGGREGATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_AGGREGATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_AGGREGATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_COLUMNINDEX_TYPE(pub i32);
impl ::core::marker::Copy for PROPDESC_COLUMNINDEX_TYPE {}
impl ::core::clone::Clone for PROPDESC_COLUMNINDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_COLUMNINDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_COLUMNINDEX_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_COLUMNINDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_COLUMNINDEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_CONDITION_TYPE(pub i32);
impl ::core::marker::Copy for PROPDESC_CONDITION_TYPE {}
impl ::core::clone::Clone for PROPDESC_CONDITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_CONDITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_CONDITION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_CONDITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_CONDITION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_DISPLAYTYPE(pub i32);
impl ::core::marker::Copy for PROPDESC_DISPLAYTYPE {}
impl ::core::clone::Clone for PROPDESC_DISPLAYTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_DISPLAYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_DISPLAYTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_DISPLAYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_DISPLAYTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_ENUMFILTER(pub i32);
impl ::core::marker::Copy for PROPDESC_ENUMFILTER {}
impl ::core::clone::Clone for PROPDESC_ENUMFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_ENUMFILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_ENUMFILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_ENUMFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_ENUMFILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_FORMAT_FLAGS(pub i32);
impl ::core::marker::Copy for PROPDESC_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPDESC_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_FORMAT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_GROUPING_RANGE(pub i32);
impl ::core::marker::Copy for PROPDESC_GROUPING_RANGE {}
impl ::core::clone::Clone for PROPDESC_GROUPING_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_GROUPING_RANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_GROUPING_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_GROUPING_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_GROUPING_RANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_RELATIVEDESCRIPTION_TYPE(pub i32);
impl ::core::marker::Copy for PROPDESC_RELATIVEDESCRIPTION_TYPE {}
impl ::core::clone::Clone for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_RELATIVEDESCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_RELATIVEDESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_SEARCHINFO_FLAGS(pub i32);
impl ::core::marker::Copy for PROPDESC_SEARCHINFO_FLAGS {}
impl ::core::clone::Clone for PROPDESC_SEARCHINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SEARCHINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_SEARCHINFO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_SEARCHINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SEARCHINFO_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_SEARCHINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_SEARCHINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_SEARCHINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_SORTDESCRIPTION(pub i32);
impl ::core::marker::Copy for PROPDESC_SORTDESCRIPTION {}
impl ::core::clone::Clone for PROPDESC_SORTDESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_SORTDESCRIPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_SORTDESCRIPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_SORTDESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_SORTDESCRIPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_TYPE_FLAGS(pub u32);
impl ::core::marker::Copy for PROPDESC_TYPE_FLAGS {}
impl ::core::clone::Clone for PROPDESC_TYPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_TYPE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_TYPE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_TYPE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_TYPE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_TYPE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPDESC_VIEW_FLAGS(pub i32);
impl ::core::marker::Copy for PROPDESC_VIEW_FLAGS {}
impl ::core::clone::Clone for PROPDESC_VIEW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPDESC_VIEW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPDESC_VIEW_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPDESC_VIEW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPDESC_VIEW_FLAGS").field(&self.0).finish()
    }
}
impl PROPDESC_VIEW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPDESC_VIEW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPDESC_VIEW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPDESC_VIEW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPENUMTYPE(pub i32);
impl ::core::marker::Copy for PROPENUMTYPE {}
impl ::core::clone::Clone for PROPENUMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPENUMTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPENUMTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPENUMTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPENUMTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_FLAGS(pub i32);
impl ::core::marker::Copy for PROPERTYUI_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPERTYUI_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_FORMAT_FLAGS(pub i32);
impl ::core::marker::Copy for PROPERTYUI_FORMAT_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPERTYUI_FORMAT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_FORMAT_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_FORMAT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_FORMAT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_FORMAT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPERTYUI_NAME_FLAGS(pub i32);
impl ::core::marker::Copy for PROPERTYUI_NAME_FLAGS {}
impl ::core::clone::Clone for PROPERTYUI_NAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYUI_NAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROPERTYUI_NAME_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROPERTYUI_NAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYUI_NAME_FLAGS").field(&self.0).finish()
    }
}
impl PROPERTYUI_NAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROPERTYUI_NAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROPERTYUI_NAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROPERTYUI_NAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSC_STATE(pub i32);
impl ::core::marker::Copy for PSC_STATE {}
impl ::core::clone::Clone for PSC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PSC_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PSC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSC_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_ENGINE_STATE_FLAGS(pub i32);
impl ::core::marker::Copy for SYNC_ENGINE_STATE_FLAGS {}
impl ::core::clone::Clone for SYNC_ENGINE_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_ENGINE_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SYNC_ENGINE_STATE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_ENGINE_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_ENGINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl SYNC_ENGINE_STATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_ENGINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_ENGINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_TRANSFER_STATUS(pub i32);
impl ::core::marker::Copy for SYNC_TRANSFER_STATUS {}
impl ::core::clone::Clone for SYNC_TRANSFER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_TRANSFER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SYNC_TRANSFER_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SYNC_TRANSFER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_TRANSFER_STATUS").field(&self.0).finish()
    }
}
impl SYNC_TRANSFER_STATUS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNC_TRANSFER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNC_TRANSFER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNC_TRANSFER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _PERSIST_SPROPSTORE_FLAGS(pub i32);
impl ::core::marker::Copy for _PERSIST_SPROPSTORE_FLAGS {}
impl ::core::clone::Clone for _PERSIST_SPROPSTORE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _PERSIST_SPROPSTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _PERSIST_SPROPSTORE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _PERSIST_SPROPSTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_PERSIST_SPROPSTORE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PCUSERIALIZEDPROPSTORAGE(pub isize);
impl ::core::default::Default for PCUSERIALIZEDPROPSTORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PCUSERIALIZEDPROPSTORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PCUSERIALIZEDPROPSTORAGE {}
impl ::core::fmt::Debug for PCUSERIALIZEDPROPSTORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PCUSERIALIZEDPROPSTORAGE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for PCUSERIALIZEDPROPSTORAGE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub struct PROPERTYKEY {
    pub fmtid: ::windows_core::GUID,
    pub pid: u32,
}
impl ::core::marker::Copy for PROPERTYKEY {}
impl ::core::clone::Clone for PROPERTYKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPERTYKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPERTYKEY").field("fmtid", &self.fmtid).field("pid", &self.pid).finish()
    }
}
impl ::windows_core::TypeKind for PROPERTYKEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PROPERTYKEY {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.pid == other.pid
    }
}
impl ::core::cmp::Eq for PROPERTYKEY {}
impl ::core::default::Default for PROPERTYKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
pub struct PROPPRG {
    pub flPrg: u16,
    pub flPrgInit: u16,
    pub achTitle: [u8; 30],
    pub achCmdLine: [u8; 128],
    pub achWorkDir: [u8; 64],
    pub wHotKey: u16,
    pub achIconFile: [u8; 80],
    pub wIconIndex: u16,
    pub dwEnhModeFlags: u32,
    pub dwRealModeFlags: u32,
    pub achOtherFile: [u8; 80],
    pub achPIFFile: [u8; 260],
}
impl ::core::marker::Copy for PROPPRG {}
impl ::core::clone::Clone for PROPPRG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PROPPRG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PROPPRG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERIALIZEDPROPSTORAGE(pub isize);
impl ::core::default::Default for SERIALIZEDPROPSTORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SERIALIZEDPROPSTORAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SERIALIZEDPROPSTORAGE {}
impl ::core::fmt::Debug for SERIALIZEDPROPSTORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERIALIZEDPROPSTORAGE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for SERIALIZEDPROPSTORAGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
