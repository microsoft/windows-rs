pub trait ICreateObject_Impl: Sized {
    fn CreateObject(&mut self, clsid: *const ::windows::core::GUID, punkouter: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ICreateObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateObject_Vtbl {
        unsafe extern "system" fn CreateObject<Impl: ICreateObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateObject(::core::mem::transmute_copy(&clsid), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateObject: CreateObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateObject as ::windows::core::Interface>::IID
    }
}
pub trait IDelayedPropertyStoreFactory_Impl: Sized + IPropertyStoreFactory_Impl {
    fn GetDelayedPropertyStore(&mut self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDelayedPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelayedPropertyStoreFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelayedPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetDelayedPropertyStore<Impl: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDelayedPropertyStore(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&dwstoreid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: IPropertyStoreFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDelayedPropertyStore: GetDelayedPropertyStore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDelayedPropertyStoreFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInitializeWithFile_Impl: Sized {
    fn Initialize(&mut self, pszfilepath: super::super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInitializeWithFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeWithFile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeWithFile_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: super::super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pszfilepath), ::core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInitializeWithStream_Impl: Sized {
    fn Initialize(&mut self, pstream: ::core::option::Option<super::super::super::System::Com::IStream>, grfmode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IInitializeWithStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeWithStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeWithStream_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait INamedPropertyStore_Impl: Sized {
    fn GetNamedValue(&mut self, pszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetNamedValue(&mut self, pszname: super::super::super::Foundation::PWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetNameCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetNameAt(&mut self, iprop: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl INamedPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedPropertyStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INamedPropertyStore_Vtbl {
        unsafe extern "system" fn GetNamedValue<Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamedValue(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedValue<Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamedValue(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn GetNameCount<Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameAt<Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameAt(::core::mem::transmute_copy(&iprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNamedValue: GetNamedValue::<Impl, IMPL_OFFSET>,
            SetNamedValue: SetNamedValue::<Impl, IMPL_OFFSET>,
            GetNameCount: GetNameCount::<Impl, IMPL_OFFSET>,
            GetNameAt: GetNameAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedPropertyStore as ::windows::core::Interface>::IID
    }
}
pub trait IObjectWithPropertyKey_Impl: Sized {
    fn SetPropertyKey(&mut self, key: *const PROPERTYKEY) -> ::windows::core::Result<()>;
    fn GetPropertyKey(&mut self) -> ::windows::core::Result<PROPERTYKEY>;
}
impl IObjectWithPropertyKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithPropertyKey_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectWithPropertyKey_Vtbl {
        unsafe extern "system" fn SetPropertyKey<Impl: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyKey(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetPropertyKey<Impl: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPropertyKey: SetPropertyKey::<Impl, IMPL_OFFSET>,
            GetPropertyKey: GetPropertyKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectWithPropertyKey as ::windows::core::Interface>::IID
    }
}
pub trait IPersistSerializedPropStorage_Impl: Sized {
    fn SetFlags(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn SetPropertyStorage(&mut self, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::Result<()>;
    fn GetPropertyStorage(&mut self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::Result<()>;
}
impl IPersistSerializedPropStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistSerializedPropStorage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistSerializedPropStorage_Vtbl {
        unsafe extern "system" fn SetFlags<Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetPropertyStorage<Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyStorage(::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn GetPropertyStorage<Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyStorage(::core::mem::transmute_copy(&ppsps), ::core::mem::transmute_copy(&pcb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            SetPropertyStorage: SetPropertyStorage::<Impl, IMPL_OFFSET>,
            GetPropertyStorage: GetPropertyStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage as ::windows::core::Interface>::IID
    }
}
pub trait IPersistSerializedPropStorage2_Impl: Sized + IPersistSerializedPropStorage_Impl {
    fn GetPropertyStorageSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetPropertyStorageBuffer(&mut self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::Result<()>;
}
impl IPersistSerializedPropStorage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistSerializedPropStorage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistSerializedPropStorage2_Vtbl {
        unsafe extern "system" fn GetPropertyStorageSize<Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyStorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyStorageBuffer(::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        Self {
            base: IPersistSerializedPropStorage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPropertyStorageSize: GetPropertyStorageSize::<Impl, IMPL_OFFSET>,
            GetPropertyStorageBuffer: GetPropertyStorageBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyChange_Impl: Sized + IObjectWithPropertyKey_Impl {
    fn ApplyToPropVariant(&mut self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyChange_Vtbl {
        unsafe extern "system" fn ApplyToPropVariant<Impl: IPropertyChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyToPropVariant(::core::mem::transmute_copy(&propvarin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvarout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IObjectWithPropertyKey_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ApplyToPropVariant: ApplyToPropVariant::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyChange as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyChangeArray_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn InsertAt(&mut self, iindex: u32, ppropchange: ::core::option::Option<IPropertyChange>) -> ::windows::core::Result<()>;
    fn Append(&mut self, ppropchange: ::core::option::Option<IPropertyChange>) -> ::windows::core::Result<()>;
    fn AppendOrReplace(&mut self, ppropchange: ::core::option::Option<IPropertyChange>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, iindex: u32) -> ::windows::core::Result<()>;
    fn IsKeyInArray(&mut self, key: *const PROPERTYKEY) -> ::windows::core::Result<()>;
}
impl IPropertyChangeArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyChangeArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyChangeArray_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoperations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn InsertAt<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute(&ppropchange)).into()
        }
        unsafe extern "system" fn Append<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&ppropchange)).into()
        }
        unsafe extern "system" fn AppendOrReplace<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendOrReplace(::core::mem::transmute(&ppropchange)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&iindex)).into()
        }
        unsafe extern "system" fn IsKeyInArray<Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsKeyInArray(::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            InsertAt: InsertAt::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            AppendOrReplace: AppendOrReplace::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            IsKeyInArray: IsKeyInArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyChangeArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescription_Impl: Sized {
    fn GetPropertyKey(&mut self) -> ::windows::core::Result<PROPERTYKEY>;
    fn GetCanonicalName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetPropertyType(&mut self) -> ::windows::core::Result<u16>;
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetEditInvitation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetTypeFlags(&mut self, mask: PROPDESC_TYPE_FLAGS) -> ::windows::core::Result<PROPDESC_TYPE_FLAGS>;
    fn GetViewFlags(&mut self) -> ::windows::core::Result<PROPDESC_VIEW_FLAGS>;
    fn GetDefaultColumnWidth(&mut self) -> ::windows::core::Result<u32>;
    fn GetDisplayType(&mut self) -> ::windows::core::Result<PROPDESC_DISPLAYTYPE>;
    fn GetColumnState(&mut self) -> ::windows::core::Result<u32>;
    fn GetGroupingRange(&mut self) -> ::windows::core::Result<PROPDESC_GROUPING_RANGE>;
    fn GetRelativeDescriptionType(&mut self) -> ::windows::core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE>;
    fn GetRelativeDescription(&mut self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetSortDescription(&mut self) -> ::windows::core::Result<PROPDESC_SORTDESCRIPTION>;
    fn GetSortDescriptionLabel(&mut self, fdescending: super::super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetAggregationType(&mut self) -> ::windows::core::Result<PROPDESC_AGGREGATION_TYPE>;
    fn GetConditionType(&mut self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::Result<()>;
    fn GetEnumTypeList(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CoerceToCanonicalValue(&mut self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn FormatForDisplay(&mut self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn IsValueCanonical(&mut self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescription_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescription_Vtbl {
        unsafe extern "system" fn GetPropertyKey<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCanonicalName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyType<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvartype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEditInvitation<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEditInvitation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszinvite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeFlags(::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdtflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewFlags<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdvflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultColumnWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcxchars = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayType<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdisplaytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnState<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnState() {
                ::core::result::Result::Ok(ok__) => {
                    *pcsflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupingRange<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroupingRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pgr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelativeDescriptionType() {
                ::core::result::Result::Ok(ok__) => {
                    *prdt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescription<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRelativeDescription(::core::mem::transmute_copy(&propvar1), ::core::mem::transmute_copy(&propvar2), ::core::mem::transmute_copy(&ppszdesc1), ::core::mem::transmute_copy(&ppszdesc2)).into()
        }
        unsafe extern "system" fn GetSortDescription<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *psd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSortDescriptionLabel(::core::mem::transmute_copy(&fdescending)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAggregationType<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAggregationType() {
                ::core::result::Result::Ok(ok__) => {
                    *paggtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConditionType<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConditionType(::core::mem::transmute_copy(&pcontype), ::core::mem::transmute_copy(&popdefault)).into()
        }
        unsafe extern "system" fn GetEnumTypeList<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEnumTypeList(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CoerceToCanonicalValue(::core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatForDisplay(::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValueCanonical<Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValueCanonical(::core::mem::transmute_copy(&propvar)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPropertyKey: GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName: GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType: GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation: GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags: GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags: GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth: GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType: GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState: GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange: GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType: GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription: GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription: GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel: GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType: GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType: GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList: GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue: CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay: FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical: IsValueCanonical::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescription2_Impl: Sized + IPropertyDescription_Impl {
    fn GetImageReferenceForValue(&mut self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescription2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescription2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescription2_Vtbl {
        unsafe extern "system" fn GetImageReferenceForValue<Impl: IPropertyDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageReferenceForValue(::core::mem::transmute_copy(&propvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszimageres = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPropertyDescription_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetImageReferenceForValue: GetImageReferenceForValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionAliasInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSortByAlias(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdditionalSortByAliases(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionAliasInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionAliasInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionAliasInfo_Vtbl {
        unsafe extern "system" fn GetSortByAlias<Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSortByAlias(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdditionalSortByAliases(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: IPropertyDescription_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSortByAlias: GetSortByAlias::<Impl, IMPL_OFFSET>,
            GetAdditionalSortByAliases: GetAdditionalSortByAliases::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionAliasInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyDescriptionList_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IPropertyDescriptionList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionList_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcelem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&ielem), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionRelatedPropertyInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetRelatedProperty(&mut self, pszrelationshipname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionRelatedPropertyInfo_Vtbl {
        unsafe extern "system" fn GetRelatedProperty<Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrelationshipname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRelatedProperty(::core::mem::transmute_copy(&pszrelationshipname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: IPropertyDescription_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetRelatedProperty: GetRelatedProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionRelatedPropertyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionSearchInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSearchInfoFlags(&mut self) -> ::windows::core::Result<PROPDESC_SEARCHINFO_FLAGS>;
    fn GetColumnIndexType(&mut self) -> ::windows::core::Result<PROPDESC_COLUMNINDEX_TYPE>;
    fn GetProjectionString(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn GetMaxSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionSearchInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionSearchInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionSearchInfo_Vtbl {
        unsafe extern "system" fn GetSearchInfoFlags<Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchInfoFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdsiflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnIndexType<Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnIndexType() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdcitype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProjectionString<Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszprojection: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProjectionString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszprojection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxSize<Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmaxsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IPropertyDescription_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSearchInfoFlags: GetSearchInfoFlags::<Impl, IMPL_OFFSET>,
            GetColumnIndexType: GetColumnIndexType::<Impl, IMPL_OFFSET>,
            GetProjectionString: GetProjectionString::<Impl, IMPL_OFFSET>,
            GetMaxSize: GetMaxSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionSearchInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumType_Impl: Sized {
    fn GetEnumType(&mut self) -> ::windows::core::Result<PROPENUMTYPE>;
    fn GetValue(&mut self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeMinValue(&mut self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeSetValue(&mut self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetDisplayText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumType_Vtbl {
        unsafe extern "system" fn GetEnumType<Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumType() {
                ::core::result::Result::Ok(ok__) => {
                    *penumtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeMinValue<Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeMinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvarmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeSetValue<Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeSetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvarset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayText<Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEnumType: GetEnumType::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetRangeMinValue: GetRangeMinValue::<Impl, IMPL_OFFSET>,
            GetRangeSetValue: GetRangeSetValue::<Impl, IMPL_OFFSET>,
            GetDisplayText: GetDisplayText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumType2_Impl: Sized + IPropertyEnumType_Impl {
    fn GetImageReference(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumType2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumType2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumType2_Vtbl {
        unsafe extern "system" fn GetImageReference<Impl: IPropertyEnumType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageReference() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszimageres = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IPropertyEnumType_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetImageReference: GetImageReference::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumType2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumTypeList_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetConditionAt(&mut self, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FindMatchingIndex(&mut self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumTypeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumTypeList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumTypeList_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pctypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&itype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetConditionAt<Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConditionAt(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FindMatchingIndex<Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindMatchingIndex(::core::mem::transmute_copy(&propvarcmp)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetConditionAt: GetConditionAt::<Impl, IMPL_OFFSET>,
            FindMatchingIndex: FindMatchingIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumTypeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyStore_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, iprop: u32) -> ::windows::core::Result<PROPERTYKEY>;
    fn GetValue(&mut self, key: *const PROPERTYKEY) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&mut self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStore_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *cprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&iprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyStoreCache_Impl: Sized + IPropertyStore_Impl {
    fn GetState(&mut self, key: *const PROPERTYKEY) -> ::windows::core::Result<PSC_STATE>;
    fn GetValueAndState(&mut self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::Result<()>;
    fn SetState(&mut self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::Result<()>;
    fn SetValueAndState(&mut self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyStoreCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCache_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCache_Vtbl {
        unsafe extern "system" fn GetState<Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAndState<Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValueAndState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetState<Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetValueAndState<Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueAndState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: IPropertyStore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetValueAndState: GetValueAndState::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            SetValueAndState: SetValueAndState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCache as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyStoreCapabilities_Impl: Sized {
    fn IsPropertyWritable(&mut self, key: *const PROPERTYKEY) -> ::windows::core::Result<()>;
}
impl IPropertyStoreCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertyWritable<Impl: IPropertyStoreCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPropertyWritable(::core::mem::transmute_copy(&key)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsPropertyWritable: IsPropertyWritable::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyStoreFactory_Impl: Sized {
    fn GetPropertyStore(&mut self, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPropertyStoreForKeys(&mut self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetPropertyStore<Impl: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyStore(::core::mem::transmute_copy(&flags), ::core::mem::transmute(&punkfactory), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Impl: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyStoreForKeys(::core::mem::transmute_copy(&rgkeys), ::core::mem::transmute_copy(&ckeys), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPropertyStore: GetPropertyStore::<Impl, IMPL_OFFSET>,
            GetPropertyStoreForKeys: GetPropertyStoreForKeys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertySystem_Impl: Sized {
    fn GetPropertyDescription(&mut self, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPropertyDescriptionByName(&mut self, pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPropertyDescriptionListFromString(&mut self, pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumeratePropertyDescriptions(&mut self, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FormatForDisplay(&mut self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn FormatForDisplayAlloc(&mut self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn RegisterPropertySchema(&mut self, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UnregisterPropertySchema(&mut self, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RefreshPropertySchema(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertySystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySystem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySystem_Vtbl {
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyDescription(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyDescriptionByName(::core::mem::transmute_copy(&pszcanonicalname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyDescriptionListFromString(::core::mem::transmute_copy(&pszproplist), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumeratePropertyDescriptions(::core::mem::transmute_copy(&filteron), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FormatForDisplay(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff), ::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatForDisplayAlloc(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPropertySchema<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterPropertySchema(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn UnregisterPropertySchema<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterPropertySchema(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn RefreshPropertySchema<Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshPropertySchema().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPropertyDescription: GetPropertyDescription::<Impl, IMPL_OFFSET>,
            GetPropertyDescriptionByName: GetPropertyDescriptionByName::<Impl, IMPL_OFFSET>,
            GetPropertyDescriptionListFromString: GetPropertyDescriptionListFromString::<Impl, IMPL_OFFSET>,
            EnumeratePropertyDescriptions: EnumeratePropertyDescriptions::<Impl, IMPL_OFFSET>,
            FormatForDisplay: FormatForDisplay::<Impl, IMPL_OFFSET>,
            FormatForDisplayAlloc: FormatForDisplayAlloc::<Impl, IMPL_OFFSET>,
            RegisterPropertySchema: RegisterPropertySchema::<Impl, IMPL_OFFSET>,
            UnregisterPropertySchema: UnregisterPropertySchema::<Impl, IMPL_OFFSET>,
            RefreshPropertySchema: RefreshPropertySchema::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySystem as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySystemChangeNotify_Impl: Sized {
    fn SchemaRefreshed(&mut self) -> ::windows::core::Result<()>;
}
impl IPropertySystemChangeNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySystemChangeNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySystemChangeNotify_Vtbl {
        unsafe extern "system" fn SchemaRefreshed<Impl: IPropertySystemChangeNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SchemaRefreshed().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SchemaRefreshed: SchemaRefreshed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySystemChangeNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyUI_Impl: Sized {
    fn ParsePropertyName(&mut self, pszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::Result<()>;
    fn GetCannonicalName(&mut self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetDisplayName(&mut self, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetPropertyDescription(&mut self, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetDefaultWidth(&mut self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<u32>;
    fn GetFlags(&mut self, fmtid: *const ::windows::core::GUID, pid: u32) -> ::windows::core::Result<PROPERTYUI_FLAGS>;
    fn FormatForDisplay(&mut self, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetHelpInfo(&mut self, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyUI_Vtbl {
        unsafe extern "system" fn ParsePropertyName<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParsePropertyName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pfmtid), ::core::mem::transmute_copy(&ppid), ::core::mem::transmute_copy(&pcheaten)).into()
        }
        unsafe extern "system" fn GetCannonicalName<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCannonicalName(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayName(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyDescription(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDefaultWidth<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultWidth(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcxchars = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FormatForDisplay(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&puiff), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHelpInfo(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwszhelpfile), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&puhelpid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ParsePropertyName: ParsePropertyName::<Impl, IMPL_OFFSET>,
            GetCannonicalName: GetCannonicalName::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            GetPropertyDescription: GetPropertyDescription::<Impl, IMPL_OFFSET>,
            GetDefaultWidth: GetDefaultWidth::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            FormatForDisplay: FormatForDisplay::<Impl, IMPL_OFFSET>,
            GetHelpInfo: GetHelpInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyUI as ::windows::core::Interface>::IID
    }
}
