#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUTHENTICATION_INFO {
    pub dwSize: u32,
    pub atAuthenticationType: AUTH_TYPE,
    pub pcwszUser: windows_core::PCWSTR,
    pub pcwszPassword: windows_core::PCWSTR,
}
pub type AUTH_TYPE = i32;
pub const CATALOG_PAUSED_REASON_DELAYED_RECOVERY: CatalogPausedReason = 7;
pub const CATALOG_PAUSED_REASON_EXTERNAL: CatalogPausedReason = 9;
pub const CATALOG_PAUSED_REASON_HIGH_CPU: CatalogPausedReason = 2;
pub const CATALOG_PAUSED_REASON_HIGH_IO: CatalogPausedReason = 1;
pub const CATALOG_PAUSED_REASON_HIGH_NTF_RATE: CatalogPausedReason = 3;
pub const CATALOG_PAUSED_REASON_LOW_BATTERY: CatalogPausedReason = 4;
pub const CATALOG_PAUSED_REASON_LOW_DISK: CatalogPausedReason = 6;
pub const CATALOG_PAUSED_REASON_LOW_MEMORY: CatalogPausedReason = 5;
pub const CATALOG_PAUSED_REASON_NONE: CatalogPausedReason = 0;
pub const CATALOG_PAUSED_REASON_UPGRADING: CatalogPausedReason = 10;
pub const CATALOG_PAUSED_REASON_USER_ACTIVE: CatalogPausedReason = 8;
pub const CATALOG_STATUS_FULL_CRAWL: CatalogStatus = 3;
pub const CATALOG_STATUS_IDLE: CatalogStatus = 0;
pub const CATALOG_STATUS_INCREMENTAL_CRAWL: CatalogStatus = 4;
pub const CATALOG_STATUS_PAUSED: CatalogStatus = 1;
pub const CATALOG_STATUS_PROCESSING_NOTIFICATIONS: CatalogStatus = 5;
pub const CATALOG_STATUS_RECOVERING: CatalogStatus = 2;
pub const CATALOG_STATUS_SHUTTING_DOWN: CatalogStatus = 6;
pub const CLUSIONREASON_DEFAULT: CLUSION_REASON = 1;
pub const CLUSIONREASON_GROUPPOLICY: CLUSION_REASON = 3;
pub const CLUSIONREASON_UNKNOWNSCOPE: CLUSION_REASON = 0;
pub const CLUSIONREASON_USER: CLUSION_REASON = 2;
pub type CLUSION_REASON = i32;
pub const CSearchLanguageSupport: windows_core::GUID = windows_core::GUID::from_u128(0x6a68cc80_4337_4dbc_bd27_fbfb1053820b);
pub const CSearchManager: windows_core::GUID = windows_core::GUID::from_u128(0x7d096c5f_ac08_4f1f_beb7_5c22c517ce39);
pub const CSearchRoot: windows_core::GUID = windows_core::GUID::from_u128(0x30766bd2_ea1c_4f28_bf27_0b44e2f68db7);
pub const CSearchScopeRule: windows_core::GUID = windows_core::GUID::from_u128(0xe63de750_3bd7_4be5_9c84_6b4281988c44);
pub type CatalogPausedReason = i32;
pub type CatalogStatus = i32;
pub const FF_INDEXCOMPLEXURLS: FOLLOW_FLAGS = 1;
pub const FF_SUPPRESSINDEXING: FOLLOW_FLAGS = 2;
pub type FOLLOW_FLAGS = i32;
pub const FilterRegistration: windows_core::GUID = windows_core::GUID::from_u128(0x9e175b8d_f52a_11d8_b9a5_505054503030);
windows_core::imp::define_interface!(IEnumSearchRoots, IEnumSearchRoots_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef52);
windows_core::imp::interface_hierarchy!(IEnumSearchRoots, windows_core::IUnknown);
impl IEnumSearchRoots {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<ISearchRoot>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSearchRoots_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSearchRoots_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<ISearchRoot>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSearchRoots>;
}
impl IEnumSearchRoots_Vtbl {
    pub const fn new<Identity: IEnumSearchRoots_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchRoots_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchRoots_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchRoots_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSearchRoots_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSearchRoots_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSearchRoots as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSearchRoots {}
windows_core::imp::define_interface!(IEnumSearchScopeRules, IEnumSearchScopeRules_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef54);
windows_core::imp::interface_hierarchy!(IEnumSearchScopeRules, windows_core::IUnknown);
impl IEnumSearchScopeRules {
    pub unsafe fn Next(&self, celt: u32, pprgelt: *mut Option<ISearchScopeRule>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(pprgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSearchScopeRules_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSearchScopeRules_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pprgelt: windows_core::OutRef<ISearchScopeRule>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSearchScopeRules>;
}
impl IEnumSearchScopeRules_Vtbl {
    pub const fn new<Identity: IEnumSearchScopeRules_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pprgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchScopeRules_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pprgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchScopeRules_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSearchScopeRules_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSearchScopeRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSearchScopeRules_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSearchScopeRules as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSearchScopeRules {}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INCREMENTAL_ACCESS_INFO {
    pub dwSize: u32,
    pub ftLastModifiedTime: super::minwindef::FILETIME,
}
windows_core::imp::define_interface!(IOpLockStatus, IOpLockStatus_Vtbl, 0xc731065d_ac80_11d1_8df3_00c04fb6ef4f);
windows_core::imp::interface_hierarchy!(IOpLockStatus, windows_core::IUnknown);
impl IOpLockStatus {
    pub unsafe fn IsOplockValid(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOplockValid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsOplockBroken(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOplockBroken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetOplockEventHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOplockEventHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpLockStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsOplockValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsOplockBroken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub GetOplockEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetOplockEventHandle: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait IOpLockStatus_Impl: windows_core::IUnknownImpl {
    fn IsOplockValid(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsOplockBroken(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetOplockEventHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "Win32_winnt")]
impl IOpLockStatus_Vtbl {
    pub const fn new<Identity: IOpLockStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsOplockValid<Identity: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisoplockvalid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpLockStatus_Impl::IsOplockValid(this) {
                    Ok(ok__) => {
                        pfisoplockvalid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsOplockBroken<Identity: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisoplockbroken: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpLockStatus_Impl::IsOplockBroken(this) {
                    Ok(ok__) => {
                        pfisoplockbroken.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOplockEventHandle<Identity: IOpLockStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phoplockev: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpLockStatus_Impl::GetOplockEventHandle(this) {
                    Ok(ok__) => {
                        phoplockev.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsOplockValid: IsOplockValid::<Identity, OFFSET>,
            IsOplockBroken: IsOplockBroken::<Identity, OFFSET>,
            GetOplockEventHandle: GetOplockEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpLockStatus as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IOpLockStatus {}
windows_core::imp::define_interface!(IProtocolHandlerSite, IProtocolHandlerSite_Vtbl, 0x0b63e385_9ccc_11d0_bcdb_00805fccce04);
windows_core::imp::interface_hierarchy!(IProtocolHandlerSite, windows_core::IUnknown);
impl IProtocolHandlerSite {
    #[cfg(feature = "Win32_filter")]
    pub unsafe fn GetFilter<P1, P2>(&self, pclsidobj: *const windows_core::GUID, pcwszcontenttype: P1, pcwszextension: P2) -> windows_core::Result<super::filter::IFilter>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), pclsidobj, pcwszcontenttype.param().abi(), pcwszextension.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolHandlerSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_filter")]
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_filter"))]
    GetFilter: usize,
}
#[cfg(feature = "Win32_filter")]
pub trait IProtocolHandlerSite_Impl: windows_core::IUnknownImpl {
    fn GetFilter(&self, pclsidobj: *const windows_core::GUID, pcwszcontenttype: &windows_core::PCWSTR, pcwszextension: &windows_core::PCWSTR) -> windows_core::Result<super::filter::IFilter>;
}
#[cfg(feature = "Win32_filter")]
impl IProtocolHandlerSite_Vtbl {
    pub const fn new<Identity: IProtocolHandlerSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFilter<Identity: IProtocolHandlerSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidobj: *const windows_core::GUID, pcwszcontenttype: windows_core::PCWSTR, pcwszextension: windows_core::PCWSTR, ppfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProtocolHandlerSite_Impl::GetFilter(this, core::mem::transmute_copy(&pclsidobj), core::mem::transmute(&pcwszcontenttype), core::mem::transmute(&pcwszextension)) {
                    Ok(ok__) => {
                        ppfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFilter: GetFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolHandlerSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_filter")]
impl windows_core::RuntimeName for IProtocolHandlerSite {}
windows_core::imp::define_interface!(IRowsetEvents, IRowsetEvents_Vtbl, 0x1551aea5_5d66_4b11_86f5_d5634cb211b9);
windows_core::imp::interface_hierarchy!(IRowsetEvents, windows_core::IUnknown);
impl IRowsetEvents {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OnNewItem(&self, itemid: *const super::propidlbase::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNewItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), newitemstate) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OnChangedItem(&self, itemid: *const super::propidlbase::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChangedItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), rowsetitemstate, changeditemstate) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OnDeletedItem(&self, itemid: *const super::propidlbase::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDeletedItem)(windows_core::Interface::as_raw(self), core::mem::transmute(itemid), deleteditemstate) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn OnRowsetEvent(&self, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnRowsetEvent)(windows_core::Interface::as_raw(self), eventtype, core::mem::transmute(eventdata)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OnNewItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OnNewItem: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OnChangedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, ROWSETEVENT_ITEMSTATE, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OnChangedItem: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OnDeletedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OnDeletedItem: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub OnRowsetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, ROWSETEVENT_TYPE, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    OnRowsetEvent: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IRowsetEvents_Impl: windows_core::IUnknownImpl {
    fn OnNewItem(&self, itemid: *const super::propidlbase::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnChangedItem(&self, itemid: *const super::propidlbase::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnDeletedItem(&self, itemid: *const super::propidlbase::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnRowsetEvent(&self, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IRowsetEvents_Vtbl {
    pub const fn new<Identity: IRowsetEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNewItem<Identity: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const super::propidlbase::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetEvents_Impl::OnNewItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&newitemstate)).into()
            }
        }
        unsafe extern "system" fn OnChangedItem<Identity: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const super::propidlbase::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetEvents_Impl::OnChangedItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&rowsetitemstate), core::mem::transmute_copy(&changeditemstate)).into()
            }
        }
        unsafe extern "system" fn OnDeletedItem<Identity: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const super::propidlbase::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetEvents_Impl::OnDeletedItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&deleteditemstate)).into()
            }
        }
        unsafe extern "system" fn OnRowsetEvent<Identity: IRowsetEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetEvents_Impl::OnRowsetEvent(this, core::mem::transmute_copy(&eventtype), core::mem::transmute_copy(&eventdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNewItem: OnNewItem::<Identity, OFFSET>,
            OnChangedItem: OnChangedItem::<Identity, OFFSET>,
            OnDeletedItem: OnDeletedItem::<Identity, OFFSET>,
            OnRowsetEvent: OnRowsetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IRowsetEvents {}
windows_core::imp::define_interface!(IRowsetPrioritization, IRowsetPrioritization_Vtbl, 0x42811652_079d_481b_87a2_09a69ecc5f44);
windows_core::imp::interface_hierarchy!(IRowsetPrioritization, windows_core::IUnknown);
impl IRowsetPrioritization {
    pub unsafe fn SetScopePriority(&self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScopePriority)(windows_core::Interface::as_raw(self), priority, scopestatisticseventfrequency) }
    }
    pub unsafe fn GetScopePriority(&self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScopePriority)(windows_core::Interface::as_raw(self), priority as _, scopestatisticseventfrequency as _) }
    }
    pub unsafe fn GetScopeStatistics(&self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScopeStatistics)(windows_core::Interface::as_raw(self), indexeddocumentcount as _, oustandingaddcount as _, oustandingmodifycount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRowsetPrioritization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScopePriority: unsafe extern "system" fn(*mut core::ffi::c_void, PRIORITY_LEVEL, u32) -> windows_core::HRESULT,
    pub GetScopePriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PRIORITY_LEVEL, *mut u32) -> windows_core::HRESULT,
    pub GetScopeStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IRowsetPrioritization_Impl: windows_core::IUnknownImpl {
    fn SetScopePriority(&self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::Result<()>;
    fn GetScopePriority(&self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::Result<()>;
    fn GetScopeStatistics(&self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::Result<()>;
}
impl IRowsetPrioritization_Vtbl {
    pub const fn new<Identity: IRowsetPrioritization_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetScopePriority<Identity: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetPrioritization_Impl::SetScopePriority(this, core::mem::transmute_copy(&priority), core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
            }
        }
        unsafe extern "system" fn GetScopePriority<Identity: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetPrioritization_Impl::GetScopePriority(this, core::mem::transmute_copy(&priority), core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
            }
        }
        unsafe extern "system" fn GetScopeStatistics<Identity: IRowsetPrioritization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRowsetPrioritization_Impl::GetScopeStatistics(this, core::mem::transmute_copy(&indexeddocumentcount), core::mem::transmute_copy(&oustandingaddcount), core::mem::transmute_copy(&oustandingmodifycount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScopePriority: SetScopePriority::<Identity, OFFSET>,
            GetScopePriority: GetScopePriority::<Identity, OFFSET>,
            GetScopeStatistics: GetScopeStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetPrioritization as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRowsetPrioritization {}
windows_core::imp::define_interface!(ISearchCatalogManager, ISearchCatalogManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef50);
windows_core::imp::interface_hierarchy!(ISearchCatalogManager, windows_core::IUnknown);
impl ISearchCatalogManager {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> windows_core::Result<*mut super::propidlbase::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(pvalue)) }
    }
    pub unsafe fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCatalogStatus)(windows_core::Interface::as_raw(self), pstatus as _, ppausedreason as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reindex(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reindex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReindexMatchingURLs<P0>(&self, pszpattern: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReindexMatchingURLs)(windows_core::Interface::as_raw(self), pszpattern.param().abi()) }
    }
    pub unsafe fn ReindexSearchRoot<P0>(&self, pszrooturl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReindexSearchRoot)(windows_core::Interface::as_raw(self), pszrooturl.param().abi()) }
    }
    pub unsafe fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConnectTimeout)(windows_core::Interface::as_raw(self), dwconnecttimeout) }
    }
    pub unsafe fn ConnectTimeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDataTimeout(&self, dwdatatimeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDataTimeout)(windows_core::Interface::as_raw(self), dwdatatimeout) }
    }
    pub unsafe fn DataTimeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumberOfItems(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NumberOfItemsToIndex)(windows_core::Interface::as_raw(self), plincrementalcount as _, plnotificationqueue as _, plhighpriorityqueue as _) }
    }
    pub unsafe fn URLBeingIndexed(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).URLBeingIndexed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetURLIndexingState<P0>(&self, pszurl: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURLIndexingState)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPersistentItemsChangedSink(&self) -> windows_core::Result<ISearchPersistentItemsChangedSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPersistentItemsChangedSink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterViewForNotification<P0, P1>(&self, pszview: P0, pviewchangedsink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ISearchViewChangedSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterViewForNotification)(windows_core::Interface::as_raw(self), pszview.param().abi(), pviewchangedsink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemsChangedSink<P0, T>(&self, pisearchnotifyinlinesite: P0, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<ISearchNotifyInlineSite>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetItemsChangedSink)(windows_core::Interface::as_raw(self), pisearchnotifyinlinesite.param().abi(), &T::IID, &mut result__, pguidcatalogresetsignature as _, pguidcheckpointsignature as _, pdwlastcheckpointnumber as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn UnregisterViewForNotification(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterViewForNotification)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn SetExtensionClusion<P0>(&self, pszextension: P0, fexclude: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetExtensionClusion)(windows_core::Interface::as_raw(self), pszextension.param().abi(), fexclude.into()) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn EnumerateExcludedExtensions(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateExcludedExtensions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetQueryHelper(&self) -> windows_core::Result<ISearchQueryHelper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQueryHelper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDiacriticSensitivity(&self, fdiacriticsensitive: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDiacriticSensitivity)(windows_core::Interface::as_raw(self), fdiacriticsensitive.into()) }
    }
    pub unsafe fn DiacriticSensitivity(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiacriticSensitivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCrawlScopeManager(&self) -> windows_core::Result<ISearchCrawlScopeManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCrawlScopeManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCatalogManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetParameter: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetParameter: usize,
    pub GetCatalogStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CatalogStatus, *mut CatalogPausedReason) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reindex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReindexMatchingURLs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReindexSearchRoot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConnectTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDataTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DataTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NumberOfItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfItemsToIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub URLBeingIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetURLIndexingState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetPersistentItemsChangedSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterViewForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemsChangedSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub UnregisterViewForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetExtensionClusion: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub EnumerateExcludedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    EnumerateExcludedExtensions: usize,
    pub GetQueryHelper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub DiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCrawlScopeManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchCatalogManager_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParameter(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<*mut super::propidlbase::PROPVARIANT>;
    fn SetParameter(&self, pszname: &windows_core::PCWSTR, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Reindex(&self) -> windows_core::Result<()>;
    fn ReindexMatchingURLs(&self, pszpattern: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReindexSearchRoot(&self, pszrooturl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> windows_core::Result<()>;
    fn ConnectTimeout(&self) -> windows_core::Result<u32>;
    fn SetDataTimeout(&self, dwdatatimeout: u32) -> windows_core::Result<()>;
    fn DataTimeout(&self) -> windows_core::Result<u32>;
    fn NumberOfItems(&self) -> windows_core::Result<i32>;
    fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::Result<()>;
    fn URLBeingIndexed(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetURLIndexingState(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetPersistentItemsChangedSink(&self) -> windows_core::Result<ISearchPersistentItemsChangedSink>;
    fn RegisterViewForNotification(&self, pszview: &windows_core::PCWSTR, pviewchangedsink: windows_core::Ref<ISearchViewChangedSink>) -> windows_core::Result<u32>;
    fn GetItemsChangedSink(&self, pisearchnotifyinlinesite: windows_core::Ref<ISearchNotifyInlineSite>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::Result<()>;
    fn UnregisterViewForNotification(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn SetExtensionClusion(&self, pszextension: &windows_core::PCWSTR, fexclude: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnumerateExcludedExtensions(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
    fn GetQueryHelper(&self) -> windows_core::Result<ISearchQueryHelper>;
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: windows_core::BOOL) -> windows_core::Result<()>;
    fn DiacriticSensitivity(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCrawlScopeManager(&self) -> windows_core::Result<ISearchCrawlScopeManager>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchCatalogManager_Vtbl {
    pub const fn new<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::Name(this) {
                    Ok(ok__) => {
                        pszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppvalue: *mut *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::GetParameter(this, core::mem::transmute(&pszname)) {
                    Ok(ok__) => {
                        ppvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::SetParameter(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetCatalogStatus<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::GetCatalogStatus(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppausedreason)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Reindex<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::Reindex(this).into()
            }
        }
        unsafe extern "system" fn ReindexMatchingURLs<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::ReindexMatchingURLs(this, core::mem::transmute(&pszpattern)).into()
            }
        }
        unsafe extern "system" fn ReindexSearchRoot<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrooturl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::ReindexSearchRoot(this, core::mem::transmute(&pszrooturl)).into()
            }
        }
        unsafe extern "system" fn SetConnectTimeout<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnecttimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::SetConnectTimeout(this, core::mem::transmute_copy(&dwconnecttimeout)).into()
            }
        }
        unsafe extern "system" fn ConnectTimeout<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnecttimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::ConnectTimeout(this) {
                    Ok(ok__) => {
                        pdwconnecttimeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataTimeout<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdatatimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::SetDataTimeout(this, core::mem::transmute_copy(&dwdatatimeout)).into()
            }
        }
        unsafe extern "system" fn DataTimeout<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdatatimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::DataTimeout(this) {
                    Ok(ok__) => {
                        pdwdatatimeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfItems<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::NumberOfItems(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::NumberOfItemsToIndex(this, core::mem::transmute_copy(&plincrementalcount), core::mem::transmute_copy(&plnotificationqueue), core::mem::transmute_copy(&plhighpriorityqueue)).into()
            }
        }
        unsafe extern "system" fn URLBeingIndexed<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::URLBeingIndexed(this) {
                    Ok(ok__) => {
                        pszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURLIndexingState<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::GetURLIndexingState(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::GetPersistentItemsChangedSink(this) {
                    Ok(ok__) => {
                        ppisearchpersistentitemschangedsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterViewForNotification<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszview: windows_core::PCWSTR, pviewchangedsink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::RegisterViewForNotification(this, core::mem::transmute(&pszview), core::mem::transmute_copy(&pviewchangedsink)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemsChangedSink<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisearchnotifyinlinesite: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::GetItemsChangedSink(this, core::mem::transmute_copy(&pisearchnotifyinlinesite), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv), core::mem::transmute_copy(&pguidcatalogresetsignature), core::mem::transmute_copy(&pguidcheckpointsignature), core::mem::transmute_copy(&pdwlastcheckpointnumber)).into()
            }
        }
        unsafe extern "system" fn UnregisterViewForNotification<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::UnregisterViewForNotification(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn SetExtensionClusion<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszextension: windows_core::PCWSTR, fexclude: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::SetExtensionClusion(this, core::mem::transmute(&pszextension), core::mem::transmute_copy(&fexclude)).into()
            }
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::EnumerateExcludedExtensions(this) {
                    Ok(ok__) => {
                        ppextensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQueryHelper<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchqueryhelper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::GetQueryHelper(this) {
                    Ok(ok__) => {
                        ppsearchqueryhelper.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdiacriticsensitive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager_Impl::SetDiacriticSensitivity(this, core::mem::transmute_copy(&fdiacriticsensitive)).into()
            }
        }
        unsafe extern "system" fn DiacriticSensitivity<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdiacriticsensitive: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::DiacriticSensitivity(this) {
                    Ok(ok__) => {
                        pfdiacriticsensitive.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCrawlScopeManager<Identity: ISearchCatalogManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcrawlscopemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager_Impl::GetCrawlScopeManager(this) {
                    Ok(ok__) => {
                        ppcrawlscopemanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            GetParameter: GetParameter::<Identity, OFFSET>,
            SetParameter: SetParameter::<Identity, OFFSET>,
            GetCatalogStatus: GetCatalogStatus::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Reindex: Reindex::<Identity, OFFSET>,
            ReindexMatchingURLs: ReindexMatchingURLs::<Identity, OFFSET>,
            ReindexSearchRoot: ReindexSearchRoot::<Identity, OFFSET>,
            SetConnectTimeout: SetConnectTimeout::<Identity, OFFSET>,
            ConnectTimeout: ConnectTimeout::<Identity, OFFSET>,
            SetDataTimeout: SetDataTimeout::<Identity, OFFSET>,
            DataTimeout: DataTimeout::<Identity, OFFSET>,
            NumberOfItems: NumberOfItems::<Identity, OFFSET>,
            NumberOfItemsToIndex: NumberOfItemsToIndex::<Identity, OFFSET>,
            URLBeingIndexed: URLBeingIndexed::<Identity, OFFSET>,
            GetURLIndexingState: GetURLIndexingState::<Identity, OFFSET>,
            GetPersistentItemsChangedSink: GetPersistentItemsChangedSink::<Identity, OFFSET>,
            RegisterViewForNotification: RegisterViewForNotification::<Identity, OFFSET>,
            GetItemsChangedSink: GetItemsChangedSink::<Identity, OFFSET>,
            UnregisterViewForNotification: UnregisterViewForNotification::<Identity, OFFSET>,
            SetExtensionClusion: SetExtensionClusion::<Identity, OFFSET>,
            EnumerateExcludedExtensions: EnumerateExcludedExtensions::<Identity, OFFSET>,
            GetQueryHelper: GetQueryHelper::<Identity, OFFSET>,
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, OFFSET>,
            DiacriticSensitivity: DiacriticSensitivity::<Identity, OFFSET>,
            GetCrawlScopeManager: GetCrawlScopeManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCatalogManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchCatalogManager {}
windows_core::imp::define_interface!(ISearchCatalogManager2, ISearchCatalogManager2_Vtbl, 0x7ac3286d_4d1d_4817_84fc_c1c85e3af0d9);
impl core::ops::Deref for ISearchCatalogManager2 {
    type Target = ISearchCatalogManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCatalogManager2, windows_core::IUnknown, ISearchCatalogManager);
impl ISearchCatalogManager2 {
    pub unsafe fn PrioritizeMatchingURLs<P0>(&self, pszpattern: P0, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrioritizeMatchingURLs)(windows_core::Interface::as_raw(self), pszpattern.param().abi(), dwprioritizeflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCatalogManager2_Vtbl {
    pub base__: ISearchCatalogManager_Vtbl,
    pub PrioritizeMatchingURLs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PRIORITIZE_FLAGS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchCatalogManager2_Impl: ISearchCatalogManager_Impl {
    fn PrioritizeMatchingURLs(&self, pszpattern: &windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchCatalogManager2_Vtbl {
    pub const fn new<Identity: ISearchCatalogManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrioritizeMatchingURLs<Identity: ISearchCatalogManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCatalogManager2_Impl::PrioritizeMatchingURLs(this, core::mem::transmute(&pszpattern), core::mem::transmute_copy(&dwprioritizeflags)).into()
            }
        }
        Self { base__: ISearchCatalogManager_Vtbl::new::<Identity, OFFSET>(), PrioritizeMatchingURLs: PrioritizeMatchingURLs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCatalogManager2 as windows_core::Interface>::IID || iid == &<ISearchCatalogManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchCatalogManager2 {}
windows_core::imp::define_interface!(ISearchCatalogManager3, ISearchCatalogManager3_Vtbl, 0xde837e8f_634f_4ab0_bdfc_9fc3a1fc50dc);
impl core::ops::Deref for ISearchCatalogManager3 {
    type Target = ISearchCatalogManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCatalogManager3, windows_core::IUnknown, ISearchCatalogManager, ISearchCatalogManager2);
impl ISearchCatalogManager3 {
    pub unsafe fn IsContainsSemanticSupported(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsContainsSemanticSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCatalogManager3_Vtbl {
    pub base__: ISearchCatalogManager2_Vtbl,
    pub IsContainsSemanticSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchCatalogManager3_Impl: ISearchCatalogManager2_Impl {
    fn IsContainsSemanticSupported(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchCatalogManager3_Vtbl {
    pub const fn new<Identity: ISearchCatalogManager3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsContainsSemanticSupported<Identity: ISearchCatalogManager3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iscontainssemanticsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCatalogManager3_Impl::IsContainsSemanticSupported(this) {
                    Ok(ok__) => {
                        iscontainssemanticsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISearchCatalogManager2_Vtbl::new::<Identity, OFFSET>(), IsContainsSemanticSupported: IsContainsSemanticSupported::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCatalogManager3 as windows_core::Interface>::IID || iid == &<ISearchCatalogManager as windows_core::Interface>::IID || iid == &<ISearchCatalogManager2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchCatalogManager3 {}
windows_core::imp::define_interface!(ISearchCrawlScopeManager, ISearchCrawlScopeManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef55);
windows_core::imp::interface_hierarchy!(ISearchCrawlScopeManager, windows_core::IUnknown);
impl ISearchCrawlScopeManager {
    pub unsafe fn AddDefaultScopeRule<P0>(&self, pszurl: P0, finclude: bool, ffollowflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDefaultScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.into(), ffollowflags) }
    }
    pub unsafe fn AddRoot<P0>(&self, psearchroot: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISearchRoot>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRoot)(windows_core::Interface::as_raw(self), psearchroot.param().abi()) }
    }
    pub unsafe fn RemoveRoot<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveRoot)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn EnumerateRoots(&self) -> windows_core::Result<IEnumSearchRoots> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateRoots)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddHierarchicalScope<P0>(&self, pszurl: P0, finclude: bool, fdefault: bool, foverridechildren: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddHierarchicalScope)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.into(), fdefault.into(), foverridechildren.into()) }
    }
    pub unsafe fn AddUserScopeRule<P0>(&self, pszurl: P0, finclude: bool, foverridechildren: bool, ffollowflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUserScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), finclude.into(), foverridechildren.into(), ffollowflags) }
    }
    pub unsafe fn RemoveScopeRule<P0>(&self, pszrule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveScopeRule)(windows_core::Interface::as_raw(self), pszrule.param().abi()) }
    }
    pub unsafe fn EnumerateScopeRules(&self) -> windows_core::Result<IEnumSearchScopeRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateScopeRules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HasParentScopeRule<P0>(&self, pszurl: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasParentScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HasChildScopeRule<P0>(&self, pszurl: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasChildScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IncludedInCrawlScope<P0>(&self, pszurl: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludedInCrawlScope)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IncludedInCrawlScopeEx<P0>(&self, pszurl: P0, pfisincluded: *mut windows_core::BOOL, preason: *mut CLUSION_REASON) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IncludedInCrawlScopeEx)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pfisincluded as _, preason as _) }
    }
    pub unsafe fn RevertToDefaultScopes(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevertToDefaultScopes)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SaveAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveAll)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetParentScopeVersionId<P0>(&self, pszurl: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentScopeVersionId)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveDefaultScopeRule<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveDefaultScopeRule)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCrawlScopeManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddDefaultScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub AddRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveRoot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumerateRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddHierarchicalScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub AddUserScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub RemoveScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumerateScopeRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasParentScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub HasChildScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IncludedInCrawlScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IncludedInCrawlScopeEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL, *mut CLUSION_REASON) -> windows_core::HRESULT,
    pub RevertToDefaultScopes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentScopeVersionId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub RemoveDefaultScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ISearchCrawlScopeManager_Impl: windows_core::IUnknownImpl {
    fn AddDefaultScopeRule(&self, pszurl: &windows_core::PCWSTR, finclude: windows_core::BOOL, ffollowflags: u32) -> windows_core::Result<()>;
    fn AddRoot(&self, psearchroot: windows_core::Ref<ISearchRoot>) -> windows_core::Result<()>;
    fn RemoveRoot(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateRoots(&self) -> windows_core::Result<IEnumSearchRoots>;
    fn AddHierarchicalScope(&self, pszurl: &windows_core::PCWSTR, finclude: windows_core::BOOL, fdefault: windows_core::BOOL, foverridechildren: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddUserScopeRule(&self, pszurl: &windows_core::PCWSTR, finclude: windows_core::BOOL, foverridechildren: windows_core::BOOL, ffollowflags: u32) -> windows_core::Result<()>;
    fn RemoveScopeRule(&self, pszrule: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateScopeRules(&self) -> windows_core::Result<IEnumSearchScopeRules>;
    fn HasParentScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn HasChildScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn IncludedInCrawlScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn IncludedInCrawlScopeEx(&self, pszurl: &windows_core::PCWSTR, pfisincluded: *mut windows_core::BOOL, preason: *mut CLUSION_REASON) -> windows_core::Result<()>;
    fn RevertToDefaultScopes(&self) -> windows_core::Result<()>;
    fn SaveAll(&self) -> windows_core::Result<()>;
    fn GetParentScopeVersionId(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn RemoveDefaultScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ISearchCrawlScopeManager_Vtbl {
    pub const fn new<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddDefaultScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: windows_core::BOOL, ffollowflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::AddDefaultScopeRule(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&ffollowflags)).into()
            }
        }
        unsafe extern "system" fn AddRoot<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchroot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::AddRoot(this, core::mem::transmute_copy(&psearchroot)).into()
            }
        }
        unsafe extern "system" fn RemoveRoot<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::RemoveRoot(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn EnumerateRoots<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchroots: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::EnumerateRoots(this) {
                    Ok(ok__) => {
                        ppsearchroots.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddHierarchicalScope<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: windows_core::BOOL, fdefault: windows_core::BOOL, foverridechildren: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::AddHierarchicalScope(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&fdefault), core::mem::transmute_copy(&foverridechildren)).into()
            }
        }
        unsafe extern "system" fn AddUserScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: windows_core::BOOL, foverridechildren: windows_core::BOOL, ffollowflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::AddUserScopeRule(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&foverridechildren), core::mem::transmute_copy(&ffollowflags)).into()
            }
        }
        unsafe extern "system" fn RemoveScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrule: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::RemoveScopeRule(this, core::mem::transmute(&pszrule)).into()
            }
        }
        unsafe extern "system" fn EnumerateScopeRules<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchscoperules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::EnumerateScopeRules(this) {
                    Ok(ok__) => {
                        ppsearchscoperules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasParentScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfhasparentrule: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::HasParentScopeRule(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        pfhasparentrule.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasChildScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfhaschildrule: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::HasChildScopeRule(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        pfhaschildrule.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncludedInCrawlScope<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfisincluded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::IncludedInCrawlScope(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        pfisincluded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfisincluded: *mut windows_core::BOOL, preason: *mut CLUSION_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::IncludedInCrawlScopeEx(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&pfisincluded), core::mem::transmute_copy(&preason)).into()
            }
        }
        unsafe extern "system" fn RevertToDefaultScopes<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::RevertToDefaultScopes(this).into()
            }
        }
        unsafe extern "system" fn SaveAll<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::SaveAll(this).into()
            }
        }
        unsafe extern "system" fn GetParentScopeVersionId<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, plscopeid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchCrawlScopeManager_Impl::GetParentScopeVersionId(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        plscopeid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Identity: ISearchCrawlScopeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager_Impl::RemoveDefaultScopeRule(this, core::mem::transmute(&pszurl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddDefaultScopeRule: AddDefaultScopeRule::<Identity, OFFSET>,
            AddRoot: AddRoot::<Identity, OFFSET>,
            RemoveRoot: RemoveRoot::<Identity, OFFSET>,
            EnumerateRoots: EnumerateRoots::<Identity, OFFSET>,
            AddHierarchicalScope: AddHierarchicalScope::<Identity, OFFSET>,
            AddUserScopeRule: AddUserScopeRule::<Identity, OFFSET>,
            RemoveScopeRule: RemoveScopeRule::<Identity, OFFSET>,
            EnumerateScopeRules: EnumerateScopeRules::<Identity, OFFSET>,
            HasParentScopeRule: HasParentScopeRule::<Identity, OFFSET>,
            HasChildScopeRule: HasChildScopeRule::<Identity, OFFSET>,
            IncludedInCrawlScope: IncludedInCrawlScope::<Identity, OFFSET>,
            IncludedInCrawlScopeEx: IncludedInCrawlScopeEx::<Identity, OFFSET>,
            RevertToDefaultScopes: RevertToDefaultScopes::<Identity, OFFSET>,
            SaveAll: SaveAll::<Identity, OFFSET>,
            GetParentScopeVersionId: GetParentScopeVersionId::<Identity, OFFSET>,
            RemoveDefaultScopeRule: RemoveDefaultScopeRule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchCrawlScopeManager {}
windows_core::imp::define_interface!(ISearchCrawlScopeManager2, ISearchCrawlScopeManager2_Vtbl, 0x6292f7ad_4e19_4717_a534_8fc22bcd5ccd);
impl core::ops::Deref for ISearchCrawlScopeManager2 {
    type Target = ISearchCrawlScopeManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCrawlScopeManager2, windows_core::IUnknown, ISearchCrawlScopeManager);
impl ISearchCrawlScopeManager2 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetVersion(&self, plversion: *mut *mut i32, phfilemapping: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), plversion as _, phfilemapping as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCrawlScopeManager2_Vtbl {
    pub base__: ISearchCrawlScopeManager_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i32, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetVersion: usize,
}
#[cfg(feature = "Win32_winnt")]
pub trait ISearchCrawlScopeManager2_Impl: ISearchCrawlScopeManager_Impl {
    fn GetVersion(&self, plversion: *mut *mut i32, phfilemapping: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl ISearchCrawlScopeManager2_Vtbl {
    pub const fn new<Identity: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: ISearchCrawlScopeManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCrawlScopeManager2_Impl::GetVersion(this, core::mem::transmute_copy(&plversion), core::mem::transmute_copy(&phfilemapping)).into()
            }
        }
        Self { base__: ISearchCrawlScopeManager_Vtbl::new::<Identity, OFFSET>(), GetVersion: GetVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager2 as windows_core::Interface>::IID || iid == &<ISearchCrawlScopeManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for ISearchCrawlScopeManager2 {}
windows_core::imp::define_interface!(ISearchItemsChangedSink, ISearchItemsChangedSink_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef58);
windows_core::imp::interface_hierarchy!(ISearchItemsChangedSink, windows_core::IUnknown);
impl ISearchItemsChangedSink {
    pub unsafe fn StartedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn StoppedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StoppedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    #[cfg(feature = "Win32_wtypesbase")]
    pub unsafe fn OnItemsChanged(&self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnItemsChanged)(windows_core::Interface::as_raw(self), dwnumberofchanges, rgdatachangeentries, rgdwdocids as _, rghrcompletioncodes as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchItemsChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypesbase")]
    pub OnItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SEARCH_ITEM_CHANGE, *mut u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypesbase"))]
    OnItemsChanged: usize,
}
#[cfg(feature = "Win32_wtypesbase")]
pub trait ISearchItemsChangedSink_Impl: windows_core::IUnknownImpl {
    fn StartedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_wtypesbase")]
impl ISearchItemsChangedSink_Vtbl {
    pub const fn new<Identity: ISearchItemsChangedSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchItemsChangedSink_Impl::StartedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchItemsChangedSink_Impl::StoppedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ISearchItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchItemsChangedSink_Impl::OnItemsChanged(this, core::mem::transmute_copy(&dwnumberofchanges), core::mem::transmute_copy(&rgdatachangeentries), core::mem::transmute_copy(&rgdwdocids), core::mem::transmute_copy(&rghrcompletioncodes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchItemsChangedSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wtypesbase")]
impl windows_core::RuntimeName for ISearchItemsChangedSink {}
windows_core::imp::define_interface!(ISearchLanguageSupport, ISearchLanguageSupport_Vtbl, 0x24c3cbaa_ebc1_491a_9ef1_9f6d8deb1b8f);
windows_core::imp::interface_hierarchy!(ISearchLanguageSupport, windows_core::IUnknown);
impl ISearchLanguageSupport {
    pub unsafe fn SetDiacriticSensitivity(&self, fdiacriticsensitive: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDiacriticSensitivity)(windows_core::Interface::as_raw(self), fdiacriticsensitive.into()) }
    }
    pub unsafe fn GetDiacriticSensitivity(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDiacriticSensitivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn LoadWordBreaker<T>(&self, lcid: super::winnt::LCID, plcidused: *mut super::winnt::LCID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).LoadWordBreaker)(windows_core::Interface::as_raw(self), lcid, &T::IID, &mut result__, plcidused as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn LoadStemmer<T>(&self, lcid: super::winnt::LCID, plcidused: *mut super::winnt::LCID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).LoadStemmer)(windows_core::Interface::as_raw(self), lcid, &T::IID, &mut result__, plcidused as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn IsPrefixNormalized<P0, P2>(&self, pwcsquerytoken: P0, cwcquerytoken: u32, pwcsdocumenttoken: P2, cwcdocumenttoken: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPrefixNormalized)(windows_core::Interface::as_raw(self), pwcsquerytoken.param().abi(), cwcquerytoken, pwcsdocumenttoken.param().abi(), cwcdocumenttoken, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchLanguageSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDiacriticSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub LoadWordBreaker: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    LoadWordBreaker: usize,
    #[cfg(feature = "Win32_winnt")]
    pub LoadStemmer: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    LoadStemmer: usize,
    pub IsPrefixNormalized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait ISearchLanguageSupport_Impl: windows_core::IUnknownImpl {
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDiacriticSensitivity(&self) -> windows_core::Result<windows_core::BOOL>;
    fn LoadWordBreaker(&self, lcid: super::winnt::LCID, riid: *const windows_core::GUID, ppwordbreaker: *mut *mut core::ffi::c_void, plcidused: *mut super::winnt::LCID) -> windows_core::Result<()>;
    fn LoadStemmer(&self, lcid: super::winnt::LCID, riid: *const windows_core::GUID, ppstemmer: *mut *mut core::ffi::c_void, plcidused: *mut super::winnt::LCID) -> windows_core::Result<()>;
    fn IsPrefixNormalized(&self, pwcsquerytoken: &windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: &windows_core::PCWSTR, cwcdocumenttoken: u32) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_winnt")]
impl ISearchLanguageSupport_Vtbl {
    pub const fn new<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdiacriticsensitive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchLanguageSupport_Impl::SetDiacriticSensitivity(this, core::mem::transmute_copy(&fdiacriticsensitive)).into()
            }
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdiacriticsensitive: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchLanguageSupport_Impl::GetDiacriticSensitivity(this) {
                    Ok(ok__) => {
                        pfdiacriticsensitive.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadWordBreaker<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID, riid: *const windows_core::GUID, ppwordbreaker: *mut *mut core::ffi::c_void, plcidused: *mut super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchLanguageSupport_Impl::LoadWordBreaker(this, core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppwordbreaker), core::mem::transmute_copy(&plcidused)).into()
            }
        }
        unsafe extern "system" fn LoadStemmer<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID, riid: *const windows_core::GUID, ppstemmer: *mut *mut core::ffi::c_void, plcidused: *mut super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchLanguageSupport_Impl::LoadStemmer(this, core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstemmer), core::mem::transmute_copy(&plcidused)).into()
            }
        }
        unsafe extern "system" fn IsPrefixNormalized<Identity: ISearchLanguageSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsquerytoken: windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: windows_core::PCWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchLanguageSupport_Impl::IsPrefixNormalized(this, core::mem::transmute(&pwcsquerytoken), core::mem::transmute_copy(&cwcquerytoken), core::mem::transmute(&pwcsdocumenttoken), core::mem::transmute_copy(&cwcdocumenttoken)) {
                    Ok(ok__) => {
                        pulprefixlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, OFFSET>,
            GetDiacriticSensitivity: GetDiacriticSensitivity::<Identity, OFFSET>,
            LoadWordBreaker: LoadWordBreaker::<Identity, OFFSET>,
            LoadStemmer: LoadStemmer::<Identity, OFFSET>,
            IsPrefixNormalized: IsPrefixNormalized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchLanguageSupport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for ISearchLanguageSupport {}
windows_core::imp::define_interface!(ISearchManager, ISearchManager_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef69);
windows_core::imp::interface_hierarchy!(ISearchManager, windows_core::IUnknown);
impl ISearchManager {
    pub unsafe fn GetIndexerVersionStr(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndexerVersionStr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIndexerVersion)(windows_core::Interface::as_raw(self), pdwmajor as _, pdwminor as _) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> windows_core::Result<*mut super::propidlbase::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetParameter)(windows_core::Interface::as_raw(self), pszname.param().abi(), core::mem::transmute(pvalue)) }
    }
    pub unsafe fn ProxyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BypassList(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BypassList)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProxy<P3, P4>(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: bool, dwportnumber: u32, pszproxyname: P3, pszbypasslist: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProxy)(windows_core::Interface::as_raw(self), suseproxy, flocalbypassproxy.into(), dwportnumber, pszproxyname.param().abi(), pszbypasslist.param().abi()) }
    }
    pub unsafe fn GetCatalog<P0>(&self, pszcatalog: P0) -> windows_core::Result<ISearchCatalogManager>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UserAgent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAgent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUserAgent<P0>(&self, pszuseragent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUserAgent)(windows_core::Interface::as_raw(self), pszuseragent.param().abi()) }
    }
    pub unsafe fn UseProxy(&self) -> windows_core::Result<PROXY_ACCESS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseProxy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalBypass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalBypass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PortNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PortNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIndexerVersionStr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetIndexerVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetParameter: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetParameter: usize,
    pub ProxyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetProxy: unsafe extern "system" fn(*mut core::ffi::c_void, PROXY_ACCESS, windows_core::BOOL, u32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UseProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROXY_ACCESS) -> windows_core::HRESULT,
    pub LocalBypass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchManager_Impl: windows_core::IUnknownImpl {
    fn GetIndexerVersionStr(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()>;
    fn GetParameter(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<*mut super::propidlbase::PROPVARIANT>;
    fn SetParameter(&self, pszname: &windows_core::PCWSTR, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn ProxyName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn BypassList(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetProxy(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: windows_core::BOOL, dwportnumber: u32, pszproxyname: &windows_core::PCWSTR, pszbypasslist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<ISearchCatalogManager>;
    fn UserAgent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUserAgent(&self, pszuseragent: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UseProxy(&self) -> windows_core::Result<PROXY_ACCESS>;
    fn LocalBypass(&self) -> windows_core::Result<windows_core::BOOL>;
    fn PortNumber(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchManager_Vtbl {
    pub const fn new<Identity: ISearchManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIndexerVersionStr<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszversionstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::GetIndexerVersionStr(this) {
                    Ok(ok__) => {
                        ppszversionstring.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndexerVersion<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchManager_Impl::GetIndexerVersion(this, core::mem::transmute_copy(&pdwmajor), core::mem::transmute_copy(&pdwminor)).into()
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppvalue: *mut *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::GetParameter(this, core::mem::transmute(&pszname)) {
                    Ok(ok__) => {
                        ppvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameter<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvalue: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchManager_Impl::SetParameter(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn ProxyName<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszproxyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::ProxyName(this) {
                    Ok(ok__) => {
                        ppszproxyname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BypassList<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszbypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::BypassList(this) {
                    Ok(ok__) => {
                        ppszbypasslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProxy<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: windows_core::BOOL, dwportnumber: u32, pszproxyname: windows_core::PCWSTR, pszbypasslist: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchManager_Impl::SetProxy(this, core::mem::transmute_copy(&suseproxy), core::mem::transmute_copy(&flocalbypassproxy), core::mem::transmute_copy(&dwportnumber), core::mem::transmute(&pszproxyname), core::mem::transmute(&pszbypasslist)).into()
            }
        }
        unsafe extern "system" fn GetCatalog<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, ppcatalogmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::GetCatalog(this, core::mem::transmute(&pszcatalog)) {
                    Ok(ok__) => {
                        ppcatalogmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserAgent<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszuseragent: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::UserAgent(this) {
                    Ok(ok__) => {
                        ppszuseragent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserAgent<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuseragent: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchManager_Impl::SetUserAgent(this, core::mem::transmute(&pszuseragent)).into()
            }
        }
        unsafe extern "system" fn UseProxy<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::UseProxy(this) {
                    Ok(ok__) => {
                        puseproxy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalBypass<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflocalbypass: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::LocalBypass(this) {
                    Ok(ok__) => {
                        pflocalbypass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PortNumber<Identity: ISearchManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwportnumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager_Impl::PortNumber(this) {
                    Ok(ok__) => {
                        pdwportnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndexerVersionStr: GetIndexerVersionStr::<Identity, OFFSET>,
            GetIndexerVersion: GetIndexerVersion::<Identity, OFFSET>,
            GetParameter: GetParameter::<Identity, OFFSET>,
            SetParameter: SetParameter::<Identity, OFFSET>,
            ProxyName: ProxyName::<Identity, OFFSET>,
            BypassList: BypassList::<Identity, OFFSET>,
            SetProxy: SetProxy::<Identity, OFFSET>,
            GetCatalog: GetCatalog::<Identity, OFFSET>,
            UserAgent: UserAgent::<Identity, OFFSET>,
            SetUserAgent: SetUserAgent::<Identity, OFFSET>,
            UseProxy: UseProxy::<Identity, OFFSET>,
            LocalBypass: LocalBypass::<Identity, OFFSET>,
            PortNumber: PortNumber::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchManager {}
windows_core::imp::define_interface!(ISearchManager2, ISearchManager2_Vtbl, 0xdbab3f73_db19_4a79_bfc0_a61a93886ddf);
impl core::ops::Deref for ISearchManager2 {
    type Target = ISearchManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchManager2, windows_core::IUnknown, ISearchManager);
impl ISearchManager2 {
    pub unsafe fn CreateCatalog<P0>(&self, pszcatalog: P0) -> windows_core::Result<ISearchCatalogManager>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteCatalog<P0>(&self, pszcatalog: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteCatalog)(windows_core::Interface::as_raw(self), pszcatalog.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchManager2_Vtbl {
    pub base__: ISearchManager_Vtbl,
    pub CreateCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchManager2_Impl: ISearchManager_Impl {
    fn CreateCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<ISearchCatalogManager>;
    fn DeleteCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchManager2_Vtbl {
    pub const fn new<Identity: ISearchManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCatalog<Identity: ISearchManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, ppcatalogmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchManager2_Impl::CreateCatalog(this, core::mem::transmute(&pszcatalog)) {
                    Ok(ok__) => {
                        ppcatalogmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteCatalog<Identity: ISearchManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchManager2_Impl::DeleteCatalog(this, core::mem::transmute(&pszcatalog)).into()
            }
        }
        Self {
            base__: ISearchManager_Vtbl::new::<Identity, OFFSET>(),
            CreateCatalog: CreateCatalog::<Identity, OFFSET>,
            DeleteCatalog: DeleteCatalog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchManager2 as windows_core::Interface>::IID || iid == &<ISearchManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchManager2 {}
windows_core::imp::define_interface!(ISearchNotifyInlineSite, ISearchNotifyInlineSite_Vtbl, 0xb5702e61_e75c_4b64_82a1_6cb4f832fccf);
windows_core::imp::interface_hierarchy!(ISearchNotifyInlineSite, windows_core::IUnknown);
impl ISearchNotifyInlineSite {
    pub unsafe fn OnItemIndexedStatusChange(&self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnItemIndexedStatusChange)(windows_core::Interface::as_raw(self), sipstatus, dwnumentries, rgitemstatusentries) }
    }
    pub unsafe fn OnCatalogStatusChange(&self, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnCatalogStatusChange)(windows_core::Interface::as_raw(self), guidcatalogresetsignature, guidcheckpointsignature, dwlastcheckpointnumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchNotifyInlineSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnItemIndexedStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_INDEXING_PHASE, u32, *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::HRESULT,
    pub OnCatalogStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait ISearchNotifyInlineSite_Impl: windows_core::IUnknownImpl {
    fn OnItemIndexedStatusChange(&self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::Result<()>;
    fn OnCatalogStatusChange(&self, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::Result<()>;
}
impl ISearchNotifyInlineSite_Vtbl {
    pub const fn new<Identity: ISearchNotifyInlineSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnItemIndexedStatusChange<Identity: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchNotifyInlineSite_Impl::OnItemIndexedStatusChange(this, core::mem::transmute_copy(&sipstatus), core::mem::transmute_copy(&dwnumentries), core::mem::transmute_copy(&rgitemstatusentries)).into()
            }
        }
        unsafe extern "system" fn OnCatalogStatusChange<Identity: ISearchNotifyInlineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchNotifyInlineSite_Impl::OnCatalogStatusChange(this, core::mem::transmute_copy(&guidcatalogresetsignature), core::mem::transmute_copy(&guidcheckpointsignature), core::mem::transmute_copy(&dwlastcheckpointnumber)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnItemIndexedStatusChange: OnItemIndexedStatusChange::<Identity, OFFSET>,
            OnCatalogStatusChange: OnCatalogStatusChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchNotifyInlineSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchNotifyInlineSite {}
windows_core::imp::define_interface!(ISearchPersistentItemsChangedSink, ISearchPersistentItemsChangedSink_Vtbl, 0xa2ffdf9b_4758_4f84_b729_df81a1a0612f);
windows_core::imp::interface_hierarchy!(ISearchPersistentItemsChangedSink, windows_core::IUnknown);
impl ISearchPersistentItemsChangedSink {
    pub unsafe fn StartedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn StoppedMonitoringScope<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).StoppedMonitoringScope)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn OnItemsChanged(&self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnItemsChanged)(windows_core::Interface::as_raw(self), dwnumberofchanges, datachangeentries, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPersistentItemsChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StoppedMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SEARCH_ITEM_PERSISTENT_CHANGE, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ISearchPersistentItemsChangedSink_Impl: windows_core::IUnknownImpl {
    fn StartedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE) -> windows_core::Result<windows_core::HRESULT>;
}
impl ISearchPersistentItemsChangedSink_Vtbl {
    pub const fn new<Identity: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchPersistentItemsChangedSink_Impl::StartedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchPersistentItemsChangedSink_Impl::StoppedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ISearchPersistentItemsChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchPersistentItemsChangedSink_Impl::OnItemsChanged(this, core::mem::transmute_copy(&dwnumberofchanges), core::mem::transmute_copy(&datachangeentries)) {
                    Ok(ok__) => {
                        hrcompletioncodes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchPersistentItemsChangedSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchPersistentItemsChangedSink {}
windows_core::imp::define_interface!(ISearchProtocol, ISearchProtocol_Vtbl, 0xc73106ba_ac80_11d1_8df3_00c04fb6ef4f);
windows_core::imp::interface_hierarchy!(ISearchProtocol, windows_core::IUnknown);
impl ISearchProtocol {
    pub unsafe fn Init<P1>(&self, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: P1, pproxyinfo: *const PROXY_INFO) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IProtocolHandlerSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), ptimeoutinfo, pprotocolhandlersite.param().abi(), pproxyinfo) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn CreateAccessor<P0>(&self, pcwszurl: P0, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO) -> windows_core::Result<IUrlAccessor>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAccessor)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pauthenticationinfo, pincrementalaccessinfo, piteminfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CloseAccessor<P0>(&self, paccessor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUrlAccessor>,
    {
        unsafe { (windows_core::Interface::vtable(self).CloseAccessor)(windows_core::Interface::as_raw(self), paccessor.param().abi()) }
    }
    pub unsafe fn ShutDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const TIMEOUT_INFO, *mut core::ffi::c_void, *const PROXY_INFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub CreateAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const AUTHENTICATION_INFO, *const INCREMENTAL_ACCESS_INFO, *const ITEM_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    CreateAccessor: usize,
    pub CloseAccessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_minwindef")]
pub trait ISearchProtocol_Impl: windows_core::IUnknownImpl {
    fn Init(&self, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: windows_core::Ref<IProtocolHandlerSite>, pproxyinfo: *const PROXY_INFO) -> windows_core::Result<()>;
    fn CreateAccessor(&self, pcwszurl: &windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO) -> windows_core::Result<IUrlAccessor>;
    fn CloseAccessor(&self, paccessor: windows_core::Ref<IUrlAccessor>) -> windows_core::Result<()>;
    fn ShutDown(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_minwindef")]
impl ISearchProtocol_Vtbl {
    pub const fn new<Identity: ISearchProtocol_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: *mut core::ffi::c_void, pproxyinfo: *const PROXY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocol_Impl::Init(this, core::mem::transmute_copy(&ptimeoutinfo), core::mem::transmute_copy(&pprotocolhandlersite), core::mem::transmute_copy(&pproxyinfo)).into()
            }
        }
        unsafe extern "system" fn CreateAccessor<Identity: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, ppaccessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchProtocol_Impl::CreateAccessor(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pauthenticationinfo), core::mem::transmute_copy(&pincrementalaccessinfo), core::mem::transmute_copy(&piteminfo)) {
                    Ok(ok__) => {
                        ppaccessor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseAccessor<Identity: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccessor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocol_Impl::CloseAccessor(this, core::mem::transmute_copy(&paccessor)).into()
            }
        }
        unsafe extern "system" fn ShutDown<Identity: ISearchProtocol_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocol_Impl::ShutDown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, OFFSET>,
            CloseAccessor: CloseAccessor::<Identity, OFFSET>,
            ShutDown: ShutDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocol as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwindef")]
impl windows_core::RuntimeName for ISearchProtocol {}
windows_core::imp::define_interface!(ISearchProtocol2, ISearchProtocol2_Vtbl, 0x7789f0b2_b5b2_4722_8b65_5dbd150697a9);
impl core::ops::Deref for ISearchProtocol2 {
    type Target = ISearchProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchProtocol2, windows_core::IUnknown, ISearchProtocol);
impl ISearchProtocol2 {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase"))]
    pub unsafe fn CreateAccessorEx<P0>(&self, pcwszurl: P0, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::wtypesbase::BLOB) -> windows_core::Result<IUrlAccessor>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAccessorEx)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pauthenticationinfo, pincrementalaccessinfo, piteminfo, puserdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchProtocol2_Vtbl {
    pub base__: ISearchProtocol_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase"))]
    pub CreateAccessorEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const AUTHENTICATION_INFO, *const INCREMENTAL_ACCESS_INFO, *const ITEM_INFO, *const super::wtypesbase::BLOB, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase")))]
    CreateAccessorEx: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase"))]
pub trait ISearchProtocol2_Impl: ISearchProtocol_Impl {
    fn CreateAccessorEx(&self, pcwszurl: &windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::wtypesbase::BLOB) -> windows_core::Result<IUrlAccessor>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase"))]
impl ISearchProtocol2_Vtbl {
    pub const fn new<Identity: ISearchProtocol2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateAccessorEx<Identity: ISearchProtocol2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::wtypesbase::BLOB, ppaccessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchProtocol2_Impl::CreateAccessorEx(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pauthenticationinfo), core::mem::transmute_copy(&pincrementalaccessinfo), core::mem::transmute_copy(&piteminfo), core::mem::transmute_copy(&puserdata)) {
                    Ok(ok__) => {
                        ppaccessor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISearchProtocol_Vtbl::new::<Identity, OFFSET>(), CreateAccessorEx: CreateAccessorEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocol2 as windows_core::Interface>::IID || iid == &<ISearchProtocol as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchProtocol2 {}
windows_core::imp::define_interface!(ISearchProtocolThreadContext, ISearchProtocolThreadContext_Vtbl, 0xc73106e1_ac80_11d1_8df3_00c04fb6ef4f);
windows_core::imp::interface_hierarchy!(ISearchProtocolThreadContext, windows_core::IUnknown);
impl ISearchProtocolThreadContext {
    pub unsafe fn ThreadInit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ThreadInit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ThreadShutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ThreadShutdown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ThreadIdle(&self, dwtimeelaspedsincelastcallinms: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ThreadIdle)(windows_core::Interface::as_raw(self), dwtimeelaspedsincelastcallinms) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchProtocolThreadContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadInit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThreadShutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThreadIdle: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISearchProtocolThreadContext_Impl: windows_core::IUnknownImpl {
    fn ThreadInit(&self) -> windows_core::Result<()>;
    fn ThreadShutdown(&self) -> windows_core::Result<()>;
    fn ThreadIdle(&self, dwtimeelaspedsincelastcallinms: u32) -> windows_core::Result<()>;
}
impl ISearchProtocolThreadContext_Vtbl {
    pub const fn new<Identity: ISearchProtocolThreadContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadInit<Identity: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocolThreadContext_Impl::ThreadInit(this).into()
            }
        }
        unsafe extern "system" fn ThreadShutdown<Identity: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocolThreadContext_Impl::ThreadShutdown(this).into()
            }
        }
        unsafe extern "system" fn ThreadIdle<Identity: ISearchProtocolThreadContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchProtocolThreadContext_Impl::ThreadIdle(this, core::mem::transmute_copy(&dwtimeelaspedsincelastcallinms)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadInit: ThreadInit::<Identity, OFFSET>,
            ThreadShutdown: ThreadShutdown::<Identity, OFFSET>,
            ThreadIdle: ThreadIdle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocolThreadContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchProtocolThreadContext {}
windows_core::imp::define_interface!(ISearchQueryHelper, ISearchQueryHelper_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef63);
windows_core::imp::interface_hierarchy!(ISearchQueryHelper, windows_core::IUnknown);
impl ISearchQueryHelper {
    pub unsafe fn ConnectionString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn SetQueryContentLocale(&self, lcid: super::winnt::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQueryContentLocale)(windows_core::Interface::as_raw(self), lcid) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn QueryContentLocale(&self) -> windows_core::Result<super::winnt::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryContentLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn SetQueryKeywordLocale(&self, lcid: super::winnt::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQueryKeywordLocale)(windows_core::Interface::as_raw(self), lcid) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn QueryKeywordLocale(&self) -> windows_core::Result<super::winnt::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryKeywordLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQueryTermExpansion(&self, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQueryTermExpansion)(windows_core::Interface::as_raw(self), expandterms) }
    }
    pub unsafe fn QueryTermExpansion(&self) -> windows_core::Result<SEARCH_TERM_EXPANSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryTermExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuerySyntax(&self, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQuerySyntax)(windows_core::Interface::as_raw(self), querysyntax) }
    }
    pub unsafe fn QuerySyntax(&self) -> windows_core::Result<SEARCH_QUERY_SYNTAX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySyntax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQueryContentProperties<P0>(&self, pszcontentproperties: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQueryContentProperties)(windows_core::Interface::as_raw(self), pszcontentproperties.param().abi()) }
    }
    pub unsafe fn QueryContentProperties(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryContentProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuerySelectColumns<P0>(&self, pszselectcolumns: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQuerySelectColumns)(windows_core::Interface::as_raw(self), pszselectcolumns.param().abi()) }
    }
    pub unsafe fn QuerySelectColumns(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySelectColumns)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQueryWhereRestrictions<P0>(&self, pszrestrictions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQueryWhereRestrictions)(windows_core::Interface::as_raw(self), pszrestrictions.param().abi()) }
    }
    pub unsafe fn QueryWhereRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryWhereRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuerySorting<P0>(&self, pszsorting: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQuerySorting)(windows_core::Interface::as_raw(self), pszsorting.param().abi()) }
    }
    pub unsafe fn QuerySorting(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySorting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GenerateSQLFromUserQuery<P0>(&self, pszquery: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateSQLFromUserQuery)(windows_core::Interface::as_raw(self), pszquery.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn WriteProperties(&self, itemid: ITEMID, dwnumberofcolumns: u32, pcolumns: *const super::wtypes::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteProperties)(windows_core::Interface::as_raw(self), itemid, dwnumberofcolumns, pcolumns, core::mem::transmute(pvalues), pftgathermodifiedtime) }
    }
    pub unsafe fn SetQueryMaxResults(&self, cmaxresults: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQueryMaxResults)(windows_core::Interface::as_raw(self), cmaxresults) }
    }
    pub unsafe fn QueryMaxResults(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryMaxResults)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub SetQueryContentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    SetQueryContentLocale: usize,
    #[cfg(feature = "Win32_winnt")]
    pub QueryContentLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    QueryContentLocale: usize,
    #[cfg(feature = "Win32_winnt")]
    pub SetQueryKeywordLocale: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    SetQueryKeywordLocale: usize,
    #[cfg(feature = "Win32_winnt")]
    pub QueryKeywordLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    QueryKeywordLocale: usize,
    pub SetQueryTermExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_TERM_EXPANSION) -> windows_core::HRESULT,
    pub QueryTermExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SEARCH_TERM_EXPANSION) -> windows_core::HRESULT,
    pub SetQuerySyntax: unsafe extern "system" fn(*mut core::ffi::c_void, SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT,
    pub QuerySyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT,
    pub SetQueryContentProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QueryContentProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQuerySelectColumns: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QuerySelectColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQueryWhereRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QueryWhereRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetQuerySorting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub QuerySorting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GenerateSQLFromUserQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub WriteProperties: unsafe extern "system" fn(*mut core::ffi::c_void, ITEMID, u32, *const super::wtypes::PROPERTYKEY, *const SEARCH_COLUMN_PROPERTIES, *const super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    WriteProperties: usize,
    pub SetQueryMaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub QueryMaxResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISearchQueryHelper_Impl: windows_core::IUnknownImpl {
    fn ConnectionString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQueryContentLocale(&self, lcid: super::winnt::LCID) -> windows_core::Result<()>;
    fn QueryContentLocale(&self) -> windows_core::Result<super::winnt::LCID>;
    fn SetQueryKeywordLocale(&self, lcid: super::winnt::LCID) -> windows_core::Result<()>;
    fn QueryKeywordLocale(&self) -> windows_core::Result<super::winnt::LCID>;
    fn SetQueryTermExpansion(&self, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::Result<()>;
    fn QueryTermExpansion(&self) -> windows_core::Result<SEARCH_TERM_EXPANSION>;
    fn SetQuerySyntax(&self, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::Result<()>;
    fn QuerySyntax(&self) -> windows_core::Result<SEARCH_QUERY_SYNTAX>;
    fn SetQueryContentProperties(&self, pszcontentproperties: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QueryContentProperties(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQuerySelectColumns(&self, pszselectcolumns: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QuerySelectColumns(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQueryWhereRestrictions(&self, pszrestrictions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QueryWhereRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQuerySorting(&self, pszsorting: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QuerySorting(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GenerateSQLFromUserQuery(&self, pszquery: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn WriteProperties(&self, itemid: ITEMID, dwnumberofcolumns: u32, pcolumns: *const super::wtypes::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn SetQueryMaxResults(&self, cmaxresults: i32) -> windows_core::Result<()>;
    fn QueryMaxResults(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISearchQueryHelper_Vtbl {
    pub const fn new<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionString<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconnectionstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::ConnectionString(this) {
                    Ok(ok__) => {
                        pszconnectionstring.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryContentLocale<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryContentLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn QueryContentLocale<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryContentLocale(this) {
                    Ok(ok__) => {
                        plcid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryKeywordLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        unsafe extern "system" fn QueryKeywordLocale<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut super::winnt::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryKeywordLocale(this) {
                    Ok(ok__) => {
                        plcid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryTermExpansion<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryTermExpansion(this, core::mem::transmute_copy(&expandterms)).into()
            }
        }
        unsafe extern "system" fn QueryTermExpansion<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryTermExpansion(this) {
                    Ok(ok__) => {
                        pexpandterms.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuerySyntax<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQuerySyntax(this, core::mem::transmute_copy(&querysyntax)).into()
            }
        }
        unsafe extern "system" fn QuerySyntax<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QuerySyntax(this) {
                    Ok(ok__) => {
                        pquerysyntax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryContentProperties<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontentproperties: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryContentProperties(this, core::mem::transmute(&pszcontentproperties)).into()
            }
        }
        unsafe extern "system" fn QueryContentProperties<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcontentproperties: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryContentProperties(this) {
                    Ok(ok__) => {
                        ppszcontentproperties.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuerySelectColumns<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszselectcolumns: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQuerySelectColumns(this, core::mem::transmute(&pszselectcolumns)).into()
            }
        }
        unsafe extern "system" fn QuerySelectColumns<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszselectcolumns: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QuerySelectColumns(this) {
                    Ok(ok__) => {
                        ppszselectcolumns.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrestrictions: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryWhereRestrictions(this, core::mem::transmute(&pszrestrictions)).into()
            }
        }
        unsafe extern "system" fn QueryWhereRestrictions<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszrestrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryWhereRestrictions(this) {
                    Ok(ok__) => {
                        ppszrestrictions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuerySorting<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsorting: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQuerySorting(this, core::mem::transmute(&pszsorting)).into()
            }
        }
        unsafe extern "system" fn QuerySorting<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszsorting: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QuerySorting(this) {
                    Ok(ok__) => {
                        ppszsorting.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszquery: windows_core::PCWSTR, ppszsql: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::GenerateSQLFromUserQuery(this, core::mem::transmute(&pszquery)) {
                    Ok(ok__) => {
                        ppszsql.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteProperties<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: ITEMID, dwnumberofcolumns: u32, pcolumns: *const super::wtypes::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::WriteProperties(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&dwnumberofcolumns), core::mem::transmute_copy(&pcolumns), core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&pftgathermodifiedtime)).into()
            }
        }
        unsafe extern "system" fn SetQueryMaxResults<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmaxresults: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchQueryHelper_Impl::SetQueryMaxResults(this, core::mem::transmute_copy(&cmaxresults)).into()
            }
        }
        unsafe extern "system" fn QueryMaxResults<Identity: ISearchQueryHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmaxresults: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchQueryHelper_Impl::QueryMaxResults(this) {
                    Ok(ok__) => {
                        pcmaxresults.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, OFFSET>,
            SetQueryContentLocale: SetQueryContentLocale::<Identity, OFFSET>,
            QueryContentLocale: QueryContentLocale::<Identity, OFFSET>,
            SetQueryKeywordLocale: SetQueryKeywordLocale::<Identity, OFFSET>,
            QueryKeywordLocale: QueryKeywordLocale::<Identity, OFFSET>,
            SetQueryTermExpansion: SetQueryTermExpansion::<Identity, OFFSET>,
            QueryTermExpansion: QueryTermExpansion::<Identity, OFFSET>,
            SetQuerySyntax: SetQuerySyntax::<Identity, OFFSET>,
            QuerySyntax: QuerySyntax::<Identity, OFFSET>,
            SetQueryContentProperties: SetQueryContentProperties::<Identity, OFFSET>,
            QueryContentProperties: QueryContentProperties::<Identity, OFFSET>,
            SetQuerySelectColumns: SetQuerySelectColumns::<Identity, OFFSET>,
            QuerySelectColumns: QuerySelectColumns::<Identity, OFFSET>,
            SetQueryWhereRestrictions: SetQueryWhereRestrictions::<Identity, OFFSET>,
            QueryWhereRestrictions: QueryWhereRestrictions::<Identity, OFFSET>,
            SetQuerySorting: SetQuerySorting::<Identity, OFFSET>,
            QuerySorting: QuerySorting::<Identity, OFFSET>,
            GenerateSQLFromUserQuery: GenerateSQLFromUserQuery::<Identity, OFFSET>,
            WriteProperties: WriteProperties::<Identity, OFFSET>,
            SetQueryMaxResults: SetQueryMaxResults::<Identity, OFFSET>,
            QueryMaxResults: QueryMaxResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchQueryHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISearchQueryHelper {}
windows_core::imp::define_interface!(ISearchRoot, ISearchRoot_Vtbl, 0x04c18ccf_1f57_4cbd_88cc_3900f5195ce3);
windows_core::imp::interface_hierarchy!(ISearchRoot, windows_core::IUnknown);
impl ISearchRoot {
    pub unsafe fn SetSchedule<P0>(&self, psztaskarg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSchedule)(windows_core::Interface::as_raw(self), psztaskarg.param().abi()) }
    }
    pub unsafe fn Schedule(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Schedule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRootURL<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRootURL)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
    pub unsafe fn RootURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsHierarchical(&self, fishierarchical: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsHierarchical)(windows_core::Interface::as_raw(self), fishierarchical.into()) }
    }
    pub unsafe fn IsHierarchical(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHierarchical)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProvidesNotifications(&self, fprovidesnotifications: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProvidesNotifications)(windows_core::Interface::as_raw(self), fprovidesnotifications.into()) }
    }
    pub unsafe fn ProvidesNotifications(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProvidesNotifications)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseNotificationsOnly(&self, fusenotificationsonly: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUseNotificationsOnly)(windows_core::Interface::as_raw(self), fusenotificationsonly.into()) }
    }
    pub unsafe fn UseNotificationsOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseNotificationsOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnumerationDepth(&self, dwdepth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnumerationDepth)(windows_core::Interface::as_raw(self), dwdepth) }
    }
    pub unsafe fn EnumerationDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHostDepth(&self, dwdepth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHostDepth)(windows_core::Interface::as_raw(self), dwdepth) }
    }
    pub unsafe fn HostDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFollowDirectories(&self, ffollowdirectories: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFollowDirectories)(windows_core::Interface::as_raw(self), ffollowdirectories.into()) }
    }
    pub unsafe fn FollowDirectories(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FollowDirectories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthenticationType(&self, authtype: AUTH_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticationType)(windows_core::Interface::as_raw(self), authtype) }
    }
    pub unsafe fn AuthenticationType(&self) -> windows_core::Result<AUTH_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthenticationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUser<P0>(&self, pszuser: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUser)(windows_core::Interface::as_raw(self), pszuser.param().abi()) }
    }
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPassword<P0>(&self, pszpassword: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pszpassword.param().abi()) }
    }
    pub unsafe fn Password(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetRootURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RootURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIsHierarchical: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsHierarchical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetProvidesNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ProvidesNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetUseNotificationsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UseNotificationsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnumerationDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumerationDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHostDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HostDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFollowDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub FollowDirectories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, AUTH_TYPE) -> windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUTH_TYPE) -> windows_core::HRESULT,
    pub SetUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISearchRoot_Impl: windows_core::IUnknownImpl {
    fn SetSchedule(&self, psztaskarg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Schedule(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRootURL(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RootURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIsHierarchical(&self, fishierarchical: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsHierarchical(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetProvidesNotifications(&self, fprovidesnotifications: windows_core::BOOL) -> windows_core::Result<()>;
    fn ProvidesNotifications(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetUseNotificationsOnly(&self, fusenotificationsonly: windows_core::BOOL) -> windows_core::Result<()>;
    fn UseNotificationsOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnumerationDepth(&self, dwdepth: u32) -> windows_core::Result<()>;
    fn EnumerationDepth(&self) -> windows_core::Result<u32>;
    fn SetHostDepth(&self, dwdepth: u32) -> windows_core::Result<()>;
    fn HostDepth(&self) -> windows_core::Result<u32>;
    fn SetFollowDirectories(&self, ffollowdirectories: windows_core::BOOL) -> windows_core::Result<()>;
    fn FollowDirectories(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetAuthenticationType(&self, authtype: AUTH_TYPE) -> windows_core::Result<()>;
    fn AuthenticationType(&self) -> windows_core::Result<AUTH_TYPE>;
    fn SetUser(&self, pszuser: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn User(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPassword(&self, pszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Password(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISearchRoot_Vtbl {
    pub const fn new<Identity: ISearchRoot_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSchedule<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztaskarg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetSchedule(this, core::mem::transmute(&psztaskarg)).into()
            }
        }
        unsafe extern "system" fn Schedule<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztaskarg: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::Schedule(this) {
                    Ok(ok__) => {
                        ppsztaskarg.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRootURL<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetRootURL(this, core::mem::transmute(&pszurl)).into()
            }
        }
        unsafe extern "system" fn RootURL<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::RootURL(this) {
                    Ok(ok__) => {
                        ppszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHierarchical<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fishierarchical: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetIsHierarchical(this, core::mem::transmute_copy(&fishierarchical)).into()
            }
        }
        unsafe extern "system" fn IsHierarchical<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfishierarchical: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::IsHierarchical(this) {
                    Ok(ok__) => {
                        pfishierarchical.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProvidesNotifications<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fprovidesnotifications: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetProvidesNotifications(this, core::mem::transmute_copy(&fprovidesnotifications)).into()
            }
        }
        unsafe extern "system" fn ProvidesNotifications<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprovidesnotifications: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::ProvidesNotifications(this) {
                    Ok(ok__) => {
                        pfprovidesnotifications.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusenotificationsonly: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetUseNotificationsOnly(this, core::mem::transmute_copy(&fusenotificationsonly)).into()
            }
        }
        unsafe extern "system" fn UseNotificationsOnly<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfusenotificationsonly: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::UseNotificationsOnly(this) {
                    Ok(ok__) => {
                        pfusenotificationsonly.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnumerationDepth<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdepth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetEnumerationDepth(this, core::mem::transmute_copy(&dwdepth)).into()
            }
        }
        unsafe extern "system" fn EnumerationDepth<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdepth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::EnumerationDepth(this) {
                    Ok(ok__) => {
                        pdwdepth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHostDepth<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdepth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetHostDepth(this, core::mem::transmute_copy(&dwdepth)).into()
            }
        }
        unsafe extern "system" fn HostDepth<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdepth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::HostDepth(this) {
                    Ok(ok__) => {
                        pdwdepth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFollowDirectories<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffollowdirectories: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetFollowDirectories(this, core::mem::transmute_copy(&ffollowdirectories)).into()
            }
        }
        unsafe extern "system" fn FollowDirectories<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffollowdirectories: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::FollowDirectories(this) {
                    Ok(ok__) => {
                        pffollowdirectories.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authtype: AUTH_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetAuthenticationType(this, core::mem::transmute_copy(&authtype)).into()
            }
        }
        unsafe extern "system" fn AuthenticationType<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::AuthenticationType(this) {
                    Ok(ok__) => {
                        pauthtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUser<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuser: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetUser(this, core::mem::transmute(&pszuser)).into()
            }
        }
        unsafe extern "system" fn User<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszuser: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::User(this) {
                    Ok(ok__) => {
                        ppszuser.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPassword<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchRoot_Impl::SetPassword(this, core::mem::transmute(&pszpassword)).into()
            }
        }
        unsafe extern "system" fn Password<Identity: ISearchRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchRoot_Impl::Password(this) {
                    Ok(ok__) => {
                        ppszpassword.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSchedule: SetSchedule::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            SetRootURL: SetRootURL::<Identity, OFFSET>,
            RootURL: RootURL::<Identity, OFFSET>,
            SetIsHierarchical: SetIsHierarchical::<Identity, OFFSET>,
            IsHierarchical: IsHierarchical::<Identity, OFFSET>,
            SetProvidesNotifications: SetProvidesNotifications::<Identity, OFFSET>,
            ProvidesNotifications: ProvidesNotifications::<Identity, OFFSET>,
            SetUseNotificationsOnly: SetUseNotificationsOnly::<Identity, OFFSET>,
            UseNotificationsOnly: UseNotificationsOnly::<Identity, OFFSET>,
            SetEnumerationDepth: SetEnumerationDepth::<Identity, OFFSET>,
            EnumerationDepth: EnumerationDepth::<Identity, OFFSET>,
            SetHostDepth: SetHostDepth::<Identity, OFFSET>,
            HostDepth: HostDepth::<Identity, OFFSET>,
            SetFollowDirectories: SetFollowDirectories::<Identity, OFFSET>,
            FollowDirectories: FollowDirectories::<Identity, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, OFFSET>,
            AuthenticationType: AuthenticationType::<Identity, OFFSET>,
            SetUser: SetUser::<Identity, OFFSET>,
            User: User::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchRoot as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchRoot {}
windows_core::imp::define_interface!(ISearchScopeRule, ISearchScopeRule_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef53);
windows_core::imp::interface_hierarchy!(ISearchScopeRule, windows_core::IUnknown);
impl ISearchScopeRule {
    pub unsafe fn PatternOrURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PatternOrURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsIncluded(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsIncluded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FollowFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FollowFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchScopeRule_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PatternOrURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsIncluded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FollowFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISearchScopeRule_Impl: windows_core::IUnknownImpl {
    fn PatternOrURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsIncluded(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL>;
    fn FollowFlags(&self) -> windows_core::Result<u32>;
}
impl ISearchScopeRule_Vtbl {
    pub const fn new<Identity: ISearchScopeRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PatternOrURL<Identity: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpatternorurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchScopeRule_Impl::PatternOrURL(this) {
                    Ok(ok__) => {
                        ppszpatternorurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsIncluded<Identity: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisincluded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchScopeRule_Impl::IsIncluded(this) {
                    Ok(ok__) => {
                        pfisincluded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDefault<Identity: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchScopeRule_Impl::IsDefault(this) {
                    Ok(ok__) => {
                        pfisdefault.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FollowFlags<Identity: ISearchScopeRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfollowflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchScopeRule_Impl::FollowFlags(this) {
                    Ok(ok__) => {
                        pfollowflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PatternOrURL: PatternOrURL::<Identity, OFFSET>,
            IsIncluded: IsIncluded::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            FollowFlags: FollowFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchScopeRule as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISearchScopeRule {}
windows_core::imp::define_interface!(ISearchViewChangedSink, ISearchViewChangedSink_Vtbl, 0xab310581_ac80_11d1_8df3_00c04fb6ef65);
windows_core::imp::interface_hierarchy!(ISearchViewChangedSink, windows_core::IUnknown);
impl ISearchViewChangedSink {
    #[cfg(feature = "Win32_wtypesbase")]
    pub unsafe fn OnChange(&self, pdwdocid: *const ITEMID, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), pdwdocid, pchange, pfinview) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchViewChangedSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wtypesbase")]
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const ITEMID, *const SEARCH_ITEM_CHANGE, *const windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypesbase"))]
    OnChange: usize,
}
#[cfg(feature = "Win32_wtypesbase")]
pub trait ISearchViewChangedSink_Impl: windows_core::IUnknownImpl {
    fn OnChange(&self, pdwdocid: *const ITEMID, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_wtypesbase")]
impl ISearchViewChangedSink_Vtbl {
    pub const fn new<Identity: ISearchViewChangedSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChange<Identity: ISearchViewChangedSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdocid: *const ITEMID, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchViewChangedSink_Impl::OnChange(this, core::mem::transmute_copy(&pdwdocid), core::mem::transmute_copy(&pchange), core::mem::transmute_copy(&pfinview)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchViewChangedSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wtypesbase")]
impl windows_core::RuntimeName for ISearchViewChangedSink {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ITEMID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ITEM_INFO {
    pub dwSize: u32,
    pub pcwszFromEMail: windows_core::PCWSTR,
    pub pcwszApplicationName: windows_core::PCWSTR,
    pub pcwszCatalogName: windows_core::PCWSTR,
    pub pcwszContentClass: windows_core::PCWSTR,
}
windows_core::imp::define_interface!(IUrlAccessor, IUrlAccessor_Vtbl, 0x0b63e318_9ccc_11d0_bcdb_00805fccce04);
windows_core::imp::interface_hierarchy!(IUrlAccessor, windows_core::IUnknown);
impl IUrlAccessor {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn AddRequestParameter(&self, pspec: *const super::propidlbase::PROPSPEC, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRequestParameter)(windows_core::Interface::as_raw(self), pspec, core::mem::transmute(pvar)) }
    }
    pub unsafe fn GetDocFormat(&self, wszdocformat: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocFormat)(windows_core::Interface::as_raw(self), wszdocformat as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHost(&self, wszhost: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), wszhost as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn IsDirectory(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirectory)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetLastModified(&self) -> windows_core::Result<super::minwindef::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileName(&self, wszfilename: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), wszfilename as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn GetSecurityDescriptor(&self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), psd as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn GetRedirectedURL(&self, wszredirectedurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRedirectedURL)(windows_core::Interface::as_raw(self), wszredirectedurl as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn GetSecurityProvider(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn BindToStream(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BindToStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_filter")]
    pub unsafe fn BindToFilter(&self) -> windows_core::Result<super::filter::IFilter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BindToFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlAccessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub AddRequestParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPSPEC, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    AddRequestParameter: usize,
    pub GetDocFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub IsDirectory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetLastModified: usize,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRedirectedURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub BindToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    BindToStream: usize,
    #[cfg(feature = "Win32_filter")]
    pub BindToFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_filter"))]
    BindToFilter: usize,
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IUrlAccessor_Impl: windows_core::IUnknownImpl {
    fn AddRequestParameter(&self, pspec: *const super::propidlbase::PROPSPEC, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetDocFormat(&self, wszdocformat: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetHost(&self, wszhost: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn IsDirectory(&self) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetLastModified(&self) -> windows_core::Result<super::minwindef::FILETIME>;
    fn GetFileName(&self, wszfilename: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityDescriptor(&self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetRedirectedURL(&self, wszredirectedurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityProvider(&self) -> windows_core::Result<windows_core::GUID>;
    fn BindToStream(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn BindToFilter(&self) -> windows_core::Result<super::filter::IFilter>;
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IUrlAccessor_Vtbl {
    pub const fn new<Identity: IUrlAccessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRequestParameter<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspec: *const super::propidlbase::PROPSPEC, pvar: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::AddRequestParameter(this, core::mem::transmute_copy(&pspec), core::mem::transmute_copy(&pvar)).into()
            }
        }
        unsafe extern "system" fn GetDocFormat<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdocformat: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::GetDocFormat(this, core::mem::transmute_copy(&wszdocformat), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn GetCLSID<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::GetCLSID(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHost<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszhost: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::GetHost(this, core::mem::transmute_copy(&wszhost), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn IsDirectory<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::IsDirectory(this).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::GetSize(this) {
                    Ok(ok__) => {
                        pllsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastModified<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftlastmodified: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::GetLastModified(this) {
                    Ok(ok__) => {
                        pftlastmodified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilename: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::GetFileName(this, core::mem::transmute_copy(&wszfilename), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&psd), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn GetRedirectedURL<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszredirectedurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor_Impl::GetRedirectedURL(this, core::mem::transmute_copy(&wszredirectedurl), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn GetSecurityProvider<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::GetSecurityProvider(this) {
                    Ok(ok__) => {
                        pspclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BindToStream<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::BindToStream(this) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BindToFilter<Identity: IUrlAccessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor_Impl::BindToFilter(this) {
                    Ok(ok__) => {
                        ppfilter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRequestParameter: AddRequestParameter::<Identity, OFFSET>,
            GetDocFormat: GetDocFormat::<Identity, OFFSET>,
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            IsDirectory: IsDirectory::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetLastModified: GetLastModified::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            GetRedirectedURL: GetRedirectedURL::<Identity, OFFSET>,
            GetSecurityProvider: GetSecurityProvider::<Identity, OFFSET>,
            BindToStream: BindToStream::<Identity, OFFSET>,
            BindToFilter: BindToFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IUrlAccessor {}
windows_core::imp::define_interface!(IUrlAccessor2, IUrlAccessor2_Vtbl, 0xc7310734_ac80_11d1_8df3_00c04fb6ef4f);
impl core::ops::Deref for IUrlAccessor2 {
    type Target = IUrlAccessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor2, windows_core::IUnknown, IUrlAccessor);
impl IUrlAccessor2 {
    pub unsafe fn GetDisplayUrl(&self, wszdocurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayUrl)(windows_core::Interface::as_raw(self), wszdocurl as _, dwsize, pdwlength as _) }
    }
    pub unsafe fn IsDocument(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDocument)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCodePage(&self, wszcodepage: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodePage)(windows_core::Interface::as_raw(self), wszcodepage as _, dwsize, pdwlength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlAccessor2_Vtbl {
    pub base__: IUrlAccessor_Vtbl,
    pub GetDisplayUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub IsDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCodePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IUrlAccessor2_Impl: IUrlAccessor_Impl {
    fn GetDisplayUrl(&self, wszdocurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn IsDocument(&self) -> windows_core::Result<()>;
    fn GetCodePage(&self, wszcodepage: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IUrlAccessor2_Vtbl {
    pub const fn new<Identity: IUrlAccessor2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayUrl<Identity: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdocurl: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor2_Impl::GetDisplayUrl(this, core::mem::transmute_copy(&wszdocurl), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        unsafe extern "system" fn IsDocument<Identity: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor2_Impl::IsDocument(this).into()
            }
        }
        unsafe extern "system" fn GetCodePage<Identity: IUrlAccessor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszcodepage: *mut u16, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor2_Impl::GetCodePage(this, core::mem::transmute_copy(&wszcodepage), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
            }
        }
        Self {
            base__: IUrlAccessor_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayUrl: GetDisplayUrl::<Identity, OFFSET>,
            IsDocument: IsDocument::<Identity, OFFSET>,
            GetCodePage: GetCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor2 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IUrlAccessor2 {}
windows_core::imp::define_interface!(IUrlAccessor3, IUrlAccessor3_Vtbl, 0x6fbc7005_0455_4874_b8ff_7439450241a3);
impl core::ops::Deref for IUrlAccessor3 {
    type Target = IUrlAccessor2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor3, windows_core::IUnknown, IUrlAccessor, IUrlAccessor2);
impl IUrlAccessor3 {
    #[cfg(feature = "Win32_wtypesbase")]
    pub unsafe fn GetImpersonationSidBlobs<P0>(&self, pcwszurl: P0, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::wtypesbase::BLOB) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetImpersonationSidBlobs)(windows_core::Interface::as_raw(self), pcwszurl.param().abi(), pcsidcount as _, ppsidblobs as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlAccessor3_Vtbl {
    pub base__: IUrlAccessor2_Vtbl,
    #[cfg(feature = "Win32_wtypesbase")]
    pub GetImpersonationSidBlobs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut super::wtypesbase::BLOB) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypesbase"))]
    GetImpersonationSidBlobs: usize,
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IUrlAccessor3_Impl: IUrlAccessor2_Impl {
    fn GetImpersonationSidBlobs(&self, pcwszurl: &windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::wtypesbase::BLOB) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IUrlAccessor3_Vtbl {
    pub const fn new<Identity: IUrlAccessor3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImpersonationSidBlobs<Identity: IUrlAccessor3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::wtypesbase::BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlAccessor3_Impl::GetImpersonationSidBlobs(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pcsidcount), core::mem::transmute_copy(&ppsidblobs)).into()
            }
        }
        Self { base__: IUrlAccessor2_Vtbl::new::<Identity, OFFSET>(), GetImpersonationSidBlobs: GetImpersonationSidBlobs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor3 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID || iid == &<IUrlAccessor2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IUrlAccessor3 {}
windows_core::imp::define_interface!(IUrlAccessor4, IUrlAccessor4_Vtbl, 0x5cc51041_c8d2_41d7_bca3_9e9e286297dc);
impl core::ops::Deref for IUrlAccessor4 {
    type Target = IUrlAccessor3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlAccessor4, windows_core::IUnknown, IUrlAccessor, IUrlAccessor2, IUrlAccessor3);
impl IUrlAccessor4 {
    pub unsafe fn ShouldIndexItemContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldIndexItemContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn ShouldIndexProperty(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldIndexProperty)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlAccessor4_Vtbl {
    pub base__: IUrlAccessor3_Vtbl,
    pub ShouldIndexItemContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub ShouldIndexProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    ShouldIndexProperty: usize,
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IUrlAccessor4_Impl: IUrlAccessor3_Impl {
    fn ShouldIndexItemContent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ShouldIndexProperty(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IUrlAccessor4_Vtbl {
    pub const fn new<Identity: IUrlAccessor4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShouldIndexItemContent<Identity: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfindexcontent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor4_Impl::ShouldIndexItemContent(this) {
                    Ok(ok__) => {
                        pfindexcontent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShouldIndexProperty<Identity: IUrlAccessor4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pfindexproperty: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlAccessor4_Impl::ShouldIndexProperty(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pfindexproperty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUrlAccessor3_Vtbl::new::<Identity, OFFSET>(),
            ShouldIndexItemContent: ShouldIndexItemContent::<Identity, OFFSET>,
            ShouldIndexProperty: ShouldIndexProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor4 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID || iid == &<IUrlAccessor2 as windows_core::Interface>::IID || iid == &<IUrlAccessor3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_filter", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IUrlAccessor4 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PRIORITIZE_FLAGS(pub i32);
pub const PRIORITIZE_FLAG_IGNOREFAILURECOUNT: tagPRIORITIZE_FLAGS = 2;
pub const PRIORITIZE_FLAG_RETRYFAILEDITEMS: tagPRIORITIZE_FLAGS = 1;
pub type PRIORITY_LEVEL = i32;
pub const PRIORITY_LEVEL_DEFAULT: PRIORITY_LEVEL = 3;
pub const PRIORITY_LEVEL_FOREGROUND: PRIORITY_LEVEL = 0;
pub const PRIORITY_LEVEL_HIGH: PRIORITY_LEVEL = 1;
pub const PRIORITY_LEVEL_LOW: PRIORITY_LEVEL = 2;
pub type PROXY_ACCESS = i32;
pub const PROXY_ACCESS_DIRECT: PROXY_ACCESS = 1;
pub const PROXY_ACCESS_PRECONFIG: PROXY_ACCESS = 0;
pub const PROXY_ACCESS_PROXY: PROXY_ACCESS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROXY_INFO {
    pub dwSize: u32,
    pub pcwszUserAgent: windows_core::PCWSTR,
    pub paUseProxy: PROXY_ACCESS,
    pub fLocalBypass: windows_core::BOOL,
    pub dwPortNumber: u32,
    pub pcwszProxyName: windows_core::PCWSTR,
    pub pcwszBypassList: windows_core::PCWSTR,
}
pub type ROWSETEVENT_ITEMSTATE = i32;
pub const ROWSETEVENT_ITEMSTATE_INROWSET: ROWSETEVENT_ITEMSTATE = 1;
pub const ROWSETEVENT_ITEMSTATE_NOTINROWSET: ROWSETEVENT_ITEMSTATE = 0;
pub const ROWSETEVENT_ITEMSTATE_UNKNOWN: ROWSETEVENT_ITEMSTATE = 2;
pub type ROWSETEVENT_TYPE = i32;
pub const ROWSETEVENT_TYPE_DATAEXPIRED: ROWSETEVENT_TYPE = 0;
pub const ROWSETEVENT_TYPE_FOREGROUNDLOST: ROWSETEVENT_TYPE = 1;
pub const ROWSETEVENT_TYPE_SCOPESTATISTICS: ROWSETEVENT_TYPE = 2;
pub const SEARCH_ADVANCED_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 1;
pub const SEARCH_CHANGE_ADD: SEARCH_KIND_OF_CHANGE = 0;
pub const SEARCH_CHANGE_DELETE: SEARCH_KIND_OF_CHANGE = 1;
pub const SEARCH_CHANGE_MODIFY: SEARCH_KIND_OF_CHANGE = 2;
pub const SEARCH_CHANGE_MOVE_RENAME: SEARCH_KIND_OF_CHANGE = 3;
pub const SEARCH_CHANGE_SEMANTICS_DIRECTORY: SEARCH_KIND_OF_CHANGE = 262144;
pub const SEARCH_CHANGE_SEMANTICS_SHALLOW: SEARCH_KIND_OF_CHANGE = 524288;
pub const SEARCH_CHANGE_SEMANTICS_UPDATE_SECURITY: SEARCH_KIND_OF_CHANGE = 4194304;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub struct SEARCH_COLUMN_PROPERTIES {
    pub Value: super::propidlbase::PROPVARIANT,
    pub lcid: super::winnt::LCID,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Clone for SEARCH_COLUMN_PROPERTIES {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for SEARCH_COLUMN_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEARCH_HIGH_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 1;
pub type SEARCH_INDEXING_PHASE = i32;
pub const SEARCH_INDEXING_PHASE_GATHERER: SEARCH_INDEXING_PHASE = 0;
pub const SEARCH_INDEXING_PHASE_PERSISTED: SEARCH_INDEXING_PHASE = 2;
pub const SEARCH_INDEXING_PHASE_QUERYABLE: SEARCH_INDEXING_PHASE = 1;
#[repr(C)]
#[cfg(feature = "Win32_wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEARCH_ITEM_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
    pub pUserData: *mut super::wtypesbase::BLOB,
    pub lpwszURL: windows_core::PWSTR,
    pub lpwszOldURL: windows_core::PWSTR,
}
#[cfg(feature = "Win32_wtypesbase")]
impl Default for SEARCH_ITEM_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEARCH_ITEM_INDEXING_STATUS {
    pub dwDocID: u32,
    pub hrIndexingStatus: windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SEARCH_ITEM_PERSISTENT_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub URL: windows_core::PWSTR,
    pub OldURL: windows_core::PWSTR,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
}
pub type SEARCH_KIND_OF_CHANGE = i32;
pub const SEARCH_NATURAL_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 2;
pub const SEARCH_NORMAL_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 0;
pub type SEARCH_NOTIFICATION_PRIORITY = i32;
pub const SEARCH_NO_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 0;
pub type SEARCH_QUERY_SYNTAX = i32;
pub type SEARCH_TERM_EXPANSION = i32;
pub const SEARCH_TERM_NO_EXPANSION: SEARCH_TERM_EXPANSION = 0;
pub const SEARCH_TERM_PREFIX_ALL: SEARCH_TERM_EXPANSION = 1;
pub const SEARCH_TERM_STEM_ALL: SEARCH_TERM_EXPANSION = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TIMEOUT_INFO {
    pub dwSize: u32,
    pub dwConnectTimeout: u32,
    pub dwDataTimeout: u32,
}
pub const eAUTH_TYPE_ANONYMOUS: AUTH_TYPE = 0;
pub const eAUTH_TYPE_BASIC: AUTH_TYPE = 2;
pub const eAUTH_TYPE_NTLM: AUTH_TYPE = 1;
pub type tagPRIORITIZE_FLAGS = i32;
