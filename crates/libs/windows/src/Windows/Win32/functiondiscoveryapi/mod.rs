#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FDQUERYCONTEXT(pub super::winnt::DWORDLONG);
pub const FD_LONGHORN: u32 = 1;
windows_core::imp::define_interface!(IFunctionDiscovery, IFunctionDiscovery_Vtbl, 0x4df99b70_e148_4432_b004_4c9eeb535a5e);
windows_core::imp::interface_hierarchy!(IFunctionDiscovery, windows_core::IUnknown);
impl IFunctionDiscovery {
    pub unsafe fn GetInstanceCollection(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: bool) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceCollection)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory, fincludeallsubcategories.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstance)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateInstanceCollectionQuery<P3>(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: bool, pifunctiondiscoverynotification: P3, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>
    where
        P3: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceCollectionQuery)(windows_core::Interface::as_raw(self), pszcategory, pszsubcategory, fincludeallsubcategories.into(), pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateInstanceQuery<P1>(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: P1, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>
    where
        P1: windows_core::Param<IFunctionDiscoveryNotification>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceQuery)(windows_core::Interface::as_raw(self), pszfunctioninstanceidentity, pifunctiondiscoverynotification.param().abi(), pfdqcquerycontext as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory, pszcategoryidentity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveInstance)(windows_core::Interface::as_raw(self), enumsystemvisibility, pszcategory, pszsubcategory, pszcategoryidentity) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInstanceCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_servprov")]
    pub GetInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    GetInstance: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, windows_core::BOOL, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateInstanceCollectionQuery: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateInstanceQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut core::ffi::c_void, *mut FDQUERYCONTEXT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateInstanceQuery: usize,
    #[cfg(feature = "Win32_servprov")]
    pub AddInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(*mut core::ffi::c_void, SystemVisibilityFlags, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait IFunctionDiscovery_Impl: windows_core::IUnknownImpl {
    fn GetInstanceCollection(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL) -> windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceCollectionQuery>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: windows_core::Ref<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut FDQUERYCONTEXT) -> windows_core::Result<IFunctionInstanceQuery>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInstanceCollection<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::GetInstanceCollection(this, core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories)) {
                    Ok(ok__) => {
                        ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const u16, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::GetInstance(this, core::mem::transmute_copy(&pszfunctioninstanceidentity)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategory: *const u16, pszsubcategory: *const u16, fincludeallsubcategories: windows_core::BOOL, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancecollectionquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::CreateInstanceCollectionQuery(this, core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&fincludeallsubcategories), core::mem::transmute_copy(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                    Ok(ok__) => {
                        ppifunctioninstancecollectionquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfunctioninstanceidentity: *const u16, pifunctiondiscoverynotification: *mut core::ffi::c_void, pfdqcquerycontext: *mut FDQUERYCONTEXT, ppifunctioninstancequery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::CreateInstanceQuery(this, core::mem::transmute_copy(&pszfunctioninstanceidentity), core::mem::transmute_copy(&pifunctiondiscoverynotification), core::mem::transmute_copy(&pfdqcquerycontext)) {
                    Ok(ok__) => {
                        ppifunctioninstancequery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionDiscovery_Impl::AddInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&pszcategoryidentity)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: *const u16, pszsubcategory: *const u16, pszcategoryidentity: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscovery_Impl::RemoveInstance(this, core::mem::transmute_copy(&enumsystemvisibility), core::mem::transmute_copy(&pszcategory), core::mem::transmute_copy(&pszsubcategory), core::mem::transmute_copy(&pszcategoryidentity)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Identity, OFFSET>,
            GetInstance: GetInstance::<Identity, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, OFFSET>,
            AddInstance: AddInstance::<Identity, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscovery as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IFunctionDiscovery {}
windows_core::imp::define_interface!(IFunctionDiscoveryNotification, IFunctionDiscoveryNotification_Vtbl, 0x5f6c1ba8_5330_422e_a368_572b244d3f87);
windows_core::imp::interface_hierarchy!(IFunctionDiscoveryNotification, windows_core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
    pub unsafe fn OnUpdate<P2>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr, fdqcquerycontext, pszprovider) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), dweventid, fdqcquerycontext, pszprovider) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, QueryUpdateAction, FDQUERYCONTEXT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_servprov", feature = "Win32_winnt")))]
    OnUpdate: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OnError: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, FDQUERYCONTEXT, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OnEvent: usize,
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
pub trait IFunctionDiscoveryNotification_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn OnError(&self, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdate<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: FDQUERYCONTEXT, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnUpdate(this, core::mem::transmute_copy(&enumqueryupdateaction), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pifunctioninstance)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnError(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pszprovider)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventid: u32, fdqcquerycontext: FDQUERYCONTEXT, pszprovider: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionDiscoveryNotification_Impl::OnEvent(this, core::mem::transmute_copy(&dweventid), core::mem::transmute_copy(&fdqcquerycontext), core::mem::transmute_copy(&pszprovider)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_servprov", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IFunctionDiscoveryNotification {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionDiscoveryProviderRefresh(pub u8);
#[cfg(feature = "Win32_servprov")]
windows_core::imp::define_interface!(IFunctionInstance, IFunctionInstance_Vtbl, 0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
#[cfg(feature = "Win32_servprov")]
impl core::ops::Deref for IFunctionInstance {
    type Target = super::servprov::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_servprov")]
windows_core::imp::interface_hierarchy!(IFunctionInstance, windows_core::IUnknown, super::servprov::IServiceProvider);
#[cfg(feature = "Win32_servprov")]
impl IFunctionInstance {
    pub unsafe fn GetID(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderInstanceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), dwstgaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), ppszcomemcategory as _, ppszcomemsubcategory as _) }
    }
}
#[cfg(feature = "Win32_servprov")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::servprov::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_propsys")]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16, *mut *mut u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
pub trait IFunctionInstance_Impl: super::servprov::IServiceProvider_Impl {
    fn GetID(&self) -> windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> windows_core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: IFunctionInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::GetID(this) {
                    Ok(ok__) => {
                        ppszcomemidentity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::GetProviderInstanceID(this) {
                    Ok(ok__) => {
                        ppszcomemproviderinstanceidentity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstgaccess: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstance_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&dwstgaccess)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstance_Impl::GetCategory(this, core::mem::transmute_copy(&ppszcomemcategory), core::mem::transmute_copy(&ppszcomemsubcategory)).into()
            }
        }
        Self {
            base__: super::servprov::IServiceProvider_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstance as windows_core::Interface>::IID || iid == &<super::servprov::IServiceProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_propsys", feature = "Win32_servprov"))]
impl windows_core::RuntimeName for IFunctionInstance {}
windows_core::imp::define_interface!(IFunctionInstanceCollection, IFunctionInstanceCollection_Vtbl, 0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollection, windows_core::IUnknown);
impl IFunctionInstanceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Add<P0>(&self, pifunctioninstance: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFunctionInstance>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pifunctioninstance.param().abi()) }
    }
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_servprov")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Get: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Item: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Add: usize,
    #[cfg(feature = "Win32_servprov")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_servprov")]
pub trait IFunctionInstanceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<IFunctionInstance>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: windows_core::Ref<IFunctionInstance>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_servprov")]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const u16, pdwindex: *mut u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Get(this, core::mem::transmute_copy(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifunctioninstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::Add(this, core::mem::transmute_copy(&pifunctioninstance)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollection_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_servprov")]
impl windows_core::RuntimeName for IFunctionInstanceCollection {}
windows_core::imp::define_interface!(IFunctionInstanceCollectionQuery, IFunctionInstanceCollectionQuery_Vtbl, 0x57cc6fd2_c09a_4289_bb72_25f04142058e);
windows_core::imp::interface_hierarchy!(IFunctionInstanceCollectionQuery, windows_core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddQueryConstraint)(windows_core::Interface::as_raw(self), pszconstraintname, pszconstraintvalue) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyConstraint)(windows_core::Interface::as_raw(self), key, core::mem::transmute(pv), enumpropertyconstraint) }
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddQueryConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *const super::propidlbase::PROPVARIANT, PropertyConstraint) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFunctionInstanceCollectionQuery_Impl: windows_core::IUnknownImpl {
    fn AddQueryConstraint(&self, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::Result<()>;
    fn Execute(&self) -> windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddQueryConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconstraintname: *const u16, pszconstraintvalue: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollectionQuery_Impl::AddQueryConstraint(this, core::mem::transmute_copy(&pszconstraintname), core::mem::transmute_copy(&pszconstraintvalue)).into()
            }
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pv: *const super::propidlbase::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFunctionInstanceCollectionQuery_Impl::AddPropertyConstraint(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&enumpropertyconstraint)).into()
            }
        }
        unsafe extern "system" fn Execute<Identity: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstancecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceCollectionQuery_Impl::Execute(this) {
                    Ok(ok__) => {
                        ppifunctioninstancecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Identity, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFunctionInstanceCollectionQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceCollectionQuery2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceCollectionQueryCollection(pub u8);
windows_core::imp::define_interface!(IFunctionInstanceQuery, IFunctionInstanceQuery_Vtbl, 0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
windows_core::imp::interface_hierarchy!(IFunctionInstanceQuery, windows_core::IUnknown);
impl IFunctionInstanceQuery {
    #[cfg(feature = "Win32_servprov")]
    pub unsafe fn Execute(&self) -> windows_core::Result<IFunctionInstance> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_servprov")]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_servprov"))]
    Execute: usize,
}
#[cfg(feature = "Win32_servprov")]
pub trait IFunctionInstanceQuery_Impl: windows_core::IUnknownImpl {
    fn Execute(&self) -> windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_servprov")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: IFunctionInstanceQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: IFunctionInstanceQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifunctioninstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFunctionInstanceQuery_Impl::Execute(this) {
                    Ok(ok__) => {
                        ppifunctioninstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_servprov")]
impl windows_core::RuntimeName for IFunctionInstanceQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IFunctionInstanceQuery2(pub u8);
windows_core::imp::define_interface!(IPropertyStoreCollection, IPropertyStoreCollection_Vtbl, 0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
windows_core::imp::interface_hierarchy!(IPropertyStoreCollection, windows_core::IUnknown);
impl IPropertyStoreCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), pszinstanceidentity, pdwindex as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Add<P0>(&self, pipropertystore: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::propsys::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pipropertystore.param().abi()) }
    }
    #[cfg(feature = "Win32_propsys")]
    pub unsafe fn Remove(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_propsys")]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Get: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Item: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Add: usize,
    #[cfg(feature = "Win32_propsys")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propsys"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_propsys")]
pub trait IPropertyStoreCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Get(&self, pszinstanceidentity: *const u16, pdwindex: *mut u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Item(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Add(&self, pipropertystore: windows_core::Ref<super::propsys::IPropertyStore>) -> windows_core::Result<()>;
    fn Remove(&self, dwindex: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_propsys")]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Get<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinstanceidentity: *const u16, pdwindex: *mut u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Get(this, core::mem::transmute_copy(&pszinstanceidentity), core::mem::transmute_copy(&pdwindex)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Item(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::Add(this, core::mem::transmute_copy(&pipropertystore)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyStoreCollection_Impl::Remove(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pipropertystore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::Delete(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyStoreCollection_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_propsys")]
impl windows_core::RuntimeName for IPropertyStoreCollection {}
pub type PropertyConstraint = i32;
pub const QCT_LAYERED: QueryCategoryType = 1;
pub const QCT_PROVIDER: QueryCategoryType = 0;
pub const QC_CONTAINS: PropertyConstraint = 9;
pub const QC_DOESNOTEXIST: PropertyConstraint = 8;
pub const QC_EQUALS: PropertyConstraint = 0;
pub const QC_EXISTS: PropertyConstraint = 7;
pub const QC_GREATERTHAN: PropertyConstraint = 4;
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = 5;
pub const QC_LESSTHAN: PropertyConstraint = 2;
pub const QC_LESSTHANOREQUAL: PropertyConstraint = 3;
pub const QC_NOTEQUAL: PropertyConstraint = 1;
pub const QC_STARTSWITH: PropertyConstraint = 6;
pub const QUA_ADD: QueryUpdateAction = 0;
pub const QUA_CHANGE: QueryUpdateAction = 2;
pub const QUA_REMOVE: QueryUpdateAction = 1;
pub type QueryCategoryType = i32;
pub type QueryUpdateAction = i32;
pub const SVF_SYSTEM: SystemVisibilityFlags = 0;
pub const SVF_USER: SystemVisibilityFlags = 1;
pub type SystemVisibilityFlags = i32;
