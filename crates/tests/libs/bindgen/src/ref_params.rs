#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IDynamicConceptProviderConcept,
    IDynamicConceptProviderConcept_Vtbl,
    0x95a7f7dd_602e_483f_9d06_a15c0ee13174
);
windows_core::imp::interface_hierarchy!(IDynamicConceptProviderConcept, windows_core::IUnknown);
impl IDynamicConceptProviderConcept {
    pub unsafe fn GetConcept<P0>(
        &self,
        contextobject: P0,
        conceptid: *const windows_core::GUID,
        conceptinterface: *mut Option<windows_core::IUnknown>,
        conceptmetadata: Option<*mut Option<IKeyStore>>,
        hasconcept: *mut bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetConcept)(
                windows_core::Interface::as_raw(self),
                contextobject.param().abi(),
                conceptid,
                core::mem::transmute(conceptinterface),
                conceptmetadata.unwrap_or(core::mem::zeroed()) as _,
                hasconcept as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetConcept<P0, P2, P3>(
        &self,
        contextobject: P0,
        conceptid: *const windows_core::GUID,
        conceptinterface: P2,
        conceptmetadata: P3,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IKeyStore>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetConcept)(
                windows_core::Interface::as_raw(self),
                contextobject.param().abi(),
                conceptid,
                conceptinterface.param().abi(),
                conceptmetadata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn NotifyParent<P0>(&self, parentmodel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyParent)(
                windows_core::Interface::as_raw(self),
                parentmodel.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn NotifyParentChange<P0>(&self, parentmodel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyParentChange)(
                windows_core::Interface::as_raw(self),
                parentmodel.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn NotifyDestruct(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).NotifyDestruct)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicConceptProviderConcept_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConcept: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetConcept: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NotifyParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NotifyParentChange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NotifyDestruct: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDynamicConceptProviderConcept_Impl: windows_core::IUnknownImpl {
    fn GetConcept(
        &self,
        contextobject: windows_core::Ref<IModelObject>,
        conceptid: *const windows_core::GUID,
        conceptinterface: windows_core::OutRef<windows_core::IUnknown>,
        conceptmetadata: windows_core::OutRef<IKeyStore>,
        hasconcept: *mut bool,
    ) -> windows_core::Result<()>;
    fn SetConcept(
        &self,
        contextobject: windows_core::Ref<IModelObject>,
        conceptid: *const windows_core::GUID,
        conceptinterface: windows_core::Ref<windows_core::IUnknown>,
        conceptmetadata: windows_core::Ref<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn NotifyParent(
        &self,
        parentmodel: windows_core::Ref<IModelObject>,
    ) -> windows_core::Result<()>;
    fn NotifyParentChange(
        &self,
        parentmodel: windows_core::Ref<IModelObject>,
    ) -> windows_core::Result<()>;
    fn NotifyDestruct(&self) -> windows_core::Result<()>;
}
impl IDynamicConceptProviderConcept_Vtbl {
    pub const fn new<Identity: IDynamicConceptProviderConcept_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConcept<
            Identity: IDynamicConceptProviderConcept_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextobject: *mut core::ffi::c_void,
            conceptid: *const windows_core::GUID,
            conceptinterface: *mut *mut core::ffi::c_void,
            conceptmetadata: *mut *mut core::ffi::c_void,
            hasconcept: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicConceptProviderConcept_Impl::GetConcept(
                    this,
                    core::mem::transmute_copy(&contextobject),
                    core::mem::transmute_copy(&conceptid),
                    core::mem::transmute_copy(&conceptinterface),
                    core::mem::transmute_copy(&conceptmetadata),
                    core::mem::transmute_copy(&hasconcept),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetConcept<
            Identity: IDynamicConceptProviderConcept_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextobject: *mut core::ffi::c_void,
            conceptid: *const windows_core::GUID,
            conceptinterface: *mut core::ffi::c_void,
            conceptmetadata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicConceptProviderConcept_Impl::SetConcept(
                    this,
                    core::mem::transmute_copy(&contextobject),
                    core::mem::transmute_copy(&conceptid),
                    core::mem::transmute_copy(&conceptinterface),
                    core::mem::transmute_copy(&conceptmetadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn NotifyParent<
            Identity: IDynamicConceptProviderConcept_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parentmodel: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicConceptProviderConcept_Impl::NotifyParent(
                    this,
                    core::mem::transmute_copy(&parentmodel),
                )
                .into()
            }
        }
        unsafe extern "system" fn NotifyParentChange<
            Identity: IDynamicConceptProviderConcept_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parentmodel: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicConceptProviderConcept_Impl::NotifyParentChange(
                    this,
                    core::mem::transmute_copy(&parentmodel),
                )
                .into()
            }
        }
        unsafe extern "system" fn NotifyDestruct<
            Identity: IDynamicConceptProviderConcept_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDynamicConceptProviderConcept_Impl::NotifyDestruct(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConcept: GetConcept::<Identity, OFFSET>,
            SetConcept: SetConcept::<Identity, OFFSET>,
            NotifyParent: NotifyParent::<Identity, OFFSET>,
            NotifyParentChange: NotifyParentChange::<Identity, OFFSET>,
            NotifyDestruct: NotifyDestruct::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicConceptProviderConcept as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDynamicConceptProviderConcept {}
windows_core::imp::define_interface!(
    IKeyStore,
    IKeyStore_Vtbl,
    0x0fc7557d_401d_4fca_9365_da1e9850697c
);
windows_core::imp::interface_hierarchy!(IKeyStore, windows_core::IUnknown);
impl IKeyStore {
    pub unsafe fn GetKey<P0>(
        &self,
        key: P0,
        object: Option<*mut Option<IModelObject>>,
        metadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetKey)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.unwrap_or(core::mem::zeroed()) as _,
                metadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetKey<P0, P1, P2>(
        &self,
        key: P0,
        object: P1,
        metadata: P2,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IModelObject>,
        P2: windows_core::Param<IKeyStore>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetKey)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.param().abi(),
                metadata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetKeyValue<P0>(
        &self,
        key: P0,
        object: Option<*mut Option<IModelObject>>,
        metadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetKeyValue)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.unwrap_or(core::mem::zeroed()) as _,
                metadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetKeyValue<P0, P1>(&self, key: P0, object: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetKeyValue)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn ClearKeys(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearKeys)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetKeyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKeyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ClearKeys: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IKeyStore_Impl: windows_core::IUnknownImpl {
    fn GetKey(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::OutRef<IModelObject>,
        metadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn SetKey(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::Ref<IModelObject>,
        metadata: windows_core::Ref<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn GetKeyValue(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::OutRef<IModelObject>,
        metadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn SetKeyValue(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::Ref<IModelObject>,
    ) -> windows_core::Result<()>;
    fn ClearKeys(&self) -> windows_core::Result<()>;
}
impl IKeyStore_Vtbl {
    pub const fn new<Identity: IKeyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKey<Identity: IKeyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut *mut core::ffi::c_void,
            metadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyStore_Impl::GetKey(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetKey<Identity: IKeyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut core::ffi::c_void,
            metadata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyStore_Impl::SetKey(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetKeyValue<Identity: IKeyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut *mut core::ffi::c_void,
            metadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyStore_Impl::GetKeyValue(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: IKeyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyStore_Impl::SetKeyValue(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                )
                .into()
            }
        }
        unsafe extern "system" fn ClearKeys<Identity: IKeyStore_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyStore_Impl::ClearKeys(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKey: GetKey::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, OFFSET>,
            ClearKeys: ClearKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyStore as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKeyStore {}
windows_core::imp::define_interface!(
    IModelObject,
    IModelObject_Vtbl,
    0xe28c7893_3f4b_4b96_baca_293cdc55f45d
);
windows_core::imp::interface_hierarchy!(IModelObject, windows_core::IUnknown);
impl IModelObject {
    pub unsafe fn GetKeyValue<P0>(
        &self,
        key: P0,
        object: Option<*mut Option<IModelObject>>,
        metadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetKeyValue)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.unwrap_or(core::mem::zeroed()) as _,
                metadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetKeyValue<P0, P1>(&self, key: P0, object: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetKeyValue)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn Dereference(&self) -> windows_core::Result<IModelObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dereference)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TryCastToRuntimeType(&self) -> windows_core::Result<IModelObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryCastToRuntimeType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConcept(
        &self,
        conceptid: *const windows_core::GUID,
        conceptinterface: *mut Option<windows_core::IUnknown>,
        conceptmetadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetConcept)(
                windows_core::Interface::as_raw(self),
                conceptid,
                core::mem::transmute(conceptinterface),
                conceptmetadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetNumberOfParentModels(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfParentModels)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetParentModel(
        &self,
        i: u64,
        model: *mut Option<IModelObject>,
        contextobject: *mut Option<IModelObject>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetParentModel)(
                windows_core::Interface::as_raw(self),
                i,
                core::mem::transmute(model),
                core::mem::transmute(contextobject),
            )
            .ok()
        }
    }
    pub unsafe fn AddParentModel<P0, P1>(
        &self,
        model: P0,
        contextobject: P1,
        r#override: u8,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
        P1: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddParentModel)(
                windows_core::Interface::as_raw(self),
                model.param().abi(),
                contextobject.param().abi(),
                r#override,
            )
            .ok()
        }
    }
    pub unsafe fn RemoveParentModel<P0>(&self, model: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveParentModel)(
                windows_core::Interface::as_raw(self),
                model.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetKey<P0>(
        &self,
        key: P0,
        object: Option<*mut Option<IModelObject>>,
        metadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetKey)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.unwrap_or(core::mem::zeroed()) as _,
                metadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetKeyReference<P0>(
        &self,
        key: P0,
        objectreference: Option<*mut Option<IModelObject>>,
        metadata: Option<*mut Option<IKeyStore>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetKeyReference)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                objectreference.unwrap_or(core::mem::zeroed()) as _,
                metadata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetKey<P0, P1, P2>(
        &self,
        key: P0,
        object: P1,
        metadata: P2,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IModelObject>,
        P2: windows_core::Param<IKeyStore>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetKey)(
                windows_core::Interface::as_raw(self),
                key.param().abi(),
                object.param().abi(),
                metadata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn ClearKeys(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearKeys)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn SetConcept<P1, P2>(
        &self,
        conceptid: *const windows_core::GUID,
        conceptinterface: P1,
        conceptmetadata: P2,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IKeyStore>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetConcept)(
                windows_core::Interface::as_raw(self),
                conceptid,
                conceptinterface.param().abi(),
                conceptmetadata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn ClearConcepts(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearConcepts)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub unsafe fn SetContextForDataModel<P0, P1>(
        &self,
        datamodelobject: P0,
        context: P1,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetContextForDataModel)(
                windows_core::Interface::as_raw(self),
                datamodelobject.param().abi(),
                context.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetContextForDataModel<P0>(
        &self,
        datamodelobject: P0,
    ) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextForDataModel)(
                windows_core::Interface::as_raw(self),
                datamodelobject.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Compare<P0>(
        &self,
        other: P0,
        ppresult: Option<*mut Option<IModelObject>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Compare)(
                windows_core::Interface::as_raw(self),
                other.param().abi(),
                ppresult.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn IsEqualTo<P0>(&self, other: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IModelObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqualTo)(
                windows_core::Interface::as_raw(self),
                other.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IModelObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetContext: usize,
    GetKind: usize,
    GetIntrinsicValue: usize,
    GetIntrinsicValueAs: usize,
    pub GetKeyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKeyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    EnumerateKeyValues: usize,
    GetRawValue: usize,
    EnumerateRawValues: usize,
    pub Dereference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TryCastToRuntimeType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetConcept: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetLocation: usize,
    GetTypeInfo: usize,
    GetTargetInfo: usize,
    pub GetNumberOfParentModels:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetParentModel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u64,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddParentModel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u8,
    ) -> windows_core::HRESULT,
    pub RemoveParentModel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetKeyReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ClearKeys: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    EnumerateKeys: usize,
    EnumerateKeyReferences: usize,
    pub SetConcept: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ClearConcepts: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    GetRawReference: usize,
    EnumerateRawReferences: usize,
    pub SetContextForDataModel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetContextForDataModel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsEqualTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
}
pub trait IModelObject_Impl: windows_core::IUnknownImpl {
    fn GetKeyValue(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::OutRef<IModelObject>,
        metadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn SetKeyValue(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::Ref<IModelObject>,
    ) -> windows_core::Result<()>;
    fn Dereference(&self) -> windows_core::Result<IModelObject>;
    fn TryCastToRuntimeType(&self) -> windows_core::Result<IModelObject>;
    fn GetConcept(
        &self,
        conceptid: *const windows_core::GUID,
        conceptinterface: windows_core::OutRef<windows_core::IUnknown>,
        conceptmetadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn GetNumberOfParentModels(&self) -> windows_core::Result<u64>;
    fn GetParentModel(
        &self,
        i: u64,
        model: windows_core::OutRef<IModelObject>,
        contextobject: windows_core::OutRef<IModelObject>,
    ) -> windows_core::Result<()>;
    fn AddParentModel(
        &self,
        model: windows_core::Ref<IModelObject>,
        contextobject: windows_core::Ref<IModelObject>,
        r#override: u8,
    ) -> windows_core::Result<()>;
    fn RemoveParentModel(&self, model: windows_core::Ref<IModelObject>)
        -> windows_core::Result<()>;
    fn GetKey(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::OutRef<IModelObject>,
        metadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn GetKeyReference(
        &self,
        key: &windows_core::PCWSTR,
        objectreference: windows_core::OutRef<IModelObject>,
        metadata: windows_core::OutRef<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn SetKey(
        &self,
        key: &windows_core::PCWSTR,
        object: windows_core::Ref<IModelObject>,
        metadata: windows_core::Ref<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn ClearKeys(&self) -> windows_core::Result<()>;
    fn SetConcept(
        &self,
        conceptid: *const windows_core::GUID,
        conceptinterface: windows_core::Ref<windows_core::IUnknown>,
        conceptmetadata: windows_core::Ref<IKeyStore>,
    ) -> windows_core::Result<()>;
    fn ClearConcepts(&self) -> windows_core::Result<()>;
    fn SetContextForDataModel(
        &self,
        datamodelobject: windows_core::Ref<IModelObject>,
        context: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetContextForDataModel(
        &self,
        datamodelobject: windows_core::Ref<IModelObject>,
    ) -> windows_core::Result<windows_core::IUnknown>;
    fn Compare(
        &self,
        other: windows_core::Ref<IModelObject>,
        ppresult: windows_core::OutRef<IModelObject>,
    ) -> windows_core::Result<()>;
    fn IsEqualTo(&self, other: windows_core::Ref<IModelObject>) -> windows_core::Result<bool>;
}
impl IModelObject_Vtbl {
    pub const fn new<Identity: IModelObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKeyValue<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut *mut core::ffi::c_void,
            metadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::GetKeyValue(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::SetKeyValue(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                )
                .into()
            }
        }
        unsafe extern "system" fn Dereference<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            object: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IModelObject_Impl::Dereference(this) {
                    Ok(ok__) => {
                        object.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TryCastToRuntimeType<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            runtimetypedobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IModelObject_Impl::TryCastToRuntimeType(this) {
                    Ok(ok__) => {
                        runtimetypedobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConcept<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            conceptid: *const windows_core::GUID,
            conceptinterface: *mut *mut core::ffi::c_void,
            conceptmetadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::GetConcept(
                    this,
                    core::mem::transmute_copy(&conceptid),
                    core::mem::transmute_copy(&conceptinterface),
                    core::mem::transmute_copy(&conceptmetadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetNumberOfParentModels<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            nummodels: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IModelObject_Impl::GetNumberOfParentModels(this) {
                    Ok(ok__) => {
                        nummodels.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentModel<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            i: u64,
            model: *mut *mut core::ffi::c_void,
            contextobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::GetParentModel(
                    this,
                    core::mem::transmute_copy(&i),
                    core::mem::transmute_copy(&model),
                    core::mem::transmute_copy(&contextobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddParentModel<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            model: *mut core::ffi::c_void,
            contextobject: *mut core::ffi::c_void,
            r#override: u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::AddParentModel(
                    this,
                    core::mem::transmute_copy(&model),
                    core::mem::transmute_copy(&contextobject),
                    core::mem::transmute_copy(&r#override),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveParentModel<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            model: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::RemoveParentModel(this, core::mem::transmute_copy(&model)).into()
            }
        }
        unsafe extern "system" fn GetKey<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut *mut core::ffi::c_void,
            metadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::GetKey(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetKeyReference<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            objectreference: *mut *mut core::ffi::c_void,
            metadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::GetKeyReference(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&objectreference),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetKey<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            key: windows_core::PCWSTR,
            object: *mut core::ffi::c_void,
            metadata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::SetKey(
                    this,
                    core::mem::transmute(&key),
                    core::mem::transmute_copy(&object),
                    core::mem::transmute_copy(&metadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn ClearKeys<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::ClearKeys(this).into()
            }
        }
        unsafe extern "system" fn SetConcept<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            conceptid: *const windows_core::GUID,
            conceptinterface: *mut core::ffi::c_void,
            conceptmetadata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::SetConcept(
                    this,
                    core::mem::transmute_copy(&conceptid),
                    core::mem::transmute_copy(&conceptinterface),
                    core::mem::transmute_copy(&conceptmetadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn ClearConcepts<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::ClearConcepts(this).into()
            }
        }
        unsafe extern "system" fn SetContextForDataModel<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            datamodelobject: *mut core::ffi::c_void,
            context: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::SetContextForDataModel(
                    this,
                    core::mem::transmute_copy(&datamodelobject),
                    core::mem::transmute_copy(&context),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetContextForDataModel<
            Identity: IModelObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            datamodelobject: *mut core::ffi::c_void,
            context: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IModelObject_Impl::GetContextForDataModel(
                    this,
                    core::mem::transmute_copy(&datamodelobject),
                ) {
                    Ok(ok__) => {
                        context.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            other: *mut core::ffi::c_void,
            ppresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModelObject_Impl::Compare(
                    this,
                    core::mem::transmute_copy(&other),
                    core::mem::transmute_copy(&ppresult),
                )
                .into()
            }
        }
        unsafe extern "system" fn IsEqualTo<Identity: IModelObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            other: *mut core::ffi::c_void,
            equal: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IModelObject_Impl::IsEqualTo(this, core::mem::transmute_copy(&other)) {
                    Ok(ok__) => {
                        equal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContext: 0,
            GetKind: 0,
            GetIntrinsicValue: 0,
            GetIntrinsicValueAs: 0,
            GetKeyValue: GetKeyValue::<Identity, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, OFFSET>,
            EnumerateKeyValues: 0,
            GetRawValue: 0,
            EnumerateRawValues: 0,
            Dereference: Dereference::<Identity, OFFSET>,
            TryCastToRuntimeType: TryCastToRuntimeType::<Identity, OFFSET>,
            GetConcept: GetConcept::<Identity, OFFSET>,
            GetLocation: 0,
            GetTypeInfo: 0,
            GetTargetInfo: 0,
            GetNumberOfParentModels: GetNumberOfParentModels::<Identity, OFFSET>,
            GetParentModel: GetParentModel::<Identity, OFFSET>,
            AddParentModel: AddParentModel::<Identity, OFFSET>,
            RemoveParentModel: RemoveParentModel::<Identity, OFFSET>,
            GetKey: GetKey::<Identity, OFFSET>,
            GetKeyReference: GetKeyReference::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            ClearKeys: ClearKeys::<Identity, OFFSET>,
            EnumerateKeys: 0,
            EnumerateKeyReferences: 0,
            SetConcept: SetConcept::<Identity, OFFSET>,
            ClearConcepts: ClearConcepts::<Identity, OFFSET>,
            GetRawReference: 0,
            EnumerateRawReferences: 0,
            SetContextForDataModel: SetContextForDataModel::<Identity, OFFSET>,
            GetContextForDataModel: GetContextForDataModel::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            IsEqualTo: IsEqualTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IModelObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IModelObject {}
