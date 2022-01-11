pub trait ICreateObjectImpl: Sized {
    fn CreateObject();
}
impl ICreateObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateObjectVtbl {
        unsafe extern "system" fn CreateObject<Impl: ICreateObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateObject as ::windows::core::Interface>::IID
    }
}
pub trait IDelayedPropertyStoreFactoryImpl: Sized + IPropertyStoreFactoryImpl {
    fn GetDelayedPropertyStore();
}
impl IDelayedPropertyStoreFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDelayedPropertyStoreFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDelayedPropertyStoreFactoryVtbl {
        unsafe extern "system" fn GetDelayedPropertyStore<Impl: IDelayedPropertyStoreFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPropertyStore::<Impl, IMPL_OFFSET>, GetPropertyStoreForKeys::<Impl, IMPL_OFFSET>, GetDelayedPropertyStore::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDelayedPropertyStoreFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInitializeWithFileImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IInitializeWithFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeWithFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeWithFileVtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: super::super::super::Foundation::PWSTR, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInitializeWithStreamImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
impl IInitializeWithStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeWithStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeWithStreamVtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait INamedPropertyStoreImpl: Sized {
    fn GetNamedValue();
    fn SetNamedValue();
    fn GetNameCount();
    fn GetNameAt();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl INamedPropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedPropertyStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INamedPropertyStoreVtbl {
        unsafe extern "system" fn GetNamedValue<Impl: INamedPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamedValue<Impl: INamedPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameCount<Impl: INamedPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNameAt<Impl: INamedPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNamedValue::<Impl, IMPL_OFFSET>, SetNamedValue::<Impl, IMPL_OFFSET>, GetNameCount::<Impl, IMPL_OFFSET>, GetNameAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedPropertyStore as ::windows::core::Interface>::IID
    }
}
pub trait IObjectWithPropertyKeyImpl: Sized {
    fn SetPropertyKey();
    fn GetPropertyKey();
}
impl IObjectWithPropertyKeyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectWithPropertyKeyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectWithPropertyKeyVtbl {
        unsafe extern "system" fn SetPropertyKey<Impl: IObjectWithPropertyKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyKey<Impl: IObjectWithPropertyKeyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPropertyKey::<Impl, IMPL_OFFSET>, GetPropertyKey::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectWithPropertyKey as ::windows::core::Interface>::IID
    }
}
pub trait IPersistSerializedPropStorageImpl: Sized {
    fn SetFlags();
    fn SetPropertyStorage();
    fn GetPropertyStorage();
}
impl IPersistSerializedPropStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistSerializedPropStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistSerializedPropStorageVtbl {
        unsafe extern "system" fn SetFlags<Impl: IPersistSerializedPropStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyStorage<Impl: IPersistSerializedPropStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: *const SERIALIZEDPROPSTORAGE, cb: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyStorage<Impl: IPersistSerializedPropStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFlags::<Impl, IMPL_OFFSET>, SetPropertyStorage::<Impl, IMPL_OFFSET>, GetPropertyStorage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage as ::windows::core::Interface>::IID
    }
}
pub trait IPersistSerializedPropStorage2Impl: Sized + IPersistSerializedPropStorageImpl {
    fn GetPropertyStorageSize();
    fn GetPropertyStorageBuffer();
}
impl IPersistSerializedPropStorage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistSerializedPropStorage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistSerializedPropStorage2Vtbl {
        unsafe extern "system" fn GetPropertyStorageSize<Impl: IPersistSerializedPropStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Impl: IPersistSerializedPropStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFlags::<Impl, IMPL_OFFSET>, SetPropertyStorage::<Impl, IMPL_OFFSET>, GetPropertyStorage::<Impl, IMPL_OFFSET>, GetPropertyStorageSize::<Impl, IMPL_OFFSET>, GetPropertyStorageBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyChangeImpl: Sized + IObjectWithPropertyKeyImpl {
    fn ApplyToPropVariant();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyChangeVtbl {
        unsafe extern "system" fn ApplyToPropVariant<Impl: IPropertyChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPropertyKey::<Impl, IMPL_OFFSET>, GetPropertyKey::<Impl, IMPL_OFFSET>, ApplyToPropVariant::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyChange as ::windows::core::Interface>::IID
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
impl IPropertyChangeArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyChangeArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyChangeArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertAt<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppendOrReplace<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsKeyInArray<Impl: IPropertyChangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, InsertAt::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, AppendOrReplace::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>, IsKeyInArray::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyChangeArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionVtbl {
        unsafe extern "system" fn GetPropertyKey<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyType<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEditInvitation<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszinvite: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeFlags<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewFlags<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayType<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnState<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroupingRange<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRelativeDescription<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut super::super::super::Foundation::PWSTR, ppszdesc2: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSortDescription<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAggregationType<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConditionType<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnumTypeList<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsValueCanonical<Impl: IPropertyDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescription2Impl: Sized + IPropertyDescriptionImpl {
    fn GetImageReferenceForValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescription2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescription2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescription2Vtbl {
        unsafe extern "system" fn GetImageReferenceForValue<Impl: IPropertyDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical::<Impl, IMPL_OFFSET>,
            GetImageReferenceForValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionAliasInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSortByAlias();
    fn GetAdditionalSortByAliases();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionAliasInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionAliasInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionAliasInfoVtbl {
        unsafe extern "system" fn GetSortByAlias<Impl: IPropertyDescriptionAliasInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Impl: IPropertyDescriptionAliasInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical::<Impl, IMPL_OFFSET>,
            GetSortByAlias::<Impl, IMPL_OFFSET>,
            GetAdditionalSortByAliases::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionAliasInfo as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyDescriptionListImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl IPropertyDescriptionListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionListVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyDescriptionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyDescriptionListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionRelatedPropertyInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetRelatedProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionRelatedPropertyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionRelatedPropertyInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionRelatedPropertyInfoVtbl {
        unsafe extern "system" fn GetRelatedProperty<Impl: IPropertyDescriptionRelatedPropertyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrelationshipname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical::<Impl, IMPL_OFFSET>,
            GetRelatedProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionRelatedPropertyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
pub trait IPropertyDescriptionSearchInfoImpl: Sized + IPropertyDescriptionImpl {
    fn GetSearchInfoFlags();
    fn GetColumnIndexType();
    fn GetProjectionString();
    fn GetMaxSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
impl IPropertyDescriptionSearchInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyDescriptionSearchInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyDescriptionSearchInfoVtbl {
        unsafe extern "system" fn GetSearchInfoFlags<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnIndexType<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProjectionString<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszprojection: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxSize<Impl: IPropertyDescriptionSearchInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyKey::<Impl, IMPL_OFFSET>,
            GetCanonicalName::<Impl, IMPL_OFFSET>,
            GetPropertyType::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            GetEditInvitation::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetViewFlags::<Impl, IMPL_OFFSET>,
            GetDefaultColumnWidth::<Impl, IMPL_OFFSET>,
            GetDisplayType::<Impl, IMPL_OFFSET>,
            GetColumnState::<Impl, IMPL_OFFSET>,
            GetGroupingRange::<Impl, IMPL_OFFSET>,
            GetRelativeDescriptionType::<Impl, IMPL_OFFSET>,
            GetRelativeDescription::<Impl, IMPL_OFFSET>,
            GetSortDescription::<Impl, IMPL_OFFSET>,
            GetSortDescriptionLabel::<Impl, IMPL_OFFSET>,
            GetAggregationType::<Impl, IMPL_OFFSET>,
            GetConditionType::<Impl, IMPL_OFFSET>,
            GetEnumTypeList::<Impl, IMPL_OFFSET>,
            CoerceToCanonicalValue::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            IsValueCanonical::<Impl, IMPL_OFFSET>,
            GetSearchInfoFlags::<Impl, IMPL_OFFSET>,
            GetColumnIndexType::<Impl, IMPL_OFFSET>,
            GetProjectionString::<Impl, IMPL_OFFSET>,
            GetMaxSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyDescriptionSearchInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumTypeImpl: Sized {
    fn GetEnumType();
    fn GetValue();
    fn GetRangeMinValue();
    fn GetRangeSetValue();
    fn GetDisplayText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumTypeVtbl {
        unsafe extern "system" fn GetEnumType<Impl: IPropertyEnumTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyEnumTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRangeMinValue<Impl: IPropertyEnumTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRangeSetValue<Impl: IPropertyEnumTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayText<Impl: IPropertyEnumTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEnumType::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, GetRangeMinValue::<Impl, IMPL_OFFSET>, GetRangeSetValue::<Impl, IMPL_OFFSET>, GetDisplayText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumType2Impl: Sized + IPropertyEnumTypeImpl {
    fn GetImageReference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumType2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumType2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumType2Vtbl {
        unsafe extern "system" fn GetImageReference<Impl: IPropertyEnumType2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageres: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEnumType::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, GetRangeMinValue::<Impl, IMPL_OFFSET>, GetRangeSetValue::<Impl, IMPL_OFFSET>, GetDisplayText::<Impl, IMPL_OFFSET>, GetImageReference::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumType2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyEnumTypeListImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetConditionAt();
    fn FindMatchingIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyEnumTypeListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyEnumTypeListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyEnumTypeListVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyEnumTypeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyEnumTypeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConditionAt<Impl: IPropertyEnumTypeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindMatchingIndex<Impl: IPropertyEnumTypeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetConditionAt::<Impl, IMPL_OFFSET>, FindMatchingIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyEnumTypeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyStoreImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
    fn Commit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPropertyStoreCacheImpl: Sized + IPropertyStoreImpl {
    fn GetState();
    fn GetValueAndState();
    fn SetState();
    fn SetValueAndState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyStoreCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCacheVtbl {
        unsafe extern "system" fn GetState<Impl: IPropertyStoreCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueAndState<Impl: IPropertyStoreCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: IPropertyStoreCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueAndState<Impl: IPropertyStoreCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, GetState::<Impl, IMPL_OFFSET>, GetValueAndState::<Impl, IMPL_OFFSET>, SetState::<Impl, IMPL_OFFSET>, SetValueAndState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCache as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyStoreCapabilitiesImpl: Sized {
    fn IsPropertyWritable();
}
impl IPropertyStoreCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCapabilitiesVtbl {
        unsafe extern "system" fn IsPropertyWritable<Impl: IPropertyStoreCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPropertyWritable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyStoreFactoryImpl: Sized {
    fn GetPropertyStore();
    fn GetPropertyStoreForKeys();
}
impl IPropertyStoreFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreFactoryVtbl {
        unsafe extern "system" fn GetPropertyStore<Impl: IPropertyStoreFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Impl: IPropertyStoreFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPropertyStore::<Impl, IMPL_OFFSET>, GetPropertyStoreForKeys::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertySystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySystemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySystemVtbl {
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcanonicalname: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszproplist: super::super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterPropertySchema<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterPropertySchema<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshPropertySchema<Impl: IPropertySystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyDescription::<Impl, IMPL_OFFSET>,
            GetPropertyDescriptionByName::<Impl, IMPL_OFFSET>,
            GetPropertyDescriptionListFromString::<Impl, IMPL_OFFSET>,
            EnumeratePropertyDescriptions::<Impl, IMPL_OFFSET>,
            FormatForDisplay::<Impl, IMPL_OFFSET>,
            FormatForDisplayAlloc::<Impl, IMPL_OFFSET>,
            RegisterPropertySchema::<Impl, IMPL_OFFSET>,
            UnregisterPropertySchema::<Impl, IMPL_OFFSET>,
            RefreshPropertySchema::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySystem as ::windows::core::Interface>::IID
    }
}
pub trait IPropertySystemChangeNotifyImpl: Sized {
    fn SchemaRefreshed();
}
impl IPropertySystemChangeNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertySystemChangeNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertySystemChangeNotifyVtbl {
        unsafe extern "system" fn SchemaRefreshed<Impl: IPropertySystemChangeNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SchemaRefreshed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertySystemChangeNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPropertyUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyUIVtbl {
        unsafe extern "system" fn ParsePropertyName<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCannonicalName<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyDescription<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultWidth<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatForDisplay<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: super::super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpInfo<Impl: IPropertyUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pid: u32, pwszhelpfile: super::super::super::Foundation::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ParsePropertyName::<Impl, IMPL_OFFSET>, GetCannonicalName::<Impl, IMPL_OFFSET>, GetDisplayName::<Impl, IMPL_OFFSET>, GetPropertyDescription::<Impl, IMPL_OFFSET>, GetDefaultWidth::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>, FormatForDisplay::<Impl, IMPL_OFFSET>, GetHelpInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyUI as ::windows::core::Interface>::IID
    }
}
