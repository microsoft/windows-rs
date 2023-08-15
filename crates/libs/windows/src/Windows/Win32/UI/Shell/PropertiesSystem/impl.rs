#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait ICreateObject_Impl: Sized {
    fn CreateObject(&self, clsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICreateObject {}
impl ICreateObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateObject_Impl, const OFFSET: isize>() -> ICreateObject_Vtbl {
        unsafe extern "system" fn CreateObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateObject(::core::mem::transmute_copy(&clsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IDelayedPropertyStoreFactory_Impl: Sized + IPropertyStoreFactory_Impl {
    fn GetDelayedPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDelayedPropertyStoreFactory {}
impl IDelayedPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>() -> IDelayedPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetDelayedPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDelayedPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDelayedPropertyStore(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&dwstoreid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IPropertyStoreFactory_Vtbl::new::<Identity, Impl, OFFSET>(), GetDelayedPropertyStore: GetDelayedPropertyStore::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDelayedPropertyStoreFactory as ::windows_core::ComInterface>::IID || iid == &<IPropertyStoreFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IInitializeWithFile_Impl: Sized {
    fn Initialize(&self, pszfilepath: &::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IInitializeWithFile {}
impl IInitializeWithFile_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithFile_Impl, const OFFSET: isize>() -> IInitializeWithFile_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInitializeWithFile as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInitializeWithStream_Impl: Sized {
    fn Initialize(&self, pstream: ::core::option::Option<&super::super::super::System::Com::IStream>, grfmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IInitializeWithStream {}
#[cfg(feature = "Win32_System_Com")]
impl IInitializeWithStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithStream_Impl, const OFFSET: isize>() -> IInitializeWithStream_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&grfmode)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInitializeWithStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait INamedPropertyStore_Impl: Sized {
    fn GetNamedValue(&self, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetNamedValue(&self, pszname: &::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetNameCount(&self) -> ::windows_core::Result<u32>;
    fn GetNameAt(&self, iprop: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INamedPropertyStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl INamedPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: isize>() -> INamedPropertyStore_Vtbl {
        unsafe extern "system" fn GetNamedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamedValue(::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamedValue(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn GetNameCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameAt(::core::mem::transmute_copy(&iprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamedValue: GetNamedValue::<Identity, Impl, OFFSET>,
            SetNamedValue: SetNamedValue::<Identity, Impl, OFFSET>,
            GetNameCount: GetNameCount::<Identity, Impl, OFFSET>,
            GetNameAt: GetNameAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INamedPropertyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IObjectWithPropertyKey_Impl: Sized {
    fn SetPropertyKey(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectWithPropertyKey {}
impl IObjectWithPropertyKey_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: isize>() -> IObjectWithPropertyKey_Vtbl {
        unsafe extern "system" fn SetPropertyKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyKey(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn GetPropertyKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyKey(::core::mem::transmute_copy(&pkey)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropertyKey: SetPropertyKey::<Identity, Impl, OFFSET>,
            GetPropertyKey: GetPropertyKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectWithPropertyKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPersistSerializedPropStorage_Impl: Sized {
    fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()>;
    fn SetPropertyStorage(&self, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::Result<()>;
    fn GetPropertyStorage(&self, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPersistSerializedPropStorage {}
impl IPersistSerializedPropStorage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>() -> IPersistSerializedPropStorage_Vtbl {
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetPropertyStorage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyStorage(::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn GetPropertyStorage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyStorage(::core::mem::transmute_copy(&ppsps), ::core::mem::transmute_copy(&pcb)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            SetPropertyStorage: SetPropertyStorage::<Identity, Impl, OFFSET>,
            GetPropertyStorage: GetPropertyStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPersistSerializedPropStorage2_Impl: Sized + IPersistSerializedPropStorage_Impl {
    fn GetPropertyStorageSize(&self) -> ::windows_core::Result<u32>;
    fn GetPropertyStorageBuffer(&self, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPersistSerializedPropStorage2 {}
impl IPersistSerializedPropStorage2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>() -> IPersistSerializedPropStorage2_Vtbl {
        unsafe extern "system" fn GetPropertyStorageSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyStorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyStorageBuffer(::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        Self {
            base__: IPersistSerializedPropStorage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyStorageSize: GetPropertyStorageSize::<Identity, Impl, OFFSET>,
            GetPropertyStorageBuffer: GetPropertyStorageBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPersistSerializedPropStorage2 as ::windows_core::ComInterface>::IID || iid == &<IPersistSerializedPropStorage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyChange_Impl: Sized + IObjectWithPropertyKey_Impl {
    fn ApplyToPropVariant(&self, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyChange {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyChange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChange_Impl, const OFFSET: isize>() -> IPropertyChange_Vtbl {
        unsafe extern "system" fn ApplyToPropVariant<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyToPropVariant(::core::mem::transmute_copy(&propvarin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IObjectWithPropertyKey_Vtbl::new::<Identity, Impl, OFFSET>(), ApplyToPropVariant: ApplyToPropVariant::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyChange as ::windows_core::ComInterface>::IID || iid == &<IObjectWithPropertyKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPropertyChangeArray_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn InsertAt(&self, iindex: u32, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn Append(&self, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn AppendOrReplace(&self, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, iindex: u32) -> ::windows_core::Result<()>;
    fn IsKeyInArray(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertyChangeArray {}
impl IPropertyChangeArray_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>() -> IPropertyChangeArray_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoperations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(::core::mem::transmute_copy(&iindex), ::windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn AppendOrReplace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppendOrReplace(::windows_core::from_raw_borrowed(&ppropchange)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&iindex)).into()
        }
        unsafe extern "system" fn IsKeyInArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsKeyInArray(::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            AppendOrReplace: AppendOrReplace::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            IsKeyInArray: IsKeyInArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyChangeArray as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription_Impl: Sized {
    fn GetPropertyKey(&self, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetCanonicalName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPropertyType(&self) -> ::windows_core::Result<u16>;
    fn GetDisplayName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetEditInvitation(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTypeFlags(&self, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS>;
    fn GetViewFlags(&self) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS>;
    fn GetDefaultColumnWidth(&self) -> ::windows_core::Result<u32>;
    fn GetDisplayType(&self) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE>;
    fn GetColumnState(&self) -> ::windows_core::Result<u32>;
    fn GetGroupingRange(&self) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE>;
    fn GetRelativeDescriptionType(&self) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE>;
    fn GetRelativeDescription(&self, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetSortDescription(&self) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION>;
    fn GetSortDescriptionLabel(&self, fdescending: super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetAggregationType(&self) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE>;
    fn GetConditionType(&self, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()>;
    fn GetEnumTypeList(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CoerceToCanonicalValue(&self, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn FormatForDisplay(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsValueCanonical(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyDescription {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescription_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>() -> IPropertyDescription_Vtbl {
        unsafe extern "system" fn GetPropertyKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyKey(::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCanonicalName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvartype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEditInvitation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszinvite: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEditInvitation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszinvite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeFlags(::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdtflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdvflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultColumnWidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcxchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdisplaytype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupingRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroupingRange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRelativeDescriptionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prdt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRelativeDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRelativeDescription(::core::mem::transmute_copy(&propvar1), ::core::mem::transmute_copy(&propvar2), ::core::mem::transmute_copy(&ppszdesc1), ::core::mem::transmute_copy(&ppszdesc2)).into()
        }
        unsafe extern "system" fn GetSortDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSortDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSortDescriptionLabel(::core::mem::transmute_copy(&fdescending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAggregationType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAggregationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paggtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConditionType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConditionType(::core::mem::transmute_copy(&pcontype), ::core::mem::transmute_copy(&popdefault)).into()
        }
        unsafe extern "system" fn GetEnumTypeList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEnumTypeList(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CoerceToCanonicalValue(::core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatForDisplay(::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValueCanonical<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValueCanonical(::core::mem::transmute_copy(&propvar)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyKey: GetPropertyKey::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetEditInvitation: GetEditInvitation::<Identity, Impl, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, Impl, OFFSET>,
            GetViewFlags: GetViewFlags::<Identity, Impl, OFFSET>,
            GetDefaultColumnWidth: GetDefaultColumnWidth::<Identity, Impl, OFFSET>,
            GetDisplayType: GetDisplayType::<Identity, Impl, OFFSET>,
            GetColumnState: GetColumnState::<Identity, Impl, OFFSET>,
            GetGroupingRange: GetGroupingRange::<Identity, Impl, OFFSET>,
            GetRelativeDescriptionType: GetRelativeDescriptionType::<Identity, Impl, OFFSET>,
            GetRelativeDescription: GetRelativeDescription::<Identity, Impl, OFFSET>,
            GetSortDescription: GetSortDescription::<Identity, Impl, OFFSET>,
            GetSortDescriptionLabel: GetSortDescriptionLabel::<Identity, Impl, OFFSET>,
            GetAggregationType: GetAggregationType::<Identity, Impl, OFFSET>,
            GetConditionType: GetConditionType::<Identity, Impl, OFFSET>,
            GetEnumTypeList: GetEnumTypeList::<Identity, Impl, OFFSET>,
            CoerceToCanonicalValue: CoerceToCanonicalValue::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            IsValueCanonical: IsValueCanonical::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription2_Impl: Sized + IPropertyDescription_Impl {
    fn GetImageReferenceForValue(&self, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyDescription2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescription2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription2_Impl, const OFFSET: isize>() -> IPropertyDescription2_Vtbl {
        unsafe extern "system" fn GetImageReferenceForValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageReferenceForValue(::core::mem::transmute_copy(&propvar)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageres, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPropertyDescription_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImageReferenceForValue: GetImageReferenceForValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescription2 as ::windows_core::ComInterface>::IID || iid == &<IPropertyDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionAliasInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSortByAlias(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAdditionalSortByAliases(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyDescriptionAliasInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionAliasInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionAliasInfo_Vtbl {
        unsafe extern "system" fn GetSortByAlias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSortByAlias(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdditionalSortByAliases(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: IPropertyDescription_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSortByAlias: GetSortByAlias::<Identity, Impl, OFFSET>,
            GetAdditionalSortByAliases: GetAdditionalSortByAliases::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionAliasInfo as ::windows_core::ComInterface>::IID || iid == &<IPropertyDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPropertyDescriptionList_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertyDescriptionList {}
impl IPropertyDescriptionList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: isize>() -> IPropertyDescriptionList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&ielem), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionRelatedPropertyInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetRelatedProperty(&self, pszrelationshipname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyDescriptionRelatedPropertyInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionRelatedPropertyInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionRelatedPropertyInfo_Vtbl {
        unsafe extern "system" fn GetRelatedProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrelationshipname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRelatedProperty(::core::mem::transmute(&pszrelationshipname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: IPropertyDescription_Vtbl::new::<Identity, Impl, OFFSET>(), GetRelatedProperty: GetRelatedProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionRelatedPropertyInfo as ::windows_core::ComInterface>::IID || iid == &<IPropertyDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionSearchInfo_Impl: Sized + IPropertyDescription_Impl {
    fn GetSearchInfoFlags(&self) -> ::windows_core::Result<PROPDESC_SEARCHINFO_FLAGS>;
    fn GetColumnIndexType(&self) -> ::windows_core::Result<PROPDESC_COLUMNINDEX_TYPE>;
    fn GetProjectionString(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMaxSize(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyDescriptionSearchInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl IPropertyDescriptionSearchInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>() -> IPropertyDescriptionSearchInfo_Vtbl {
        unsafe extern "system" fn GetSearchInfoFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSearchInfoFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsiflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnIndexType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnIndexType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdcitype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProjectionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszprojection: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProjectionString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszprojection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmaxsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPropertyDescription_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSearchInfoFlags: GetSearchInfoFlags::<Identity, Impl, OFFSET>,
            GetColumnIndexType: GetColumnIndexType::<Identity, Impl, OFFSET>,
            GetProjectionString: GetProjectionString::<Identity, Impl, OFFSET>,
            GetMaxSize: GetMaxSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyDescriptionSearchInfo as ::windows_core::ComInterface>::IID || iid == &<IPropertyDescription as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType_Impl: Sized {
    fn GetEnumType(&self) -> ::windows_core::Result<PROPENUMTYPE>;
    fn GetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeMinValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeSetValue(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetDisplayText(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyEnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>() -> IPropertyEnumType_Vtbl {
        unsafe extern "system" fn GetEnumType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnumType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penumtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeMinValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRangeMinValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeSetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRangeSetValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnumType: GetEnumType::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetRangeMinValue: GetRangeMinValue::<Identity, Impl, OFFSET>,
            GetRangeSetValue: GetRangeSetValue::<Identity, Impl, OFFSET>,
            GetDisplayText: GetDisplayText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyEnumType as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType2_Impl: Sized + IPropertyEnumType_Impl {
    fn GetImageReference(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyEnumType2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumType2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType2_Impl, const OFFSET: isize>() -> IPropertyEnumType2_Vtbl {
        unsafe extern "system" fn GetImageReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageres, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IPropertyEnumType_Vtbl::new::<Identity, Impl, OFFSET>(), GetImageReference: GetImageReference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyEnumType2 as ::windows_core::ComInterface>::IID || iid == &<IPropertyEnumType as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumTypeList_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetConditionAt(&self, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FindMatchingIndex(&self, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyEnumTypeList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyEnumTypeList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>() -> IPropertyEnumTypeList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&itype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetConditionAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConditionAt(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FindMatchingIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindMatchingIndex(::core::mem::transmute_copy(&propvarcmp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetConditionAt: GetConditionAt::<Identity, Impl, OFFSET>,
            FindMatchingIndex: FindMatchingIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyEnumTypeList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStore_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetValue(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>() -> IPropertyStore_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&iprop), ::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStoreCache_Impl: Sized + IPropertyStore_Impl {
    fn GetState(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<PSC_STATE>;
    fn GetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::Result<()>;
    fn SetState(&self, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::Result<()>;
    fn SetValueAndState(&self, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyStoreCache {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyStoreCache_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: isize>() -> IPropertyStoreCache_Vtbl {
        unsafe extern "system" fn GetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAndState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValueAndState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetValueAndState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueAndState(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: IPropertyStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetValueAndState: GetValueAndState::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            SetValueAndState: SetValueAndState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCache as ::windows_core::ComInterface>::IID || iid == &<IPropertyStore as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPropertyStoreCapabilities_Impl: Sized {
    fn IsPropertyWritable(&self, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertyStoreCapabilities {}
impl IPropertyStoreCapabilities_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCapabilities_Impl, const OFFSET: isize>() -> IPropertyStoreCapabilities_Vtbl {
        unsafe extern "system" fn IsPropertyWritable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPropertyWritable(::core::mem::transmute_copy(&key)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsPropertyWritable: IsPropertyWritable::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCapabilities as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPropertyStoreFactory_Impl: Sized {
    fn GetPropertyStore(&self, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyStoreForKeys(&self, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertyStoreFactory {}
impl IPropertyStoreFactory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: isize>() -> IPropertyStoreFactory_Vtbl {
        unsafe extern "system" fn GetPropertyStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyStore(::core::mem::transmute_copy(&flags), ::windows_core::from_raw_borrowed(&punkfactory), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyStoreForKeys(::core::mem::transmute_copy(&rgkeys), ::core::mem::transmute_copy(&ckeys), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyStore: GetPropertyStore::<Identity, Impl, OFFSET>,
            GetPropertyStoreForKeys: GetPropertyStoreForKeys::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyStoreFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertySystem_Impl: Sized {
    fn GetPropertyDescription(&self, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyDescriptionByName(&self, pszcanonicalname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyDescriptionListFromString(&self, pszproplist: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumeratePropertyDescriptions(&self, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FormatForDisplay(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn FormatForDisplayAlloc(&self, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn RegisterPropertySchema(&self, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterPropertySchema(&self, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RefreshPropertySchema(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertySystem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertySystem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>() -> IPropertySystem_Vtbl {
        unsafe extern "system" fn GetPropertyDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyDescription(::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcanonicalname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyDescriptionByName(::core::mem::transmute(&pszcanonicalname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszproplist: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyDescriptionListFromString(::core::mem::transmute(&pszproplist), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumeratePropertyDescriptions(::core::mem::transmute_copy(&filteron), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FormatForDisplay(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff), ::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatForDisplayAlloc(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPropertySchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterPropertySchema(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn UnregisterPropertySchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterPropertySchema(::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn RefreshPropertySchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshPropertySchema().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyDescription: GetPropertyDescription::<Identity, Impl, OFFSET>,
            GetPropertyDescriptionByName: GetPropertyDescriptionByName::<Identity, Impl, OFFSET>,
            GetPropertyDescriptionListFromString: GetPropertyDescriptionListFromString::<Identity, Impl, OFFSET>,
            EnumeratePropertyDescriptions: EnumeratePropertyDescriptions::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            FormatForDisplayAlloc: FormatForDisplayAlloc::<Identity, Impl, OFFSET>,
            RegisterPropertySchema: RegisterPropertySchema::<Identity, Impl, OFFSET>,
            UnregisterPropertySchema: UnregisterPropertySchema::<Identity, Impl, OFFSET>,
            RefreshPropertySchema: RefreshPropertySchema::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertySystem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
pub trait IPropertySystemChangeNotify_Impl: Sized {
    fn SchemaRefreshed(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPropertySystemChangeNotify {}
impl IPropertySystemChangeNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystemChangeNotify_Impl, const OFFSET: isize>() -> IPropertySystemChangeNotify_Vtbl {
        unsafe extern "system" fn SchemaRefreshed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySystemChangeNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SchemaRefreshed().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SchemaRefreshed: SchemaRefreshed::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertySystemChangeNotify as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyUI_Impl: Sized {
    fn ParsePropertyName(&self, pszname: &::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::Result<()>;
    fn GetCannonicalName(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetDisplayName(&self, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetPropertyDescription(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetDefaultWidth(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<u32>;
    fn GetFlags(&self, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<PROPERTYUI_FLAGS>;
    fn FormatForDisplay(&self, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetHelpInfo(&self, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPropertyUI {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IPropertyUI_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>() -> IPropertyUI_Vtbl {
        unsafe extern "system" fn ParsePropertyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParsePropertyName(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pfmtid), ::core::mem::transmute_copy(&ppid), ::core::mem::transmute_copy(&pcheaten)).into()
        }
        unsafe extern "system" fn GetCannonicalName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCannonicalName(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayName(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetPropertyDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyDescription(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetDefaultWidth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultWidth(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcxchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FormatForDisplay(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&puiff), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHelpInfo(::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwszhelpfile), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&puhelpid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParsePropertyName: ParsePropertyName::<Identity, Impl, OFFSET>,
            GetCannonicalName: GetCannonicalName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetPropertyDescription: GetPropertyDescription::<Identity, Impl, OFFSET>,
            GetDefaultWidth: GetDefaultWidth::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPropertyUI as ::windows_core::ComInterface>::IID
    }
}
