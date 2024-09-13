pub trait ICreateObject_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateObject(&self, clsid: *const windows_core::GUID, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICreateObject {}
impl ICreateObject_Vtbl {
    pub const fn new<Identity: ICreateObject_Impl, const OFFSET: isize>() -> ICreateObject_Vtbl {
        unsafe extern "system" fn CreateObject<Identity: ICreateObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateObject_Impl::CreateObject(this, core::mem::transmute_copy(&clsid), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateObject as windows_core::Interface>::IID
    }
}
pub trait IDelayedPropertyStoreFactory_Impl: Sized + IPropertyStoreFactory_Impl {
    fn GetDelayedPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDelayedPropertyStoreFactory {}
impl IDelayedPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>() -> IDelayedPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetDelayedPropertyStore<Identity: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDelayedPropertyStoreFactory_Impl::GetDelayedPropertyStore(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&dwstoreid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IPropertyStoreFactory_Vtbl::new::<Identity, OFFSET>(), GetDelayedPropertyStore: GetDelayedPropertyStore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDelayedPropertyStoreFactory as windows_core::Interface>::IID || iid == &<IPropertyStoreFactory as windows_core::Interface>::IID
    }
}
pub trait IInitializeWithFile_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pszfilepath: &windows_core::PCWSTR, grfmode: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInitializeWithFile {}
impl IInitializeWithFile_Vtbl {
    pub const fn new<Identity: IInitializeWithFile_Impl, const OFFSET: isize>() -> IInitializeWithFile_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IInitializeWithFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCWSTR, grfmode: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeWithFile_Impl::Initialize(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInitializeWithStream_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pstream: Option<&super::super::super::System::Com::IStream>, grfmode: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInitializeWithStream {}
#[cfg(feature = "Win32_System_Com")]
impl IInitializeWithStream_Vtbl {
    pub const fn new<Identity: IInitializeWithStream_Impl, const OFFSET: isize>() -> IInitializeWithStream_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IInitializeWithStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, grfmode: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeWithStream_Impl::Initialize(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait INamedPropertyStore_Impl: Sized + windows_core::IUnknownImpl {
    fn GetNamedValue(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetNamedValue(&self, pszname: &windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetNameCount(&self) -> windows_core::Result<u32>;
    fn GetNameAt(&self, iprop: u32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INamedPropertyStore {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl INamedPropertyStore_Vtbl {
    pub const fn new<Identity: INamedPropertyStore_Impl, const OFFSET: isize>() -> INamedPropertyStore_Vtbl {
        unsafe extern "system" fn GetNamedValue<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppropvar: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INamedPropertyStore_Impl::GetNamedValue(this, core::mem::transmute(&pszname)) {
                Ok(ok__) => {
                    ppropvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedValue<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INamedPropertyStore_Impl::SetNamedValue(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn GetNameCount<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INamedPropertyStore_Impl::GetNameCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameAt<Identity: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprop: u32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INamedPropertyStore_Impl::GetNameAt(this, core::mem::transmute_copy(&iprop)) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IObjectWithPropertyKey_Impl: Sized + windows_core::IUnknownImpl {
    fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> windows_core::Result<()>;
    fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectWithPropertyKey {}
impl IObjectWithPropertyKey_Vtbl {
    pub const fn new<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>() -> IObjectWithPropertyKey_Vtbl {
        unsafe extern "system" fn SetPropertyKey<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectWithPropertyKey_Impl::SetPropertyKey(this, core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetPropertyKey<Identity: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectWithPropertyKey_Impl::GetPropertyKey(this, core::mem::transmute_copy(&pkey)).into()
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
pub trait IPersistSerializedPropStorage_Impl: Sized + windows_core::IUnknownImpl {
    fn SetFlags(&self, flags: i32) -> windows_core::Result<()>;
    fn SetPropertyStorage(&self, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> windows_core::Result<()>;
    fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPersistSerializedPropStorage {}
impl IPersistSerializedPropStorage_Vtbl {
    pub const fn new<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>() -> IPersistSerializedPropStorage_Vtbl {
        unsafe extern "system" fn SetFlags<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistSerializedPropStorage_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetPropertyStorage<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistSerializedPropStorage_Impl::SetPropertyStorage(this, core::mem::transmute_copy(&psps), core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn GetPropertyStorage<Identity: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistSerializedPropStorage_Impl::GetPropertyStorage(this, core::mem::transmute_copy(&ppsps), core::mem::transmute_copy(&pcb)).into()
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
pub trait IPersistSerializedPropStorage2_Impl: Sized + IPersistSerializedPropStorage_Impl {
    fn GetPropertyStorageSize(&self) -> windows_core::Result<u32>;
    fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPersistSerializedPropStorage2 {}
impl IPersistSerializedPropStorage2_Vtbl {
    pub const fn new<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>() -> IPersistSerializedPropStorage2_Vtbl {
        unsafe extern "system" fn GetPropertyStorageSize<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPersistSerializedPropStorage2_Impl::GetPropertyStorageSize(this) {
                Ok(ok__) => {
                    pcb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Identity: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPersistSerializedPropStorage2_Impl::GetPropertyStorageBuffer(this, core::mem::transmute_copy(&psps), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyChange_Impl: Sized + IObjectWithPropertyKey_Impl {
    fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyChange {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyChange_Vtbl {
    pub const fn new<Identity: IPropertyChange_Impl, const OFFSET: isize>() -> IPropertyChange_Vtbl {
        unsafe extern "system" fn ApplyToPropVariant<Identity: IPropertyChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvarin: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppropvarout: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyChange_Impl::ApplyToPropVariant(this, core::mem::transmute_copy(&propvarin)) {
                Ok(ok__) => {
                    ppropvarout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IObjectWithPropertyKey_Vtbl::new::<Identity, OFFSET>(), ApplyToPropVariant: ApplyToPropVariant::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyChange as windows_core::Interface>::IID || iid == &<IObjectWithPropertyKey as windows_core::Interface>::IID
    }
}
pub trait IPropertyChangeArray_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, iindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertAt(&self, iindex: u32, ppropchange: Option<&IPropertyChange>) -> windows_core::Result<()>;
    fn Append(&self, ppropchange: Option<&IPropertyChange>) -> windows_core::Result<()>;
    fn AppendOrReplace(&self, ppropchange: Option<&IPropertyChange>) -> windows_core::Result<()>;
    fn RemoveAt(&self, iindex: u32) -> windows_core::Result<()>;
    fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyChangeArray {}
impl IPropertyChangeArray_Vtbl {
    pub const fn new<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>() -> IPropertyChangeArray_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoperations: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyChangeArray_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcoperations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::GetAt(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::InsertAt(this, core::mem::transmute_copy(&iindex), windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn Append<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::Append(this, windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn AppendOrReplace<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::AppendOrReplace(this, windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::RemoveAt(this, core::mem::transmute_copy(&iindex)).into()
        }
        unsafe extern "system" fn IsKeyInArray<Identity: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyChangeArray_Impl::IsKeyInArray(this, core::mem::transmute_copy(&key)).into()
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> windows_core::Result<()>;
    fn GetCanonicalName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPropertyType(&self) -> windows_core::Result<u16>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEditInvitation(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> windows_core::Result<PROPDESC_TYPE_FLAGS>;
    fn GetViewFlags(&self) -> windows_core::Result<PROPDESC_VIEW_FLAGS>;
    fn GetDefaultColumnWidth(&self) -> windows_core::Result<u32>;
    fn GetDisplayType(&self) -> windows_core::Result<PROPDESC_DISPLAYTYPE>;
    fn GetColumnState(&self) -> windows_core::Result<u32>;
    fn GetGroupingRange(&self) -> windows_core::Result<PROPDESC_GROUPING_RANGE>;
    fn GetRelativeDescriptionType(&self) -> windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE>;
    fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetSortDescription(&self) -> windows_core::Result<PROPDESC_SORTDESCRIPTION>;
    fn GetSortDescriptionLabel(&self, fdescending: super::super::super::Foundation::BOOL) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAggregationType(&self) -> windows_core::Result<PROPDESC_AGGREGATION_TYPE>;
    fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> windows_core::Result<()>;
    fn GetEnumTypeList(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>;
    fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyDescription {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescription_Vtbl {
    pub const fn new<Identity: IPropertyDescription_Impl, const OFFSET: isize>() -> IPropertyDescription_Vtbl {
        unsafe extern "system" fn GetPropertyKey<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::GetPropertyKey(this, core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetCanonicalName(this) {
                Ok(ok__) => {
                    ppszname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvartype: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetPropertyType(this) {
                Ok(ok__) => {
                    pvartype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    ppszname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEditInvitation<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszinvite: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetEditInvitation(this) {
                Ok(ok__) => {
                    ppszinvite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetTypeFlags(this, core::mem::transmute_copy(&mask)) {
                Ok(ok__) => {
                    ppdtflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewFlags<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetViewFlags(this) {
                Ok(ok__) => {
                    ppdvflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcxchars: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetDefaultColumnWidth(this) {
                Ok(ok__) => {
                    pcxchars.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetDisplayType(this) {
                Ok(ok__) => {
                    pdisplaytype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnState<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetColumnState(this) {
                Ok(ok__) => {
                    pcsflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupingRange<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetGroupingRange(this) {
                Ok(ok__) => {
                    pgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetRelativeDescriptionType(this) {
                Ok(ok__) => {
                    prdt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescription<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar1: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, propvar2: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszdesc1: *mut windows_core::PWSTR, ppszdesc2: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::GetRelativeDescription(this, core::mem::transmute_copy(&propvar1), core::mem::transmute_copy(&propvar2), core::mem::transmute_copy(&ppszdesc1), core::mem::transmute_copy(&ppszdesc2)).into()
        }
        unsafe extern "system" fn GetSortDescription<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetSortDescription(this) {
                Ok(ok__) => {
                    psd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetSortDescriptionLabel(this, core::mem::transmute_copy(&fdescending)) {
                Ok(ok__) => {
                    ppszdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAggregationType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::GetAggregationType(this) {
                Ok(ok__) => {
                    paggtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConditionType<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::GetConditionType(this, core::mem::transmute_copy(&pcontype), core::mem::transmute_copy(&popdefault)).into()
        }
        unsafe extern "system" fn GetEnumTypeList<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::GetEnumTypeList(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvar: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::CoerceToCanonicalValue(this, core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription_Impl::FormatForDisplay(this, core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdfflags)) {
                Ok(ok__) => {
                    ppszdisplay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValueCanonical<Identity: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescription_Impl::IsValueCanonical(this, core::mem::transmute_copy(&propvar)).into()
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription2_Impl: Sized + IPropertyDescription_Impl {
    fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyDescription2 {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescription2_Vtbl {
    pub const fn new<Identity: IPropertyDescription2_Impl, const OFFSET: isize>() -> IPropertyDescription2_Vtbl {
        unsafe extern "system" fn GetImageReferenceForValue<Identity: IPropertyDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, ppszimageres: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescription2_Impl::GetImageReferenceForValue(this, core::mem::transmute_copy(&propvar)) {
                Ok(ok__) => {
                    ppszimageres.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(), GetImageReferenceForValue: GetImageReferenceForValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescription2 as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionAliasInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSortByAlias(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdditionalSortByAliases(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyDescriptionAliasInfo {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionAliasInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionAliasInfo_Vtbl {
        unsafe extern "system" fn GetSortByAlias<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescriptionAliasInfo_Impl::GetSortByAlias(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Identity: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescriptionAliasInfo_Impl::GetAdditionalSortByAliases(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
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
pub trait IPropertyDescriptionList_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, ielem: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyDescriptionList {}
impl IPropertyDescriptionList_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>() -> IPropertyDescriptionList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelem: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescriptionList_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ielem: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescriptionList_Impl::GetAt(this, core::mem::transmute_copy(&ielem), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionRelatedPropertyInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetRelatedProperty(&self, pszrelationshipname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyDescriptionRelatedPropertyInfo {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionRelatedPropertyInfo_Vtbl {
        unsafe extern "system" fn GetRelatedProperty<Identity: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrelationshipname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyDescriptionRelatedPropertyInfo_Impl::GetRelatedProperty(this, core::mem::transmute(&pszrelationshipname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IPropertyDescription_Vtbl::new::<Identity, OFFSET>(), GetRelatedProperty: GetRelatedProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionRelatedPropertyInfo as windows_core::Interface>::IID || iid == &<IPropertyDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionSearchInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSearchInfoFlags(&self) -> windows_core::Result<PROPDESC_SEARCHINFO_FLAGS>;
    fn GetColumnIndexType(&self) -> windows_core::Result<PROPDESC_COLUMNINDEX_TYPE>;
    fn GetProjectionString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMaxSize(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyDescriptionSearchInfo {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionSearchInfo_Vtbl {
    pub const fn new<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionSearchInfo_Vtbl {
        unsafe extern "system" fn GetSearchInfoFlags<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescriptionSearchInfo_Impl::GetSearchInfoFlags(this) {
                Ok(ok__) => {
                    ppdsiflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnIndexType<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescriptionSearchInfo_Impl::GetColumnIndexType(this) {
                Ok(ok__) => {
                    ppdcitype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProjectionString<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszprojection: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescriptionSearchInfo_Impl::GetProjectionString(this) {
                Ok(ok__) => {
                    ppszprojection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxSize<Identity: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyDescriptionSearchInfo_Impl::GetMaxSize(this) {
                Ok(ok__) => {
                    pcbmaxsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType_Impl: Sized + windows_core::IUnknownImpl {
    fn GetEnumType(&self) -> windows_core::Result<PROPENUMTYPE>;
    fn GetValue(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeMinValue(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeSetValue(&self) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetDisplayText(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyEnumType {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumType_Vtbl {
    pub const fn new<Identity: IPropertyEnumType_Impl, const OFFSET: isize>() -> IPropertyEnumType_Vtbl {
        unsafe extern "system" fn GetEnumType<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType_Impl::GetEnumType(this) {
                Ok(ok__) => {
                    penumtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvar: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType_Impl::GetValue(this) {
                Ok(ok__) => {
                    ppropvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeMinValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvarmin: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType_Impl::GetRangeMinValue(this) {
                Ok(ok__) => {
                    ppropvarmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeSetValue<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvarset: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType_Impl::GetRangeSetValue(this) {
                Ok(ok__) => {
                    ppropvarset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayText<Identity: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType_Impl::GetDisplayText(this) {
                Ok(ok__) => {
                    ppszdisplay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType2_Impl: Sized + IPropertyEnumType_Impl {
    fn GetImageReference(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyEnumType2 {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumType2_Vtbl {
    pub const fn new<Identity: IPropertyEnumType2_Impl, const OFFSET: isize>() -> IPropertyEnumType2_Vtbl {
        unsafe extern "system" fn GetImageReference<Identity: IPropertyEnumType2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszimageres: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumType2_Impl::GetImageReference(this) {
                Ok(ok__) => {
                    ppszimageres.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IPropertyEnumType_Vtbl::new::<Identity, OFFSET>(), GetImageReference: GetImageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyEnumType2 as windows_core::Interface>::IID || iid == &<IPropertyEnumType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumTypeList_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, itype: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetConditionAt(&self, nindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyEnumTypeList {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumTypeList_Vtbl {
    pub const fn new<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>() -> IPropertyEnumTypeList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumTypeList_Impl::GetCount(this) {
                Ok(ok__) => {
                    pctypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itype: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyEnumTypeList_Impl::GetAt(this, core::mem::transmute_copy(&itype), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetConditionAt<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyEnumTypeList_Impl::GetConditionAt(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FindMatchingIndex<Identity: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvarcmp: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pnindex: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyEnumTypeList_Impl::FindMatchingIndex(this, core::mem::transmute_copy(&propvarcmp)) {
                Ok(ok__) => {
                    pnindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStore_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> windows_core::Result<()>;
    fn GetValue(&self, key: *const PROPERTYKEY) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyStore {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyStore_Vtbl {
    pub const fn new<Identity: IPropertyStore_Impl, const OFFSET: isize>() -> IPropertyStore_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cprops: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStore_Impl::GetCount(this) {
                Ok(ok__) => {
                    cprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStore_Impl::GetAt(this, core::mem::transmute_copy(&iprop), core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStore_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStore_Impl::SetValue(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn Commit<Identity: IPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStore_Impl::Commit(this).into()
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStoreCache_Impl: Sized + IPropertyStore_Impl {
    fn GetState(&self, key: *const PROPERTYKEY) -> windows_core::Result<PSC_STATE>;
    fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> windows_core::Result<()>;
    fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> windows_core::Result<()>;
    fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyStoreCache {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyStoreCache_Vtbl {
    pub const fn new<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>() -> IPropertyStoreCache_Vtbl {
        unsafe extern "system" fn GetState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyStoreCache_Impl::GetState(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAndState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pstate: *mut PSC_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCache_Impl::GetValueAndState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCache_Impl::SetState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetValueAndState<Identity: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, state: PSC_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCache_Impl::SetValueAndState(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&state)).into()
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
pub trait IPropertyStoreCapabilities_Impl: Sized + windows_core::IUnknownImpl {
    fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyStoreCapabilities {}
impl IPropertyStoreCapabilities_Vtbl {
    pub const fn new<Identity: IPropertyStoreCapabilities_Impl, const OFFSET: isize>() -> IPropertyStoreCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertyWritable<Identity: IPropertyStoreCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreCapabilities_Impl::IsPropertyWritable(this, core::mem::transmute_copy(&key)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsPropertyWritable: IsPropertyWritable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCapabilities as windows_core::Interface>::IID
    }
}
pub trait IPropertyStoreFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyStoreForKeys(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyStoreFactory {}
impl IPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>() -> IPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetPropertyStore<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreFactory_Impl::GetPropertyStore(this, core::mem::transmute_copy(&flags), windows_core::from_raw_borrowed(&punkfactory), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Identity: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyStoreFactory_Impl::GetPropertyStoreForKeys(this, core::mem::transmute_copy(&rgkeys), core::mem::transmute_copy(&ckeys), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertySystem_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPropertyDescription(&self, propkey: *const PROPERTYKEY, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyDescriptionByName(&self, pszcanonicalname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertyDescriptionListFromString(&self, pszproplist: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumeratePropertyDescriptions(&self, filteron: PROPDESC_ENUMFILTER, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> windows_core::Result<windows_core::PWSTR>;
    fn RegisterPropertySchema(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterPropertySchema(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RefreshPropertySchema(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertySystem {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertySystem_Vtbl {
    pub const fn new<Identity: IPropertySystem_Impl, const OFFSET: isize>() -> IPropertySystem_Vtbl {
        unsafe extern "system" fn GetPropertyDescription<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::GetPropertyDescription(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcanonicalname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::GetPropertyDescriptionByName(this, core::mem::transmute(&pszcanonicalname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszproplist: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::GetPropertyDescriptionListFromString(this, core::mem::transmute(&pszproplist), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::EnumeratePropertyDescriptions(this, core::mem::transmute_copy(&filteron), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, psztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::FormatForDisplay(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdff), core::mem::transmute_copy(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertySystem_Impl::FormatForDisplayAlloc(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&propvar), core::mem::transmute_copy(&pdff)) {
                Ok(ok__) => {
                    ppszdisplay.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::RegisterPropertySchema(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn UnregisterPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::UnregisterPropertySchema(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn RefreshPropertySchema<Identity: IPropertySystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystem_Impl::RefreshPropertySchema(this).into()
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
pub trait IPropertySystemChangeNotify_Impl: Sized + windows_core::IUnknownImpl {
    fn SchemaRefreshed(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertySystemChangeNotify {}
impl IPropertySystemChangeNotify_Vtbl {
    pub const fn new<Identity: IPropertySystemChangeNotify_Impl, const OFFSET: isize>() -> IPropertySystemChangeNotify_Vtbl {
        unsafe extern "system" fn SchemaRefreshed<Identity: IPropertySystemChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySystemChangeNotify_Impl::SchemaRefreshed(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SchemaRefreshed: SchemaRefreshed::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySystemChangeNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyUI_Impl: Sized + windows_core::IUnknownImpl {
    fn ParsePropertyName(&self, pszname: &windows_core::PCWSTR, pfmtid: *mut windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> windows_core::Result<()>;
    fn GetCannonicalName(&self, fmtid: *const windows_core::GUID, pid: u32, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, fmtid: *const windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetPropertyDescription(&self, fmtid: *const windows_core::GUID, pid: u32, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetDefaultWidth(&self, fmtid: *const windows_core::GUID, pid: u32) -> windows_core::Result<u32>;
    fn GetFlags(&self, fmtid: *const windows_core::GUID, pid: u32) -> windows_core::Result<PROPERTYUI_FLAGS>;
    fn FormatForDisplay(&self, fmtid: *const windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetHelpInfo(&self, fmtid: *const windows_core::GUID, pid: u32, pwszhelpfile: windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPropertyUI {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyUI_Vtbl {
    pub const fn new<Identity: IPropertyUI_Impl, const OFFSET: isize>() -> IPropertyUI_Vtbl {
        unsafe extern "system" fn ParsePropertyName<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pfmtid: *mut windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::ParsePropertyName(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pfmtid), core::mem::transmute_copy(&ppid), core::mem::transmute_copy(&pcheaten)).into()
        }
        unsafe extern "system" fn GetCannonicalName<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::GetCannonicalName(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&pwsztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::GetDisplayName(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pwsztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetPropertyDescription<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::GetPropertyDescription(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&pwsztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDefaultWidth<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, pcxchars: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyUI_Impl::GetDefaultWidth(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid)) {
                Ok(ok__) => {
                    pcxchars.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyUI_Impl::GetFlags(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid)) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, ppropvar: *const core::mem::MaybeUninit<super::super::super::System::Com::StructuredStorage::PROPVARIANT>, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: windows_core::PWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::FormatForDisplay(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&puiff), core::mem::transmute_copy(&pwsztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Identity: IPropertyUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmtid: *const windows_core::GUID, pid: u32, pwszhelpfile: windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyUI_Impl::GetHelpInfo(this, core::mem::transmute_copy(&fmtid), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&pwszhelpfile), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&puhelpid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParsePropertyName: ParsePropertyName::<Identity, OFFSET>,
            GetCannonicalName: GetCannonicalName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetPropertyDescription: GetPropertyDescription::<Identity, OFFSET>,
            GetDefaultWidth: GetDefaultWidth::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyUI as windows_core::Interface>::IID
    }
}
