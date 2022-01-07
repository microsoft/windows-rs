pub trait ICreateObjectImpl: Sized {
    fn CreateObject();
}
impl ::windows::core::RuntimeName for ICreateObject {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.ICreateObject";
}
impl ICreateObjectVtbl {
    pub const fn new<Impl: ICreateObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICreateObjectVtbl {
        unsafe extern "system" fn CreateObject<Impl: ICreateObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateObject(
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICreateObject>, base.5, CreateObject::<Impl, OFFSET>)
    }
}
pub trait IDelayedPropertyStoreFactoryImpl: Sized + IPropertyStoreFactoryImpl {
    fn GetDelayedPropertyStore();
}
impl ::windows::core::RuntimeName for IDelayedPropertyStoreFactory {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IDelayedPropertyStoreFactory";
}
impl IDelayedPropertyStoreFactoryVtbl {
    pub const fn new<Impl: IDelayedPropertyStoreFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDelayedPropertyStoreFactoryVtbl {
        unsafe extern "system" fn GetDelayedPropertyStore<Impl: IDelayedPropertyStoreFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDelayedPropertyStore(flags, dwstoreid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDelayedPropertyStoreFactory>, base.5, GetDelayedPropertyStore::<Impl, OFFSET>)
    }
}
pub trait IInitializeWithFileImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IInitializeWithFile {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IInitializeWithFile";
}
impl IInitializeWithFileVtbl {
    pub const fn new<Impl: IInitializeWithFileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInitializeWithFileVtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilepath: super::super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszfilepath as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), grfmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInitializeWithFile>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IInitializeWithStreamImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IInitializeWithStream {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IInitializeWithStream";
}
impl IInitializeWithStreamVtbl {
    pub const fn new<Impl: IInitializeWithStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInitializeWithStreamVtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), grfmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInitializeWithStream>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait INamedPropertyStoreImpl: Sized {
    fn GetNamedValue();
    fn SetNamedValue();
    fn GetNameCount();
    fn GetNameAt();
}
impl ::windows::core::RuntimeName for INamedPropertyStore {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.INamedPropertyStore";
}
impl INamedPropertyStoreVtbl {
    pub const fn new<Impl: INamedPropertyStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INamedPropertyStoreVtbl {
        unsafe extern "system" fn GetNamedValue<Impl: INamedPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamedValue(&*(&pszname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedValue<Impl: INamedPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNamedValue(&*(&pszname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCount<Impl: INamedPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNameCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameAt<Impl: INamedPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNameAt(iprop, ::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INamedPropertyStore>, base.5, GetNamedValue::<Impl, OFFSET>, SetNamedValue::<Impl, OFFSET>, GetNameCount::<Impl, OFFSET>, GetNameAt::<Impl, OFFSET>)
    }
}
pub trait IObjectWithPropertyKeyImpl: Sized {
    fn SetPropertyKey();
    fn GetPropertyKey();
}
impl ::windows::core::RuntimeName for IObjectWithPropertyKey {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IObjectWithPropertyKey";
}
impl IObjectWithPropertyKeyVtbl {
    pub const fn new<Impl: IObjectWithPropertyKeyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectWithPropertyKeyVtbl {
        unsafe extern "system" fn SetPropertyKey<Impl: IObjectWithPropertyKeyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyKey(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyKey<Impl: IObjectWithPropertyKeyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyKey(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectWithPropertyKey>, base.5, SetPropertyKey::<Impl, OFFSET>, GetPropertyKey::<Impl, OFFSET>)
    }
}
pub trait IPersistSerializedPropStorageImpl: Sized {
    fn SetFlags();
    fn SetPropertyStorage();
    fn GetPropertyStorage();
}
impl ::windows::core::RuntimeName for IPersistSerializedPropStorage {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPersistSerializedPropStorage";
}
impl IPersistSerializedPropStorageVtbl {
    pub const fn new<Impl: IPersistSerializedPropStorageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPersistSerializedPropStorageVtbl {
        unsafe extern "system" fn SetFlags<Impl: IPersistSerializedPropStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyStorage<Impl: IPersistSerializedPropStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyStorage(&*(&psps as *const <SERIALIZEDPROPSTORAGE as ::windows::core::Abi>::Abi as *const <SERIALIZEDPROPSTORAGE as ::windows::core::DefaultType>::DefaultType), cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStorage<Impl: IPersistSerializedPropStorageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStorage(::core::mem::transmute_copy(&ppsps), ::core::mem::transmute_copy(&pcb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPersistSerializedPropStorage>, base.5, SetFlags::<Impl, OFFSET>, SetPropertyStorage::<Impl, OFFSET>, GetPropertyStorage::<Impl, OFFSET>)
    }
}
pub trait IPersistSerializedPropStorage2Impl: Sized + IPersistSerializedPropStorageImpl {
    fn GetPropertyStorageSize();
    fn GetPropertyStorageBuffer();
}
impl ::windows::core::RuntimeName for IPersistSerializedPropStorage2 {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPersistSerializedPropStorage2";
}
impl IPersistSerializedPropStorage2Vtbl {
    pub const fn new<Impl: IPersistSerializedPropStorage2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPersistSerializedPropStorage2Vtbl {
        unsafe extern "system" fn GetPropertyStorageSize<Impl: IPersistSerializedPropStorage2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStorageSize(::core::mem::transmute_copy(&pcb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Impl: IPersistSerializedPropStorage2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStorageBuffer(::core::mem::transmute_copy(&psps), cb, ::core::mem::transmute_copy(&pcbwritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPersistSerializedPropStorage2>, base.5, GetPropertyStorageSize::<Impl, OFFSET>, GetPropertyStorageBuffer::<Impl, OFFSET>)
    }
}
pub trait IPropertyChangeImpl: Sized + IObjectWithPropertyKeyImpl {
    fn ApplyToPropVariant();
}
impl ::windows::core::RuntimeName for IPropertyChange {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyChange";
}
impl IPropertyChangeVtbl {
    pub const fn new<Impl: IPropertyChangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyChangeVtbl {
        unsafe extern "system" fn ApplyToPropVariant<Impl: IPropertyChangeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplyToPropVariant(&*(&propvarin as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropvarout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyChange>, base.5, ApplyToPropVariant::<Impl, OFFSET>)
    }
}
pub trait IPropertyChangeArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn Append();
    fn AppendOrReplace();
    fn RemoveAt();
    fn IsKeyInArray();
}
impl ::windows::core::RuntimeName for IPropertyChangeArray {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyChangeArray";
}
impl IPropertyChangeArrayVtbl {
    pub const fn new<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyChangeArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(iindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertAt<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InsertAt(iindex, &*(&ppropchange as *const <IPropertyChange as ::windows::core::Abi>::Abi as *const <IPropertyChange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Append(&*(&ppropchange as *const <IPropertyChange as ::windows::core::Abi>::Abi as *const <IPropertyChange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendOrReplace<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendOrReplace(&*(&ppropchange as *const <IPropertyChange as ::windows::core::Abi>::Abi as *const <IPropertyChange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(iindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyInArray<Impl: IPropertyChangeArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsKeyInArray(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyChangeArray>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, InsertAt::<Impl, OFFSET>, Append::<Impl, OFFSET>, AppendOrReplace::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>, IsKeyInArray::<Impl, OFFSET>)
    }
}
pub trait IPropertyDescriptionImpl: Sized {
    fn GetPropertyKey();
    fn GetCanonicalName();
    fn GetPropertyType();
    fn GetDisplayName();
    fn GetEditInvitation();
    fn GetTypeFlags();
    fn GetViewFlags();
    fn GetDefaultColumnWidth();
    fn GetDisplayType();
    fn GetColumnState();
    fn GetGroupingRange();
    fn GetRelativeDescriptionType();
    fn GetRelativeDescription();
    fn GetSortDescription();
    fn GetSortDescriptionLabel();
    fn GetAggregationType();
    fn GetConditionType();
    fn GetEnumTypeList();
    fn CoerceToCanonicalValue();
    fn FormatForDisplay();
    fn IsValueCanonical();
}
impl ::windows::core::RuntimeName for IPropertyDescription {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescription";
}
impl IPropertyDescriptionVtbl {
    pub const fn new<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescriptionVtbl {
        unsafe extern "system" fn GetPropertyKey<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyKey(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCanonicalName(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyType<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyType(::core::mem::transmute_copy(&pvartype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEditInvitation<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEditInvitation(::core::mem::transmute_copy(&ppszinvite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTypeFlags(mask, ::core::mem::transmute_copy(&ppdtflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewFlags<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewFlags(::core::mem::transmute_copy(&ppdvflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultColumnWidth(::core::mem::transmute_copy(&pcxchars)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayType<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayType(::core::mem::transmute_copy(&pdisplaytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnState<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnState(::core::mem::transmute_copy(&pcsflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupingRange<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGroupingRange(::core::mem::transmute_copy(&pgr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelativeDescriptionType(::core::mem::transmute_copy(&prdt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescription<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelativeDescription(
                &*(&propvar1 as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&propvar2 as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppszdesc1),
                ::core::mem::transmute_copy(&ppszdesc2),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortDescription<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSortDescription(::core::mem::transmute_copy(&psd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSortDescriptionLabel(&*(&fdescending as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAggregationType<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAggregationType(::core::mem::transmute_copy(&paggtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConditionType<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConditionType(::core::mem::transmute_copy(&pcontype), ::core::mem::transmute_copy(&popdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumTypeList<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumTypeList(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoerceToCanonicalValue(&*(&ppropvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatForDisplay(&*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), pdfflags, ::core::mem::transmute_copy(&ppszdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValueCanonical<Impl: IPropertyDescriptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsValueCanonical(&*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPropertyDescription>,
            base.5,
            GetPropertyKey::<Impl, OFFSET>,
            GetCanonicalName::<Impl, OFFSET>,
            GetPropertyType::<Impl, OFFSET>,
            GetDisplayName::<Impl, OFFSET>,
            GetEditInvitation::<Impl, OFFSET>,
            GetTypeFlags::<Impl, OFFSET>,
            GetViewFlags::<Impl, OFFSET>,
            GetDefaultColumnWidth::<Impl, OFFSET>,
            GetDisplayType::<Impl, OFFSET>,
            GetColumnState::<Impl, OFFSET>,
            GetGroupingRange::<Impl, OFFSET>,
            GetRelativeDescriptionType::<Impl, OFFSET>,
            GetRelativeDescription::<Impl, OFFSET>,
            GetSortDescription::<Impl, OFFSET>,
            GetSortDescriptionLabel::<Impl, OFFSET>,
            GetAggregationType::<Impl, OFFSET>,
            GetConditionType::<Impl, OFFSET>,
            GetEnumTypeList::<Impl, OFFSET>,
            CoerceToCanonicalValue::<Impl, OFFSET>,
            FormatForDisplay::<Impl, OFFSET>,
            IsValueCanonical::<Impl, OFFSET>,
        )
    }
}
pub trait IPropertyDescription2Impl: Sized + IPropertyDescriptionImpl {
    fn GetImageReferenceForValue();
}
impl ::windows::core::RuntimeName for IPropertyDescription2 {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescription2";
}
impl IPropertyDescription2Vtbl {
    pub const fn new<Impl: IPropertyDescription2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescription2Vtbl {
        unsafe extern "system" fn GetImageReferenceForValue<Impl: IPropertyDescription2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageReferenceForValue(&*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszimageres)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyDescription2>, base.5, GetImageReferenceForValue::<Impl, OFFSET>)
    }
}
pub trait IPropertyDescriptionAliasInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSortByAlias();
    fn GetAdditionalSortByAliases();
}
impl ::windows::core::RuntimeName for IPropertyDescriptionAliasInfo {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescriptionAliasInfo";
}
impl IPropertyDescriptionAliasInfoVtbl {
    pub const fn new<Impl: IPropertyDescriptionAliasInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescriptionAliasInfoVtbl {
        unsafe extern "system" fn GetSortByAlias<Impl: IPropertyDescriptionAliasInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSortByAlias(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Impl: IPropertyDescriptionAliasInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdditionalSortByAliases(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyDescriptionAliasInfo>, base.5, GetSortByAlias::<Impl, OFFSET>, GetAdditionalSortByAliases::<Impl, OFFSET>)
    }
}
pub trait IPropertyDescriptionListImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for IPropertyDescriptionList {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescriptionList";
}
impl IPropertyDescriptionListVtbl {
    pub const fn new<Impl: IPropertyDescriptionListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescriptionListVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyDescriptionListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcelem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyDescriptionListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(ielem, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyDescriptionList>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
    }
}
pub trait IPropertyDescriptionRelatedPropertyInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetRelatedProperty();
}
impl ::windows::core::RuntimeName for IPropertyDescriptionRelatedPropertyInfo {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescriptionRelatedPropertyInfo";
}
impl IPropertyDescriptionRelatedPropertyInfoVtbl {
    pub const fn new<Impl: IPropertyDescriptionRelatedPropertyInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescriptionRelatedPropertyInfoVtbl {
        unsafe extern "system" fn GetRelatedProperty<Impl: IPropertyDescriptionRelatedPropertyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrelationshipname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelatedProperty(&*(&pszrelationshipname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyDescriptionRelatedPropertyInfo>, base.5, GetRelatedProperty::<Impl, OFFSET>)
    }
}
pub trait IPropertyDescriptionSearchInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSearchInfoFlags();
    fn GetColumnIndexType();
    fn GetProjectionString();
    fn GetMaxSize();
}
impl ::windows::core::RuntimeName for IPropertyDescriptionSearchInfo {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyDescriptionSearchInfo";
}
impl IPropertyDescriptionSearchInfoVtbl {
    pub const fn new<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyDescriptionSearchInfoVtbl {
        unsafe extern "system" fn GetSearchInfoFlags<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSearchInfoFlags(::core::mem::transmute_copy(&ppdsiflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnIndexType<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetColumnIndexType(::core::mem::transmute_copy(&ppdcitype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProjectionString<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszprojection: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProjectionString(::core::mem::transmute_copy(&ppszprojection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxSize<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxSize(::core::mem::transmute_copy(&pcbmaxsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyDescriptionSearchInfo>, base.5, GetSearchInfoFlags::<Impl, OFFSET>, GetColumnIndexType::<Impl, OFFSET>, GetProjectionString::<Impl, OFFSET>, GetMaxSize::<Impl, OFFSET>)
    }
}
pub trait IPropertyEnumTypeImpl: Sized {
    fn GetEnumType();
    fn GetValue();
    fn GetRangeMinValue();
    fn GetRangeSetValue();
    fn GetDisplayText();
}
impl ::windows::core::RuntimeName for IPropertyEnumType {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyEnumType";
}
impl IPropertyEnumTypeVtbl {
    pub const fn new<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyEnumTypeVtbl {
        unsafe extern "system" fn GetEnumType<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEnumType(::core::mem::transmute_copy(&penumtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&ppropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeMinValue<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRangeMinValue(::core::mem::transmute_copy(&ppropvarmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeSetValue<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRangeSetValue(::core::mem::transmute_copy(&ppropvarset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayText<Impl: IPropertyEnumTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayText(::core::mem::transmute_copy(&ppszdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyEnumType>, base.5, GetEnumType::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, GetRangeMinValue::<Impl, OFFSET>, GetRangeSetValue::<Impl, OFFSET>, GetDisplayText::<Impl, OFFSET>)
    }
}
pub trait IPropertyEnumType2Impl: Sized + IPropertyEnumTypeImpl {
    fn GetImageReference();
}
impl ::windows::core::RuntimeName for IPropertyEnumType2 {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyEnumType2";
}
impl IPropertyEnumType2Vtbl {
    pub const fn new<Impl: IPropertyEnumType2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyEnumType2Vtbl {
        unsafe extern "system" fn GetImageReference<Impl: IPropertyEnumType2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetImageReference(::core::mem::transmute_copy(&ppszimageres)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyEnumType2>, base.5, GetImageReference::<Impl, OFFSET>)
    }
}
pub trait IPropertyEnumTypeListImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetConditionAt();
    fn FindMatchingIndex();
}
impl ::windows::core::RuntimeName for IPropertyEnumTypeList {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyEnumTypeList";
}
impl IPropertyEnumTypeListVtbl {
    pub const fn new<Impl: IPropertyEnumTypeListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyEnumTypeListVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyEnumTypeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pctypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyEnumTypeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(itype, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConditionAt<Impl: IPropertyEnumTypeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConditionAt(nindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindMatchingIndex<Impl: IPropertyEnumTypeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindMatchingIndex(&*(&propvarcmp as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyEnumTypeList>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetConditionAt::<Impl, OFFSET>, FindMatchingIndex::<Impl, OFFSET>)
    }
}
pub trait IPropertyStoreImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
    fn Commit();
}
impl ::windows::core::RuntimeName for IPropertyStore {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyStore";
}
impl IPropertyStoreVtbl {
    pub const fn new<Impl: IPropertyStoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyStoreVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&cprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(iprop, ::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyStore>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
pub trait IPropertyStoreCacheImpl: Sized + IPropertyStoreImpl {
    fn GetState();
    fn GetValueAndState();
    fn SetState();
    fn SetValueAndState();
}
impl ::windows::core::RuntimeName for IPropertyStoreCache {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyStoreCache";
}
impl IPropertyStoreCacheVtbl {
    pub const fn new<Impl: IPropertyStoreCacheImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyStoreCacheVtbl {
        unsafe extern "system" fn GetState<Impl: IPropertyStoreCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetState(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAndState<Impl: IPropertyStoreCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueAndState(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IPropertyStoreCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetState(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueAndState<Impl: IPropertyStoreCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValueAndState(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&ppropvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyStoreCache>, base.5, GetState::<Impl, OFFSET>, GetValueAndState::<Impl, OFFSET>, SetState::<Impl, OFFSET>, SetValueAndState::<Impl, OFFSET>)
    }
}
pub trait IPropertyStoreCapabilitiesImpl: Sized {
    fn IsPropertyWritable();
}
impl ::windows::core::RuntimeName for IPropertyStoreCapabilities {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyStoreCapabilities";
}
impl IPropertyStoreCapabilitiesVtbl {
    pub const fn new<Impl: IPropertyStoreCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyStoreCapabilitiesVtbl {
        unsafe extern "system" fn IsPropertyWritable<Impl: IPropertyStoreCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPropertyWritable(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyStoreCapabilities>, base.5, IsPropertyWritable::<Impl, OFFSET>)
    }
}
pub trait IPropertyStoreFactoryImpl: Sized {
    fn GetPropertyStore();
    fn GetPropertyStoreForKeys();
}
impl ::windows::core::RuntimeName for IPropertyStoreFactory {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyStoreFactory";
}
impl IPropertyStoreFactoryVtbl {
    pub const fn new<Impl: IPropertyStoreFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyStoreFactoryVtbl {
        unsafe extern "system" fn GetPropertyStore<Impl: IPropertyStoreFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStore(flags, &*(&punkfactory as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Impl: IPropertyStoreFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyStoreForKeys(&*(&rgkeys as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ckeys, flags, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyStoreFactory>, base.5, GetPropertyStore::<Impl, OFFSET>, GetPropertyStoreForKeys::<Impl, OFFSET>)
    }
}
pub trait IPropertySystemImpl: Sized {
    fn GetPropertyDescription();
    fn GetPropertyDescriptionByName();
    fn GetPropertyDescriptionListFromString();
    fn EnumeratePropertyDescriptions();
    fn FormatForDisplay();
    fn FormatForDisplayAlloc();
    fn RegisterPropertySchema();
    fn UnregisterPropertySchema();
    fn RefreshPropertySchema();
}
impl ::windows::core::RuntimeName for IPropertySystem {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertySystem";
}
impl IPropertySystemVtbl {
    pub const fn new<Impl: IPropertySystemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertySystemVtbl {
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyDescription(&*(&propkey as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyDescriptionByName(&*(&pszcanonicalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyDescriptionListFromString(&*(&pszproplist as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePropertyDescriptions(filteron, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatForDisplay(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), pdff, ::core::mem::transmute_copy(&psztext), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatForDisplayAlloc(&*(&key as *const <PROPERTYKEY as ::windows::core::Abi>::Abi as *const <PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&propvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), pdff, ::core::mem::transmute_copy(&ppszdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPropertySchema<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterPropertySchema(&*(&pszpath as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterPropertySchema<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterPropertySchema(&*(&pszpath as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshPropertySchema<Impl: IPropertySystemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RefreshPropertySchema() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertySystem>, base.5, GetPropertyDescription::<Impl, OFFSET>, GetPropertyDescriptionByName::<Impl, OFFSET>, GetPropertyDescriptionListFromString::<Impl, OFFSET>, EnumeratePropertyDescriptions::<Impl, OFFSET>, FormatForDisplay::<Impl, OFFSET>, FormatForDisplayAlloc::<Impl, OFFSET>, RegisterPropertySchema::<Impl, OFFSET>, UnregisterPropertySchema::<Impl, OFFSET>, RefreshPropertySchema::<Impl, OFFSET>)
    }
}
pub trait IPropertySystemChangeNotifyImpl: Sized {
    fn SchemaRefreshed();
}
impl ::windows::core::RuntimeName for IPropertySystemChangeNotify {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertySystemChangeNotify";
}
impl IPropertySystemChangeNotifyVtbl {
    pub const fn new<Impl: IPropertySystemChangeNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertySystemChangeNotifyVtbl {
        unsafe extern "system" fn SchemaRefreshed<Impl: IPropertySystemChangeNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SchemaRefreshed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertySystemChangeNotify>, base.5, SchemaRefreshed::<Impl, OFFSET>)
    }
}
pub trait IPropertyUIImpl: Sized {
    fn ParsePropertyName();
    fn GetCannonicalName();
    fn GetDisplayName();
    fn GetPropertyDescription();
    fn GetDefaultWidth();
    fn GetFlags();
    fn FormatForDisplay();
    fn GetHelpInfo();
}
impl ::windows::core::RuntimeName for IPropertyUI {
    const NAME: &'static str = "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyUI";
}
impl IPropertyUIVtbl {
    pub const fn new<Impl: IPropertyUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyUIVtbl {
        unsafe extern "system" fn ParsePropertyName<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParsePropertyName(&*(&pszname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmtid), ::core::mem::transmute_copy(&ppid), pcheaten) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCannonicalName<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCannonicalName(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, ::core::mem::transmute_copy(&pwsztext), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, flags, ::core::mem::transmute_copy(&pwsztext), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyDescription(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, ::core::mem::transmute_copy(&pwsztext), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultWidth<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultWidth(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, ::core::mem::transmute_copy(&pcxchars)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlags(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, ::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatForDisplay(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, &*(&ppropvar as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), puiff, ::core::mem::transmute_copy(&pwsztext), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IPropertyUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelpInfo(&*(&fmtid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pid, ::core::mem::transmute_copy(&pwszhelpfile), cch, ::core::mem::transmute_copy(&puhelpid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyUI>, base.5, ParsePropertyName::<Impl, OFFSET>, GetCannonicalName::<Impl, OFFSET>, GetDisplayName::<Impl, OFFSET>, GetPropertyDescription::<Impl, OFFSET>, GetDefaultWidth::<Impl, OFFSET>, GetFlags::<Impl, OFFSET>, FormatForDisplay::<Impl, OFFSET>, GetHelpInfo::<Impl, OFFSET>)
    }
}
