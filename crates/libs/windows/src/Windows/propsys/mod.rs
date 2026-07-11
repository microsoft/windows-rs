#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSCoerceToCanonicalValue(key: *const super::wtypes::PROPERTYKEY, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PSCoerceToCanonicalValue(key : *const super::wtypes::PROPERTYKEY, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PSCoerceToCanonicalValue(key, core::mem::transmute(ppropvar)) }
}
#[inline]
pub unsafe fn PSCreateAdapterFromPropertyStore<P0, T>(pps: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<IPropertyStore>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreateAdapterFromPropertyStore(pps : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreateAdapterFromPropertyStore(pps.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSCreateDelayedMultiplexPropertyStore<P1, T>(flags: GETPROPERTYSTOREFLAGS, pdpsf: P1, rgstoreids: &[u32]) -> windows_core::Result<T>
where
    P1: windows_core::Param<IDelayedPropertyStoreFactory>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreateDelayedMultiplexPropertyStore(flags : GETPROPERTYSTOREFLAGS, pdpsf : *mut core::ffi::c_void, rgstoreids : *const u32, cstores : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreateDelayedMultiplexPropertyStore(flags, pdpsf.param().abi(), core::mem::transmute(rgstoreids.as_ptr()), rgstoreids.len().try_into().unwrap(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSCreateMemoryPropertyStore<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreateMemoryPropertyStore(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreateMemoryPropertyStore(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSCreateMultiplexPropertyStore<T>(prgpunkstores: &[Option<windows_core::IUnknown>]) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreateMultiplexPropertyStore(prgpunkstores : *const *mut core::ffi::c_void, cstores : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreateMultiplexPropertyStore(core::mem::transmute(prgpunkstores.as_ptr()), prgpunkstores.len().try_into().unwrap(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSCreatePropertyChangeArray<T>(rgpropkey: Option<*const super::wtypes::PROPERTYKEY>, rgflags: Option<*const PKA_FLAGS>, rgpropvar: Option<*const super::propidlbase::PROPVARIANT>, cchanges: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreatePropertyChangeArray(rgpropkey : *const super::wtypes::PROPERTYKEY, rgflags : *const PKA_FLAGS, rgpropvar : *const super::propidlbase::PROPVARIANT, cchanges : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreatePropertyChangeArray(rgpropkey.unwrap_or(core::mem::zeroed()) as _, rgflags.unwrap_or(core::mem::zeroed()) as _, rgpropvar.unwrap_or(core::mem::zeroed()) as _, cchanges, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSCreatePropertyStoreFromObject<P0, T>(punk: P0, grfmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromObject(punk : *mut core::ffi::c_void, grfmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreatePropertyStoreFromObject(punk.param().abi(), grfmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "propidlbase")]
#[inline]
pub unsafe fn PSCreatePropertyStoreFromPropertySetStorage<P0, T>(ppss: P0, grfmode: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::propidlbase::IPropertySetStorage>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreatePropertyStoreFromPropertySetStorage(ppss : *mut core::ffi::c_void, grfmode : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreatePropertyStoreFromPropertySetStorage(ppss.param().abi(), grfmode, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSCreateSimplePropertyChange<T>(flags: PKA_FLAGS, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSCreateSimplePropertyChange(flags : PKA_FLAGS, key : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSCreateSimplePropertyChange(flags, key, core::mem::transmute(propvar), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSEnumeratePropertyDescriptions<T>(filteron: PROPDESC_ENUMFILTER) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSEnumeratePropertyDescriptions(filteron : PROPDESC_ENUMFILTER, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSEnumeratePropertyDescriptions(filteron, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSFormatForDisplay(propkey: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, pwsztext: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PSFormatForDisplay(propkey : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, pdfflags : PROPDESC_FORMAT_FLAGS, pwsztext : windows_core::PWSTR, cchtext : u32) -> windows_core::HRESULT);
    unsafe { PSFormatForDisplay(propkey, core::mem::transmute(propvar), pdfflags, core::mem::transmute(pwsztext.as_ptr()), pwsztext.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSFormatForDisplayAlloc(key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PSFormatForDisplayAlloc(key : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSFormatForDisplayAlloc(key, core::mem::transmute(propvar), pdff, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PSFormatPropertyValue<P0, P1>(pps: P0, ppd: P1, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_core::link!("propsys.dll" "system" fn PSFormatPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, pdff : PROPDESC_FORMAT_FLAGS, ppszdisplay : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSFormatPropertyValue(pps.param().abi(), ppd.param().abi(), pdff, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSGetImageReferenceForValue(propkey: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PSGetImageReferenceForValue(propkey : *const super::wtypes::PROPERTYKEY, propvar : *const super::propidlbase::PROPVARIANT, ppszimageres : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSGetImageReferenceForValue(propkey, core::mem::transmute(propvar), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PSGetItemPropertyHandler<P0, T>(punkitem: P0, freadwrite: bool) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetItemPropertyHandler(punkitem : *mut core::ffi::c_void, freadwrite : windows_core::BOOL, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetItemPropertyHandler(punkitem.param().abi(), freadwrite.into(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSGetItemPropertyHandlerWithCreateObject<P0, P2, T>(punkitem: P0, freadwrite: bool, punkcreateobject: P2) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetItemPropertyHandlerWithCreateObject(punkitem : *mut core::ffi::c_void, freadwrite : windows_core::BOOL, punkcreateobject : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetItemPropertyHandlerWithCreateObject(punkitem.param().abi(), freadwrite.into(), punkcreateobject.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn PSGetNameFromPropertyKey(propkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("propsys.dll" "system" fn PSGetNameFromPropertyKey(propkey : *const super::wtypes::PROPERTYKEY, ppszcanonicalname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSGetNameFromPropertyKey(propkey, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSGetNamedPropertyFromPropertyStorage<P2>(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, pszname: P2) -> windows_core::Result<super::propidlbase::PROPVARIANT>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSGetNamedPropertyFromPropertyStorage(psps : *const SERIALIZEDPROPSTORAGE, cb : u32, pszname : windows_core::PCWSTR, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSGetNamedPropertyFromPropertyStorage(psps, cb, pszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn PSGetPropertyDescription<T>(propkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyDescription(propkey : *const super::wtypes::PROPERTYKEY, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetPropertyDescription(propkey, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionByName<P0, T>(pszcanonicalname: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyDescriptionByName(pszcanonicalname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetPropertyDescriptionByName(pszcanonicalname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn PSGetPropertyDescriptionListFromString<P0, T>(pszproplist: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyDescriptionListFromString(pszproplist : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetPropertyDescriptionListFromString(pszproplist.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSGetPropertyFromPropertyStorage(psps: *const SERIALIZEDPROPSTORAGE, cb: u32, rpkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyFromPropertyStorage(psps : *const SERIALIZEDPROPSTORAGE, cb : u32, rpkey : *const super::wtypes::PROPERTYKEY, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSGetPropertyFromPropertyStorage(psps, cb, rpkey, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn PSGetPropertyKeyFromName<P0>(pszname: P0, ppropkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyKeyFromName(pszname : windows_core::PCWSTR, ppropkey : *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT);
    unsafe { PSGetPropertyKeyFromName(pszname.param().abi(), ppropkey as _) }
}
#[inline]
pub unsafe fn PSGetPropertySystem<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertySystem(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSGetPropertySystem(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSGetPropertyValue<P0, P1>(pps: P0, ppd: P1) -> windows_core::Result<super::propidlbase::PROPVARIANT>
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_core::link!("propsys.dll" "system" fn PSGetPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSGetPropertyValue(pps.param().abi(), ppd.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn PSLookupPropertyHandlerCLSID<P0>(pszfilepath: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSLookupPropertyHandlerCLSID(pszfilepath : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSLookupPropertyHandlerCLSID(pszfilepath.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_Delete<P0, P1>(propbag: P0, propname: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_Delete(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_Delete(propbag.param().abi(), propname.param().abi()) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBOOL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadBOOL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadBOOL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadBSTR<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadBSTR(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadBSTR(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadDWORD<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadDWORD(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadDWORD(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadGUID<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadGUID(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadGUID(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadInt<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadInt(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadInt(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadLONG<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadLONG(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadLONG(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::windef::POINTL>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::windef::POINTL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadPOINTL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPOINTS<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::windef::POINTS>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadPOINTS(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::windef::POINTS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadPOINTS(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadPropertyKey<P0, P1>(propbag: P0, propname: P1, value: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadPropertyKey(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_ReadPropertyKey(propbag.param().abi(), propname.param().abi(), value as _) }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadRECTL<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::windef::RECTL>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadRECTL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut super::windef::RECTL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadRECTL(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadSHORT<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<i16>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadSHORT(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadSHORT(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStr<P0, P1>(propbag: P0, propname: P1, value: &mut [u16]) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadStr(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : windows_core::PWSTR, charactercount : i32) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_ReadStr(propbag.param().abi(), propname.param().abi(), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadStrAlloc<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadStrAlloc(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadStrAlloc(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadStream<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<super::objidlbase::IStream>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadStream(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadStream(propbag.param().abi(), propname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSPropertyBag_ReadType<P0, P1>(propbag: P0, propname: P1, var: *mut super::oaidl::VARIANT, r#type: super::wtypes::VARTYPE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadType(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, var : *mut super::oaidl::VARIANT, r#type : super::wtypes::VARTYPE) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_ReadType(propbag.param().abi(), propname.param().abi(), core::mem::transmute(var), r#type) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadULONGLONG<P0, P1>(propbag: P0, propname: P1) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadULONGLONG(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PSPropertyBag_ReadULONGLONG(propbag.param().abi(), propname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_ReadUnknown<P0, P1, T>(propbag: P0, propname: P1) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_ReadUnknown(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PSPropertyBag_ReadUnknown(propbag.param().abi(), propname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBOOL<P0, P1>(propbag: P0, propname: P1, value: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteBOOL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteBOOL(propbag.param().abi(), propname.param().abi(), value.into()) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteBSTR<P0, P1>(propbag: P0, propname: P1, value: &windows_core::BSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteBSTR(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteBSTR(propbag.param().abi(), propname.param().abi(), core::mem::transmute_copy(value)) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteDWORD<P0, P1>(propbag: P0, propname: P1, value: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteDWORD(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : u32) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteDWORD(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteGUID<P0, P1>(propbag: P0, propname: P1, value: *const windows_core::GUID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteGUID(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteGUID(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteInt<P0, P1>(propbag: P0, propname: P1, value: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteInt(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i32) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteInt(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteLONG<P0, P1>(propbag: P0, propname: P1, value: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteLONG(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i32) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteLONG(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTL<P0, P1>(propbag: P0, propname: P1, value: *const super::windef::POINTL) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::windef::POINTL) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WritePOINTL(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePOINTS<P0, P1>(propbag: P0, propname: P1, value: *const super::windef::POINTS) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WritePOINTS(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::windef::POINTS) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WritePOINTS(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn PSPropertyBag_WritePropertyKey<P0, P1>(propbag: P0, propname: P1, value: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WritePropertyKey(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WritePropertyKey(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(all(feature = "oaidl", feature = "windef"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteRECTL<P0, P1>(propbag: P0, propname: P1, value: *const super::windef::RECTL) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteRECTL(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *const super::windef::RECTL) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteRECTL(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteSHORT<P0, P1>(propbag: P0, propname: P1, value: i16) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteSHORT(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : i16) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteSHORT(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteStr<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteStr(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteStr(propbag.param().abi(), propname.param().abi(), value.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase"))]
#[inline]
pub unsafe fn PSPropertyBag_WriteStream<P0, P1, P2>(propbag: P0, propname: P1, value: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteStream(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteStream(propbag.param().abi(), propname.param().abi(), value.param().abi()) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteULONGLONG<P0, P1>(propbag: P0, propname: P1, value: u64) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteULONGLONG(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, value : u64) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteULONGLONG(propbag.param().abi(), propname.param().abi(), value) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn PSPropertyBag_WriteUnknown<P0, P1, P2>(propbag: P0, propname: P1, punk: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::IPropertyBag>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyBag_WriteUnknown(propbag : *mut core::ffi::c_void, propname : windows_core::PCWSTR, punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PSPropertyBag_WriteUnknown(propbag.param().abi(), propname.param().abi(), punk.param().abi()) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn PSPropertyKeyFromString<P0>(pszstring: P0, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSPropertyKeyFromString(pszstring : windows_core::PCWSTR, pkey : *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT);
    unsafe { PSPropertyKeyFromString(pszstring.param().abi(), pkey as _) }
}
#[inline]
pub unsafe fn PSRefreshPropertySchema() -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PSRefreshPropertySchema() -> windows_core::HRESULT);
    unsafe { PSRefreshPropertySchema() }
}
#[inline]
pub unsafe fn PSRegisterPropertySchema<P0>(pszpath: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSRegisterPropertySchema(pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PSRegisterPropertySchema(pszpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PSSetPropertyValue<P0, P1>(pps: P0, ppd: P1, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
where
    P0: windows_core::Param<IPropertyStore>,
    P1: windows_core::Param<IPropertyDescription>,
{
    windows_core::link!("propsys.dll" "system" fn PSSetPropertyValue(pps : *mut core::ffi::c_void, ppd : *mut core::ffi::c_void, propvar : *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PSSetPropertyValue(pps.param().abi(), ppd.param().abi(), core::mem::transmute(propvar)) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn PSStringFromPropertyKey(pkey: *const super::wtypes::PROPERTYKEY, psz: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("propsys.dll" "system" fn PSStringFromPropertyKey(pkey : *const super::wtypes::PROPERTYKEY, psz : windows_core::PWSTR, cch : u32) -> windows_core::HRESULT);
    unsafe { PSStringFromPropertyKey(pkey, core::mem::transmute(psz.as_ptr()), psz.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PSUnregisterPropertySchema<P0>(pszpath: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("propsys.dll" "system" fn PSUnregisterPropertySchema(pszpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { PSUnregisterPropertySchema(pszpath.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToWinRTPropertyValue<T>(propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("propsys.dll" "system" fn PropVariantToWinRTPropertyValue(propvar : *const super::propidlbase::PROPVARIANT, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { PropVariantToWinRTPropertyValue(core::mem::transmute(propvar), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WinRTPropertyValueToPropVariant<P0>(punkpropertyvalue: P0) -> windows_core::Result<super::propidlbase::PROPVARIANT>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("propsys.dll" "system" fn WinRTPropertyValueToPropVariant(punkpropertyvalue : *mut core::ffi::c_void, ppropvar : *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WinRTPropertyValueToPropVariant(punkpropertyvalue.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
pub const FPSPS_DEFAULT: PERSIST_SPROPSTORE_FLAGS = 0;
pub const FPSPS_READONLY: PERSIST_SPROPSTORE_FLAGS = 1;
pub const FPSPS_TREAT_NEW_VALUES_AS_DIRTY: PERSIST_SPROPSTORE_FLAGS = 2;
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
windows_core::imp::define_interface!(ICreateObject, ICreateObject_Vtbl, 0x75121952_e0d0_43e5_9380_1d80483acf72);
windows_core::imp::interface_hierarchy!(ICreateObject, windows_core::IUnknown);
impl ICreateObject {
    pub unsafe fn CreateObject<P1, T>(&self, clsid: *const windows_core::GUID, punkouter: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), clsid, punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICreateObject_Impl: windows_core::IUnknownImpl {
    fn CreateObject(&self, clsid: *const windows_core::GUID, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICreateObject_Vtbl {
    pub const fn new<Identity: ICreateObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateObject<Identity: ICreateObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateObject_Impl::CreateObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICreateObject {}
windows_core::imp::define_interface!(IDelayedPropertyStoreFactory, IDelayedPropertyStoreFactory_Vtbl, 0x40d4577f_e237_4bdb_bd69_58f089431b6a);
impl core::ops::Deref for IDelayedPropertyStoreFactory {
    type Target = IPropertyStoreFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDelayedPropertyStoreFactory, windows_core::IUnknown, IPropertyStoreFactory);
impl IDelayedPropertyStoreFactory {
    pub unsafe fn GetDelayedPropertyStore<T>(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDelayedPropertyStore)(windows_core::Interface::as_raw(self), flags, dwstoreid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDelayedPropertyStoreFactory_Vtbl {
    pub base__: IPropertyStoreFactory_Vtbl,
    pub GetDelayedPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, GETPROPERTYSTOREFLAGS, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IDelayedPropertyStoreFactory_Impl: IPropertyStoreFactory_Impl {
    fn GetDelayedPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IDelayedPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDelayedPropertyStore<Identity: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDelayedPropertyStoreFactory_Impl::GetDelayedPropertyStore(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&dwstoreid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: IPropertyStoreFactory_Vtbl::new::<Identity, OFFSET>(), GetDelayedPropertyStore: GetDelayedPropertyStore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDelayedPropertyStoreFactory as windows_core::Interface>::IID || iid == &<IPropertyStoreFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IDelayedPropertyStoreFactory {}
windows_core::imp::define_interface!(IInitializeWithFile, IInitializeWithFile_Vtbl, 0xb7d14566_0509_4cce_a71f_0a554233bd9b);
windows_core::imp::interface_hierarchy!(IInitializeWithFile, windows_core::IUnknown);
impl IInitializeWithFile {
    pub unsafe fn Initialize<P0>(&self, pszfilepath: P0, grfmode: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszfilepath.param().abi(), grfmode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IInitializeWithFile_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pszfilepath: &windows_core::PCWSTR, grfmode: u32) -> windows_core::Result<()>;
}
impl IInitializeWithFile_Vtbl {
    pub const fn new<Identity: IInitializeWithFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IInitializeWithFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCWSTR, grfmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeWithFile_Impl::Initialize(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&grfmode)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithFile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInitializeWithFile {}
windows_core::imp::define_interface!(IInitializeWithStream, IInitializeWithStream_Vtbl, 0xb824b49d_22ac_4161_ac8a_9916e8fa3f7f);
windows_core::imp::interface_hierarchy!(IInitializeWithStream, windows_core::IUnknown);
impl IInitializeWithStream {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Initialize<P0>(&self, pstream: P0, grfmode: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), grfmode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Initialize: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IInitializeWithStream_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pstream: windows_core::Ref<super::objidlbase::IStream>, grfmode: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IInitializeWithStream_Vtbl {
    pub const fn new<Identity: IInitializeWithStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IInitializeWithStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, grfmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeWithStream_Impl::Initialize(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&grfmode)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IInitializeWithStream {}
windows_core::imp::define_interface!(INamedPropertyStore, INamedPropertyStore_Vtbl, 0x71604b0f_97b0_4764_8577_2f13e98a1422);
windows_core::imp::interface_hierarchy!(INamedPropertyStore, windows_core::IUnknown);
impl INamedPropertyStore {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetNamedValue<P0>(&self, pszname: P0) -> windows_core::Result<super::propidlbase::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamedValue)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetNamedValue<P0>(&self, pszname: P0, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNamedValue)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(propvar)) }
    }
    pub unsafe fn GetNameCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNameAt(&self, iprop: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameAt)(windows_core::Interface::as_raw(self), iprop, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetNamedValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetNamedValue: usize,
    pub GetNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNameAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait INamedPropertyStore_Impl: windows_core::IUnknownImpl {
    fn GetNamedValue(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn SetNamedValue(&self, pszname: &windows_core::PCWSTR, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetNameCount(&self) -> windows_core::Result<u32>;
    fn GetNameAt(&self, iprop: u32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl INamedPropertyStore_Vtbl {
    pub const fn new<Identity: INamedPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamedValue<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INamedPropertyStore_Impl::GetNamedValue(this, core::mem::transmute(&pszname)) {
                    Ok(ok__) => {
                        ppropvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamedValue<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INamedPropertyStore_Impl::SetNamedValue(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&propvar)).into()
            }
        }
        unsafe extern "system" fn GetNameCount<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INamedPropertyStore_Impl::GetNameCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameAt<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprop: u32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INamedPropertyStore_Impl::GetNameAt(this, core::mem::transmute_copy(&iprop)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamedValue: GetNamedValue::<Identity, OFFSET>,
            SetNamedValue: SetNamedValue::<Identity, OFFSET>,
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNameAt: GetNameAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INamedPropertyStore {}
windows_core::imp::define_interface!(IObjectWithPropertyKey, IObjectWithPropertyKey_Vtbl, 0xfc0ca0a7_c316_4fd2_9031_3e628e6d4f23);
windows_core::imp::interface_hierarchy!(IObjectWithPropertyKey, windows_core::IUnknown);
impl IObjectWithPropertyKey {
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetPropertyKey(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyKey)(windows_core::Interface::as_raw(self), key) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyKey(&self, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyKey)(windows_core::Interface::as_raw(self), pkey as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithPropertyKey_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub SetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetPropertyKey: usize,
    #[cfg(feature = "wtypes")]
    pub GetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyKey: usize,
}
#[cfg(feature = "wtypes")]
pub trait IObjectWithPropertyKey_Impl: windows_core::IUnknownImpl {
    fn SetPropertyKey(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetPropertyKey(&self, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IObjectWithPropertyKey_Vtbl {
    pub const fn new<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPropertyKey<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithPropertyKey_Impl::SetPropertyKey(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn GetPropertyKey<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectWithPropertyKey_Impl::GetPropertyKey(this, core::mem::transmute_copy(&pkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropertyKey: SetPropertyKey::<Identity, OFFSET>,
            GetPropertyKey: GetPropertyKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectWithPropertyKey as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IObjectWithPropertyKey {}
windows_core::imp::define_interface!(IPersistSerializedPropStorage, IPersistSerializedPropStorage_Vtbl, 0xe318ad57_0aa0_450f_aca5_6fab7103d917);
windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage, windows_core::IUnknown);
impl IPersistSerializedPropStorage {
    pub unsafe fn SetFlags(&self, flags: PERSIST_SPROPSTORE_FLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyStorage)(windows_core::Interface::as_raw(self), psps, cb) }
    }
    pub unsafe fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStorage)(windows_core::Interface::as_raw(self), ppsps as _, pcb as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, PERSIST_SPROPSTORE_FLAGS) -> windows_core::HRESULT,
    pub SetPropertyStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *const SERIALIZEDPROPSTORAGE, u32) -> windows_core::HRESULT,
    pub GetPropertyStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SERIALIZEDPROPSTORAGE, *mut u32) -> windows_core::HRESULT,
}
pub trait IPersistSerializedPropStorage_Impl: windows_core::IUnknownImpl {
    fn SetFlags(&self, flags: PERSIST_SPROPSTORE_FLAGS) -> windows_core::Result<()>;
    fn SetPropertyStorage(&self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> windows_core::Result<()>;
    fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::Result<()>;
}
impl IPersistSerializedPropStorage_Vtbl {
    pub const fn new<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFlags<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: PERSIST_SPROPSTORE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistSerializedPropStorage_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn SetPropertyStorage<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistSerializedPropStorage_Impl::SetPropertyStorage(this, core::mem::transmute_copy(&psps), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn GetPropertyStorage<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistSerializedPropStorage_Impl::GetPropertyStorage(this, core::mem::transmute_copy(&ppsps), core::mem::transmute_copy(&pcb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFlags: SetFlags::<Identity, OFFSET>,
            SetPropertyStorage: SetPropertyStorage::<Identity, OFFSET>,
            GetPropertyStorage: GetPropertyStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistSerializedPropStorage {}
windows_core::imp::define_interface!(IPersistSerializedPropStorage2, IPersistSerializedPropStorage2_Vtbl, 0x77effa68_4f98_4366_ba72_573b3d880571);
impl core::ops::Deref for IPersistSerializedPropStorage2 {
    type Target = IPersistSerializedPropStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistSerializedPropStorage2, windows_core::IUnknown, IPersistSerializedPropStorage);
impl IPersistSerializedPropStorage2 {
    pub unsafe fn GetPropertyStorageSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyStorageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStorageBuffer)(windows_core::Interface::as_raw(self), psps as _, cb, pcbwritten as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistSerializedPropStorage2_Vtbl {
    pub base__: IPersistSerializedPropStorage_Vtbl,
    pub GetPropertyStorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyStorageBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SERIALIZEDPROPSTORAGE, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IPersistSerializedPropStorage2_Impl: IPersistSerializedPropStorage_Impl {
    fn GetPropertyStorageSize(&self) -> windows_core::Result<u32>;
    fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
}
impl IPersistSerializedPropStorage2_Vtbl {
    pub const fn new<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyStorageSize<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistSerializedPropStorage2_Impl::GetPropertyStorageSize(this) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistSerializedPropStorage2_Impl::GetPropertyStorageBuffer(this, core::mem::transmute_copy(&psps), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        Self {
            base__: IPersistSerializedPropStorage_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyStorageSize: GetPropertyStorageSize::<Identity, OFFSET>,
            GetPropertyStorageBuffer: GetPropertyStorageBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage2 as windows_core::Interface>::IID || iid == &<IPersistSerializedPropStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistSerializedPropStorage2 {}
windows_core::imp::define_interface!(IPropertyChange, IPropertyChange_Vtbl, 0xf917bc8a_1bba_4478_a245_1bde03eb9431);
impl core::ops::Deref for IPropertyChange {
    type Target = IObjectWithPropertyKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyChange, windows_core::IUnknown, IObjectWithPropertyKey);
impl IPropertyChange {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ApplyToPropVariant(&self, propvarin: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplyToPropVariant)(windows_core::Interface::as_raw(self), core::mem::transmute(propvarin), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChange_Vtbl {
    pub base__: IObjectWithPropertyKey_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub ApplyToPropVariant: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    ApplyToPropVariant: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyChange_Impl: IObjectWithPropertyKey_Impl {
    fn ApplyToPropVariant(&self, propvarin: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyChange_Vtbl {
    pub const fn new<Identity: IPropertyChange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplyToPropVariant<Identity: IPropertyChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvarin: *const super::propidlbase::PROPVARIANT, ppropvarout: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyChange_Impl::ApplyToPropVariant(this, core::mem::transmute_copy(&propvarin)) {
                    Ok(ok__) => {
                        ppropvarout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IObjectWithPropertyKey_Vtbl::new::<Identity, OFFSET>(), ApplyToPropVariant: ApplyToPropVariant::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyChange as windows_core::Interface>::IID || iid == &<IObjectWithPropertyKey as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyChange {}
windows_core::imp::define_interface!(IPropertyChangeArray, IPropertyChangeArray_Vtbl, 0x380f5cad_1b5e_42f2_805d_637fd392d31e);
windows_core::imp::interface_hierarchy!(IPropertyChangeArray, windows_core::IUnknown);
impl IPropertyChangeArray {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt<T>(&self, iindex: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), iindex, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn InsertAt<P1>(&self, iindex: u32, ppropchange: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IPropertyChange>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), iindex, ppropchange.param().abi()) }
    }
    pub unsafe fn Append<P0>(&self, ppropchange: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyChange>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), ppropchange.param().abi()) }
    }
    pub unsafe fn AppendOrReplace<P0>(&self, ppropchange: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPropertyChange>,
    {
        unsafe { (windows_core::Interface::vtable(self).AppendOrReplace)(windows_core::Interface::as_raw(self), ppropchange.param().abi()) }
    }
    pub unsafe fn RemoveAt(&self, iindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), iindex) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsKeyInArray(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsKeyInArray)(windows_core::Interface::as_raw(self), key) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangeArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendOrReplace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsKeyInArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsKeyInArray: usize,
}
#[cfg(feature = "wtypes")]
pub trait IPropertyChangeArray_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, iindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertAt(&self, iindex: u32, ppropchange: windows_core::Ref<IPropertyChange>) -> windows_core::Result<()>;
    fn Append(&self, ppropchange: windows_core::Ref<IPropertyChange>) -> windows_core::Result<()>;
    fn AppendOrReplace(&self, ppropchange: windows_core::Ref<IPropertyChange>) -> windows_core::Result<()>;
    fn RemoveAt(&self, iindex: u32) -> windows_core::Result<()>;
    fn IsKeyInArray(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IPropertyChangeArray_Vtbl {
    pub const fn new<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoperations: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyChangeArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcoperations.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::GetAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn InsertAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::InsertAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&ppropchange)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::Append(this, core::mem::transmute_copy(&ppropchange)).into()
            }
        }
        unsafe extern "system" fn AppendOrReplace<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::AppendOrReplace(this, core::mem::transmute_copy(&ppropchange)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::RemoveAt(this, core::mem::transmute_copy(&iindex)).into()
            }
        }
        unsafe extern "system" fn IsKeyInArray<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyChangeArray_Impl::IsKeyInArray(this, core::mem::transmute_copy(&key)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            InsertAt: InsertAt::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            AppendOrReplace: AppendOrReplace::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            IsKeyInArray: IsKeyInArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyChangeArray as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IPropertyChangeArray {}
windows_core::imp::define_interface!(IPropertyDescription, IPropertyDescription_Vtbl, 0x6f79d558_3e96_4549_a1d1_7d75d2288814);
windows_core::imp::interface_hierarchy!(IPropertyDescription, windows_core::IUnknown);
impl IPropertyDescription {
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyKey(&self, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyKey)(windows_core::Interface::as_raw(self), pkey as _) }
    }
    pub unsafe fn GetCanonicalName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCanonicalName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyType(&self) -> windows_core::Result<super::wtypes::VARTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEditInvitation(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEditInvitation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> windows_core::Result<PROPDESC_TYPE_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), mask, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetViewFlags(&self) -> windows_core::Result<PROPDESC_VIEW_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefaultColumnWidth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultColumnWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayType(&self) -> windows_core::Result<PROPDESC_DISPLAYTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shtypes")]
    pub unsafe fn GetColumnState(&self) -> windows_core::Result<super::shtypes::SHCOLSTATEF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGroupingRange(&self) -> windows_core::Result<PROPDESC_GROUPING_RANGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroupingRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRelativeDescriptionType(&self) -> windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelativeDescriptionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetRelativeDescription(&self, propvar1: *const super::propidlbase::PROPVARIANT, propvar2: *const super::propidlbase::PROPVARIANT, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRelativeDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar1), core::mem::transmute(propvar2), ppszdesc1 as _, ppszdesc2 as _) }
    }
    pub unsafe fn GetSortDescription(&self) -> windows_core::Result<PROPDESC_SORTDESCRIPTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSortDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSortDescriptionLabel(&self, fdescending: bool) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSortDescriptionLabel)(windows_core::Interface::as_raw(self), fdescending.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAggregationType(&self) -> windows_core::Result<PROPDESC_AGGREGATION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAggregationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "structuredquerycondition")]
    pub unsafe fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::structuredquerycondition::CONDITION_OPERATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConditionType)(windows_core::Interface::as_raw(self), pcontype as _, popdefault as _) }
    }
    pub unsafe fn GetEnumTypeList<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetEnumTypeList)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CoerceToCanonicalValue(&self, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CoerceToCanonicalValue)(windows_core::Interface::as_raw(self), core::mem::transmute(ppropvar)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FormatForDisplay(&self, propvar: *const super::propidlbase::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatForDisplay)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar), pdfflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsValueCanonical(&self, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsValueCanonical)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub GetPropertyKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyKey: usize,
    pub GetCanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetPropertyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyType: usize,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetEditInvitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, PROPDESC_TYPE_FLAGS, *mut PROPDESC_TYPE_FLAGS) -> windows_core::HRESULT,
    pub GetViewFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_VIEW_FLAGS) -> windows_core::HRESULT,
    pub GetDefaultColumnWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_DISPLAYTYPE) -> windows_core::HRESULT,
    #[cfg(feature = "shtypes")]
    pub GetColumnState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::shtypes::SHCOLSTATEF) -> windows_core::HRESULT,
    #[cfg(not(feature = "shtypes"))]
    GetColumnState: usize,
    pub GetGroupingRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_GROUPING_RANGE) -> windows_core::HRESULT,
    pub GetRelativeDescriptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetRelativeDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *const super::propidlbase::PROPVARIANT, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetRelativeDescription: usize,
    pub GetSortDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_SORTDESCRIPTION) -> windows_core::HRESULT,
    pub GetSortDescriptionLabel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetAggregationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_AGGREGATION_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "structuredquerycondition")]
    pub GetConditionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_CONDITION_TYPE, *mut super::structuredquerycondition::CONDITION_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "structuredquerycondition"))]
    GetConditionType: usize,
    pub GetEnumTypeList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub CoerceToCanonicalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    CoerceToCanonicalValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub FormatForDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, PROPDESC_FORMAT_FLAGS, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub IsValueCanonical: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    IsValueCanonical: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyDescription_Impl: windows_core::IUnknownImpl {
    fn GetPropertyKey(&self, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetCanonicalName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPropertyType(&self) -> windows_core::Result<super::wtypes::VARTYPE>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEditInvitation(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> windows_core::Result<PROPDESC_TYPE_FLAGS>;
    fn GetViewFlags(&self) -> windows_core::Result<PROPDESC_VIEW_FLAGS>;
    fn GetDefaultColumnWidth(&self) -> windows_core::Result<u32>;
    fn GetDisplayType(&self) -> windows_core::Result<PROPDESC_DISPLAYTYPE>;
    fn GetColumnState(&self) -> windows_core::Result<super::shtypes::SHCOLSTATEF>;
    fn GetGroupingRange(&self) -> windows_core::Result<PROPDESC_GROUPING_RANGE>;
    fn GetRelativeDescriptionType(&self) -> windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE>;
    fn GetRelativeDescription(&self, propvar1: *const super::propidlbase::PROPVARIANT, propvar2: *const super::propidlbase::PROPVARIANT, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetSortDescription(&self) -> windows_core::Result<PROPDESC_SORTDESCRIPTION>;
    fn GetSortDescriptionLabel(&self, fdescending: windows_core::BOOL) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAggregationType(&self) -> windows_core::Result<PROPDESC_AGGREGATION_TYPE>;
    fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::structuredquerycondition::CONDITION_OPERATION) -> windows_core::Result<()>;
    fn GetEnumTypeList(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CoerceToCanonicalValue(&self, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn FormatForDisplay(&self, propvar: *const super::propidlbase::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>;
    fn IsValueCanonical(&self, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyDescription_Vtbl {
    pub const fn new<Identity: IPropertyDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyKey<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::GetPropertyKey(this, core::mem::transmute_copy(&pkey)).into()
            }
        }
        unsafe extern "system" fn GetCanonicalName<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetCanonicalName(this) {
                    Ok(ok__) => {
                        ppszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvartype: *mut super::wtypes::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetPropertyType(this) {
                    Ok(ok__) => {
                        pvartype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        ppszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEditInvitation<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszinvite: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetEditInvitation(this) {
                    Ok(ok__) => {
                        ppszinvite.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetTypeFlags(this, core::mem::transmute_copy(&mask)) {
                    Ok(ok__) => {
                        ppdtflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetViewFlags<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetViewFlags(this) {
                    Ok(ok__) => {
                        ppdvflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcxchars: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetDefaultColumnWidth(this) {
                    Ok(ok__) => {
                        pcxchars.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetDisplayType(this) {
                    Ok(ok__) => {
                        pdisplaytype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnState<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsflags: *mut super::shtypes::SHCOLSTATEF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetColumnState(this) {
                    Ok(ok__) => {
                        pcsflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGroupingRange<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetGroupingRange(this) {
                    Ok(ok__) => {
                        pgr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetRelativeDescriptionType(this) {
                    Ok(ok__) => {
                        prdt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRelativeDescription<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar1: *const super::propidlbase::PROPVARIANT, propvar2: *const super::propidlbase::PROPVARIANT, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::GetRelativeDescription(this, core::mem::transmute_copy(&propvar1), core::mem::transmute_copy(&propvar2), core::mem::transmute_copy(&ppszdesc1), core::mem::transmute_copy(&ppszdesc2)).into()
            }
        }
        unsafe extern "system" fn GetSortDescription<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetSortDescription(this) {
                    Ok(ok__) => {
                        psd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdescending: windows_core::BOOL, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetSortDescriptionLabel(this, core::mem::transmute_copy(&fdescending)) {
                    Ok(ok__) => {
                        ppszdescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAggregationType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::GetAggregationType(this) {
                    Ok(ok__) => {
                        paggtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConditionType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::structuredquerycondition::CONDITION_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::GetConditionType(this, core::mem::transmute_copy(&pcontype), core::mem::transmute_copy(&popdefault)).into()
            }
        }
        unsafe extern "system" fn GetEnumTypeList<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::GetEnumTypeList(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::CoerceToCanonicalValue(this, core::mem::transmute_copy(&ppropvar)).into()
            }
        }
        unsafe extern "system" fn FormatForDisplay<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const super::propidlbase::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription_Impl::FormatForDisplay(this, core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdfflags)) {
                    Ok(ok__) => {
                        ppszdisplay.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsValueCanonical<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescription_Impl::IsValueCanonical(this, core::mem::transmute_copy(&propvar)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyKey: GetPropertyKey::<Identity, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetEditInvitation: GetEditInvitation::<Identity, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, OFFSET>,
            GetViewFlags: GetViewFlags::<Identity, OFFSET>,
            GetDefaultColumnWidth: GetDefaultColumnWidth::<Identity, OFFSET>,
            GetDisplayType: GetDisplayType::<Identity, OFFSET>,
            GetColumnState: GetColumnState::<Identity, OFFSET>,
            GetGroupingRange: GetGroupingRange::<Identity, OFFSET>,
            GetRelativeDescriptionType: GetRelativeDescriptionType::<Identity, OFFSET>,
            GetRelativeDescription: GetRelativeDescription::<Identity, OFFSET>,
            GetSortDescription: GetSortDescription::<Identity, OFFSET>,
            GetSortDescriptionLabel: GetSortDescriptionLabel::<Identity, OFFSET>,
            GetAggregationType: GetAggregationType::<Identity, OFFSET>,
            GetConditionType: GetConditionType::<Identity, OFFSET>,
            GetEnumTypeList: GetEnumTypeList::<Identity, OFFSET>,
            CoerceToCanonicalValue: CoerceToCanonicalValue::<Identity, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, OFFSET>,
            IsValueCanonical: IsValueCanonical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyDescription {}
windows_core::imp::define_interface!(IPropertyDescription2, IPropertyDescription2_Vtbl, 0x57d2eded_5062_400e_b107_5dae79fe57a6);
impl core::ops::Deref for IPropertyDescription2 {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescription2, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescription2 {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetImageReferenceForValue(&self, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageReferenceForValue)(windows_core::Interface::as_raw(self), core::mem::transmute(propvar), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescription2_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetImageReferenceForValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetImageReferenceForValue: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyDescription2_Impl: IPropertyDescription_Impl {
    fn GetImageReferenceForValue(&self, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyDescription2_Vtbl {
    pub const fn new<Identity: IPropertyDescription2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageReferenceForValue<Identity: IPropertyDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const super::propidlbase::PROPVARIANT, ppszimageres: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescription2_Impl::GetImageReferenceForValue(this, core::mem::transmute_copy(&propvar)) {
                    Ok(ok__) => {
                        ppszimageres.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(), GetImageReferenceForValue: GetImageReferenceForValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescription2 as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyDescription2 {}
windows_core::imp::define_interface!(IPropertyDescriptionAliasInfo, IPropertyDescriptionAliasInfo_Vtbl, 0xf67104fc_2af9_46fd_b32d_243c1404f3d1);
impl core::ops::Deref for IPropertyDescriptionAliasInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionAliasInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionAliasInfo {
    pub unsafe fn GetSortByAlias<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetSortByAlias)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetAdditionalSortByAliases<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAdditionalSortByAliases)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionAliasInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSortByAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdditionalSortByAliases: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyDescriptionAliasInfo_Impl: IPropertyDescription_Impl {
    fn GetSortByAlias(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdditionalSortByAliases(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyDescriptionAliasInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSortByAlias<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescriptionAliasInfo_Impl::GetSortByAlias(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescriptionAliasInfo_Impl::GetAdditionalSortByAliases(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(),
            GetSortByAlias: GetSortByAlias::<Identity, OFFSET>,
            GetAdditionalSortByAliases: GetAdditionalSortByAliases::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionAliasInfo as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyDescriptionAliasInfo {}
windows_core::imp::define_interface!(IPropertyDescriptionList, IPropertyDescriptionList_Vtbl, 0x1f9fc1d0_c39b_4b26_817f_011967d3440e);
windows_core::imp::interface_hierarchy!(IPropertyDescriptionList, windows_core::IUnknown);
impl IPropertyDescriptionList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt<T>(&self, ielem: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ielem, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPropertyDescriptionList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ielem: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IPropertyDescriptionList_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescriptionList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ielem: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescriptionList_Impl::GetAt(this, core::mem::transmute_copy(&ielem), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPropertyDescriptionList {}
windows_core::imp::define_interface!(IPropertyDescriptionRelatedPropertyInfo, IPropertyDescriptionRelatedPropertyInfo_Vtbl, 0x507393f4_2a3d_4a60_b59e_d9c75716c2dd);
impl core::ops::Deref for IPropertyDescriptionRelatedPropertyInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionRelatedPropertyInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionRelatedPropertyInfo {
    pub unsafe fn GetRelatedProperty<P0, T>(&self, pszrelationshipname: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRelatedProperty)(windows_core::Interface::as_raw(self), pszrelationshipname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetRelatedProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyDescriptionRelatedPropertyInfo_Impl: IPropertyDescription_Impl {
    fn GetRelatedProperty(&self, pszrelationshipname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRelatedProperty<Identity: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrelationshipname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyDescriptionRelatedPropertyInfo_Impl::GetRelatedProperty(this, core::mem::transmute(&pszrelationshipname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(), GetRelatedProperty: GetRelatedProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionRelatedPropertyInfo as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyDescriptionRelatedPropertyInfo {}
windows_core::imp::define_interface!(IPropertyDescriptionSearchInfo, IPropertyDescriptionSearchInfo_Vtbl, 0x078f91bd_29a2_440f_924e_46a291524520);
impl core::ops::Deref for IPropertyDescriptionSearchInfo {
    type Target = IPropertyDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyDescriptionSearchInfo, windows_core::IUnknown, IPropertyDescription);
impl IPropertyDescriptionSearchInfo {
    pub unsafe fn GetSearchInfoFlags(&self) -> windows_core::Result<PROPDESC_SEARCHINFO_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSearchInfoFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetColumnIndexType(&self) -> windows_core::Result<PROPDESC_COLUMNINDEX_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnIndexType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProjectionString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProjectionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyDescriptionSearchInfo_Vtbl {
    pub base__: IPropertyDescription_Vtbl,
    pub GetSearchInfoFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_SEARCHINFO_FLAGS) -> windows_core::HRESULT,
    pub GetColumnIndexType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPDESC_COLUMNINDEX_TYPE) -> windows_core::HRESULT,
    pub GetProjectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyDescriptionSearchInfo_Impl: IPropertyDescription_Impl {
    fn GetSearchInfoFlags(&self) -> windows_core::Result<PROPDESC_SEARCHINFO_FLAGS>;
    fn GetColumnIndexType(&self) -> windows_core::Result<PROPDESC_COLUMNINDEX_TYPE>;
    fn GetProjectionString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMaxSize(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyDescriptionSearchInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSearchInfoFlags<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescriptionSearchInfo_Impl::GetSearchInfoFlags(this) {
                    Ok(ok__) => {
                        ppdsiflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnIndexType<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescriptionSearchInfo_Impl::GetColumnIndexType(this) {
                    Ok(ok__) => {
                        ppdcitype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProjectionString<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszprojection: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescriptionSearchInfo_Impl::GetProjectionString(this) {
                    Ok(ok__) => {
                        ppszprojection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxSize<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyDescriptionSearchInfo_Impl::GetMaxSize(this) {
                    Ok(ok__) => {
                        pcbmaxsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(),
            GetSearchInfoFlags: GetSearchInfoFlags::<Identity, OFFSET>,
            GetColumnIndexType: GetColumnIndexType::<Identity, OFFSET>,
            GetProjectionString: GetProjectionString::<Identity, OFFSET>,
            GetMaxSize: GetMaxSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionSearchInfo as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shtypes", feature = "structuredquerycondition", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyDescriptionSearchInfo {}
windows_core::imp::define_interface!(IPropertyEnumType, IPropertyEnumType_Vtbl, 0x11e1fbf9_2d56_4a6b_8db3_7cd193a471f2);
windows_core::imp::interface_hierarchy!(IPropertyEnumType, windows_core::IUnknown);
impl IPropertyEnumType {
    pub unsafe fn GetEnumType(&self) -> windows_core::Result<PROPENUMTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetRangeMinValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeMinValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetRangeSetValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeSetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayText(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnumType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPENUMTYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetRangeMinValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetRangeMinValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetRangeSetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetRangeSetValue: usize,
    pub GetDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyEnumType_Impl: windows_core::IUnknownImpl {
    fn GetEnumType(&self) -> windows_core::Result<PROPENUMTYPE>;
    fn GetValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetRangeMinValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetRangeSetValue(&self) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetDisplayText(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyEnumType_Vtbl {
    pub const fn new<Identity: IPropertyEnumType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnumType<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType_Impl::GetEnumType(this) {
                    Ok(ok__) => {
                        penumtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType_Impl::GetValue(this) {
                    Ok(ok__) => {
                        ppropvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRangeMinValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvarmin: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType_Impl::GetRangeMinValue(this) {
                    Ok(ok__) => {
                        ppropvarmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRangeSetValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvarset: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType_Impl::GetRangeSetValue(this) {
                    Ok(ok__) => {
                        ppropvarset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayText<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType_Impl::GetDisplayText(this) {
                    Ok(ok__) => {
                        ppszdisplay.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnumType: GetEnumType::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetRangeMinValue: GetRangeMinValue::<Identity, OFFSET>,
            GetRangeSetValue: GetRangeSetValue::<Identity, OFFSET>,
            GetDisplayText: GetDisplayText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyEnumType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyEnumType {}
windows_core::imp::define_interface!(IPropertyEnumType2, IPropertyEnumType2_Vtbl, 0x9b6e051c_5ddd_4321_9070_fe2acb55e794);
impl core::ops::Deref for IPropertyEnumType2 {
    type Target = IPropertyEnumType;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyEnumType2, windows_core::IUnknown, IPropertyEnumType);
impl IPropertyEnumType2 {
    pub unsafe fn GetImageReference(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumType2_Vtbl {
    pub base__: IPropertyEnumType_Vtbl,
    pub GetImageReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyEnumType2_Impl: IPropertyEnumType_Impl {
    fn GetImageReference(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyEnumType2_Vtbl {
    pub const fn new<Identity: IPropertyEnumType2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageReference<Identity: IPropertyEnumType2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszimageres: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumType2_Impl::GetImageReference(this) {
                    Ok(ok__) => {
                        ppszimageres.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IPropertyEnumType_Vtbl::new::<Identity, OFFSET>(), GetImageReference: GetImageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyEnumType2 as windows_core::Interface>::IID || iid == &<IPropertyEnumType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyEnumType2 {}
windows_core::imp::define_interface!(IPropertyEnumTypeList, IPropertyEnumTypeList_Vtbl, 0xa99400f4_3d84_4557_94ba_1242fb2cc9a6);
windows_core::imp::interface_hierarchy!(IPropertyEnumTypeList, windows_core::IUnknown);
impl IPropertyEnumTypeList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt<T>(&self, itype: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), itype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetConditionAt<T>(&self, nindex: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetConditionAt)(windows_core::Interface::as_raw(self), nindex, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindMatchingIndex(&self, propvarcmp: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindMatchingIndex)(windows_core::Interface::as_raw(self), core::mem::transmute(propvarcmp), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyEnumTypeList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConditionAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub FindMatchingIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    FindMatchingIndex: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyEnumTypeList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, itype: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetConditionAt(&self, nindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindMatchingIndex(&self, propvarcmp: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyEnumTypeList_Vtbl {
    pub const fn new<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumTypeList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pctypes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itype: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyEnumTypeList_Impl::GetAt(this, core::mem::transmute_copy(&itype), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetConditionAt<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyEnumTypeList_Impl::GetConditionAt(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn FindMatchingIndex<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvarcmp: *const super::propidlbase::PROPVARIANT, pnindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyEnumTypeList_Impl::FindMatchingIndex(this, core::mem::transmute_copy(&propvarcmp)) {
                    Ok(ok__) => {
                        pnindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetConditionAt: GetConditionAt::<Identity, OFFSET>,
            FindMatchingIndex: FindMatchingIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyEnumTypeList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyEnumTypeList {}
windows_core::imp::define_interface!(IPropertyStore, IPropertyStore_Vtbl, 0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
windows_core::imp::interface_hierarchy!(IPropertyStore, windows_core::IUnknown);
impl IPropertyStore {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetAt(&self, iprop: u32, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), iprop, pkey as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar)) }
    }
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetAt: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyStore_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, iprop: u32, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn SetValue(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyStore_Vtbl {
    pub const fn new<Identity: IPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cprops: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStore_Impl::GetCount(this) {
                    Ok(ok__) => {
                        cprops.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprop: u32, pkey: *mut super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStore_Impl::GetAt(this, core::mem::transmute_copy(&iprop), core::mem::transmute_copy(&pkey)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pv: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStore_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStore_Impl::SetValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStore_Impl::Commit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyStore {}
windows_core::imp::define_interface!(IPropertyStoreCache, IPropertyStoreCache_Vtbl, 0x3017056d_9a91_4e90_937d_746c72abbf4f);
impl core::ops::Deref for IPropertyStoreCache {
    type Target = IPropertyStore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyStoreCache, windows_core::IUnknown, IPropertyStore);
impl IPropertyStoreCache {
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetState(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<PSC_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValueAndState(&self, key: *const super::wtypes::PROPERTYKEY, ppropvar: *mut super::propidlbase::PROPVARIANT, pstate: *mut PSC_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValueAndState)(windows_core::Interface::as_raw(self), key, core::mem::transmute(ppropvar), pstate as _) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetState(&self, key: *const super::wtypes::PROPERTYKEY, state: PSC_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), key, state) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValueAndState(&self, key: *const super::wtypes::PROPERTYKEY, ppropvar: *const super::propidlbase::PROPVARIANT, state: PSC_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValueAndState)(windows_core::Interface::as_raw(self), key, core::mem::transmute(ppropvar), state) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCache_Vtbl {
    pub base__: IPropertyStore_Vtbl,
    #[cfg(feature = "wtypes")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut PSC_STATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetState: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValueAndState: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::propidlbase::PROPVARIANT, *mut PSC_STATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValueAndState: usize,
    #[cfg(feature = "wtypes")]
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, PSC_STATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetState: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValueAndState: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PSC_STATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetValueAndState: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertyStoreCache_Impl: IPropertyStore_Impl {
    fn GetState(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<PSC_STATE>;
    fn GetValueAndState(&self, key: *const super::wtypes::PROPERTYKEY, ppropvar: *mut super::propidlbase::PROPVARIANT, pstate: *mut PSC_STATE) -> windows_core::Result<()>;
    fn SetState(&self, key: *const super::wtypes::PROPERTYKEY, state: PSC_STATE) -> windows_core::Result<()>;
    fn SetValueAndState(&self, key: *const super::wtypes::PROPERTYKEY, ppropvar: *const super::propidlbase::PROPVARIANT, state: PSC_STATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertyStoreCache_Vtbl {
    pub const fn new<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pstate: *mut PSC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCache_Impl::GetState(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueAndState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, ppropvar: *mut super::propidlbase::PROPVARIANT, pstate: *mut PSC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCache_Impl::GetValueAndState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn SetState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, state: PSC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCache_Impl::SetState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn SetValueAndState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, ppropvar: *const super::propidlbase::PROPVARIANT, state: PSC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCache_Impl::SetValueAndState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&state)).into()
            }
        }
        Self {
            base__: IPropertyStore_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetValueAndState: GetValueAndState::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            SetValueAndState: SetValueAndState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCache as windows_core::Interface>::IID || iid == &<IPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertyStoreCache {}
windows_core::imp::define_interface!(IPropertyStoreCapabilities, IPropertyStoreCapabilities_Vtbl, 0xc8e2d566_186e_4d49_bf41_6909ead56acc);
windows_core::imp::interface_hierarchy!(IPropertyStoreCapabilities, windows_core::IUnknown);
impl IPropertyStoreCapabilities {
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPropertyWritable(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsPropertyWritable)(windows_core::Interface::as_raw(self), key) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCapabilities_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsPropertyWritable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPropertyWritable: usize,
}
#[cfg(feature = "wtypes")]
pub trait IPropertyStoreCapabilities_Impl: windows_core::IUnknownImpl {
    fn IsPropertyWritable(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IPropertyStoreCapabilities_Vtbl {
    pub const fn new<Identity: IPropertyStoreCapabilities_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPropertyWritable<Identity: IPropertyStoreCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCapabilities_Impl::IsPropertyWritable(this, core::mem::transmute_copy(&key)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsPropertyWritable: IsPropertyWritable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IPropertyStoreCapabilities {}
windows_core::imp::define_interface!(IPropertyStoreFactory, IPropertyStoreFactory_Vtbl, 0xbc110b6d_57e8_4148_a9c6_91015ab2f3a5);
windows_core::imp::interface_hierarchy!(IPropertyStoreFactory, windows_core::IUnknown);
impl IPropertyStoreFactory {
    pub unsafe fn GetPropertyStore<P1, T>(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStore)(windows_core::Interface::as_raw(self), flags, punkfactory.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyStoreForKeys<T>(&self, rgkeys: *const super::wtypes::PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStoreForKeys)(windows_core::Interface::as_raw(self), rgkeys, ckeys, flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, GETPROPERTYSTOREFLAGS, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetPropertyStoreForKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, u32, GETPROPERTYSTOREFLAGS, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyStoreForKeys: usize,
}
#[cfg(feature = "wtypes")]
pub trait IPropertyStoreFactory_Impl: windows_core::IUnknownImpl {
    fn GetPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyStoreForKeys(&self, rgkeys: *const super::wtypes::PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyStore<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreFactory_Impl::GetPropertyStore(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&punkfactory), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgkeys: *const super::wtypes::PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreFactory_Impl::GetPropertyStoreForKeys(this, core::mem::transmute_copy(&rgkeys), core::mem::transmute_copy(&ckeys), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyStore: GetPropertyStore::<Identity, OFFSET>,
            GetPropertyStoreForKeys: GetPropertyStoreForKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IPropertyStoreFactory {}
windows_core::imp::define_interface!(IPropertySystem, IPropertySystem_Vtbl, 0xca724e8a_c3e6_442b_88a4_6fb0db8035a3);
windows_core::imp::interface_hierarchy!(IPropertySystem, windows_core::IUnknown);
impl IPropertySystem {
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetPropertyDescription<T>(&self, propkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPropertyDescription)(windows_core::Interface::as_raw(self), propkey, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetPropertyDescriptionByName<P0, T>(&self, pszcanonicalname: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPropertyDescriptionByName)(windows_core::Interface::as_raw(self), pszcanonicalname.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetPropertyDescriptionListFromString<P0, T>(&self, pszproplist: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPropertyDescriptionListFromString)(windows_core::Interface::as_raw(self), pszproplist.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EnumeratePropertyDescriptions<T>(&self, filteron: PROPDESC_ENUMFILTER) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumeratePropertyDescriptions)(windows_core::Interface::as_raw(self), filteron, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FormatForDisplay(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FormatForDisplay)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar), pdff, core::mem::transmute(psztext), cchtext) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FormatForDisplayAlloc(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatForDisplayAlloc)(windows_core::Interface::as_raw(self), key, core::mem::transmute(propvar), pdff, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterPropertySchema<P0>(&self, pszpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterPropertySchema)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
    pub unsafe fn UnregisterPropertySchema<P0>(&self, pszpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterPropertySchema)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
    pub unsafe fn RefreshPropertySchema(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshPropertySchema)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub GetPropertyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetPropertyDescription: usize,
    pub GetPropertyDescriptionByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyDescriptionListFromString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumeratePropertyDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, PROPDESC_ENUMFILTER, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub FormatForDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PROPDESC_FORMAT_FLAGS, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    FormatForDisplay: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub FormatForDisplayAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PROPDESC_FORMAT_FLAGS, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    FormatForDisplayAlloc: usize,
    pub RegisterPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RefreshPropertySchema: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPropertySystem_Impl: windows_core::IUnknownImpl {
    fn GetPropertyDescription(&self, propkey: *const super::wtypes::PROPERTYKEY, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyDescriptionByName(&self, pszcanonicalname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyDescriptionListFromString(&self, pszproplist: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumeratePropertyDescriptions(&self, filteron: PROPDESC_ENUMFILTER, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FormatForDisplay(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn FormatForDisplayAlloc(&self, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>;
    fn RegisterPropertySchema(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterPropertySchema(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RefreshPropertySchema(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IPropertySystem_Vtbl {
    pub const fn new<Identity: IPropertySystem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyDescription<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::wtypes::PROPERTYKEY, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::GetPropertyDescription(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcanonicalname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::GetPropertyDescriptionByName(this, core::mem::transmute(&pszcanonicalname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszproplist: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::GetPropertyDescriptionListFromString(this, core::mem::transmute(&pszproplist), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::EnumeratePropertyDescriptions(this, core::mem::transmute_copy(&filteron), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn FormatForDisplay<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::FormatForDisplay(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdff), core::mem::transmute_copy(&psztext), core::mem::transmute_copy(&cchtext)).into()
            }
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, propvar: *const super::propidlbase::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySystem_Impl::FormatForDisplayAlloc(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdff)) {
                    Ok(ok__) => {
                        ppszdisplay.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::RegisterPropertySchema(this, core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn UnregisterPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::UnregisterPropertySchema(this, core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn RefreshPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystem_Impl::RefreshPropertySchema(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyDescription: GetPropertyDescription::<Identity, OFFSET>,
            GetPropertyDescriptionByName: GetPropertyDescriptionByName::<Identity, OFFSET>,
            GetPropertyDescriptionListFromString: GetPropertyDescriptionListFromString::<Identity, OFFSET>,
            EnumeratePropertyDescriptions: EnumeratePropertyDescriptions::<Identity, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, OFFSET>,
            FormatForDisplayAlloc: FormatForDisplayAlloc::<Identity, OFFSET>,
            RegisterPropertySchema: RegisterPropertySchema::<Identity, OFFSET>,
            UnregisterPropertySchema: UnregisterPropertySchema::<Identity, OFFSET>,
            RefreshPropertySchema: RefreshPropertySchema::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySystem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPropertySystem {}
windows_core::imp::define_interface!(IPropertySystemChangeNotify, IPropertySystemChangeNotify_Vtbl, 0xfa955fd9_38be_4879_a6ce_824cf52d609f);
windows_core::imp::interface_hierarchy!(IPropertySystemChangeNotify, windows_core::IUnknown);
impl IPropertySystemChangeNotify {
    pub unsafe fn SchemaRefreshed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SchemaRefreshed)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySystemChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SchemaRefreshed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPropertySystemChangeNotify_Impl: windows_core::IUnknownImpl {
    fn SchemaRefreshed(&self) -> windows_core::Result<()>;
}
impl IPropertySystemChangeNotify_Vtbl {
    pub const fn new<Identity: IPropertySystemChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SchemaRefreshed<Identity: IPropertySystemChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySystemChangeNotify_Impl::SchemaRefreshed(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SchemaRefreshed: SchemaRefreshed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySystemChangeNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPropertySystemChangeNotify {}
pub const InMemoryPropertyStore: windows_core::GUID = windows_core::GUID::from_u128(0x9a02e012_6303_4e1e_b9a1_630f802592c5);
pub const InMemoryPropertyStoreMarshalByValue: windows_core::GUID = windows_core::GUID::from_u128(0xd4ca0e2d_6da7_4b75_a97c_5f306f0eaedc);
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
pub const PropertySystem: windows_core::GUID = windows_core::GUID::from_u128(0xb8967f85_58ae_4f46_9fb2_5d7904798f4b);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIALIZEDPROPSTORAGE(pub u8);
