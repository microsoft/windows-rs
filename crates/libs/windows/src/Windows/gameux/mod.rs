pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = 0;
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = 1;
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = 1;
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = 0;
pub type GAMESTATS_OPEN_RESULT = i32;
pub type GAMESTATS_OPEN_TYPE = i32;
pub type GAME_INSTALL_SCOPE = i32;
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = 3;
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = 2;
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = 1;
pub const GameExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: windows_core::GUID = windows_core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
pub const ID_GDF_THUMBNAIL_STR: windows_core::PCWSTR = windows_core::w!("__GDF_THUMBNAIL");
pub const ID_GDF_XML_STR: windows_core::PCWSTR = windows_core::w!("__GDF_XML");
windows_core::imp::define_interface!(IGameExplorer, IGameExplorer_Vtbl, 0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
windows_core::imp::interface_hierarchy!(IGameExplorer, windows_core::IUnknown);
impl IGameExplorer {
    pub unsafe fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddGame)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgdfbinarypath), core::mem::transmute_copy(bstrgameinstalldirectory), installscope, pguidinstanceid as _) }
    }
    pub unsafe fn RemoveGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveGame)(windows_core::Interface::as_raw(self), guidinstanceid) }
    }
    pub unsafe fn UpdateGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateGame)(windows_core::Interface::as_raw(self), guidinstanceid) }
    }
    pub unsafe fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerifyAccess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgdfbinarypath), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, GAME_INSTALL_SCOPE, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RemoveGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub VerifyAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IGameExplorer_Impl: windows_core::IUnknownImpl {
    fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: IGameExplorer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: *mut core::ffi::c_void, bstrgameinstalldirectory: *mut core::ffi::c_void, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::AddGame(this, core::mem::transmute(&bstrgdfbinarypath), core::mem::transmute(&bstrgameinstalldirectory), core::mem::transmute_copy(&installscope), core::mem::transmute_copy(&pguidinstanceid)).into()
            }
        }
        unsafe extern "system" fn RemoveGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::RemoveGame(this, core::mem::transmute(&guidinstanceid)).into()
            }
        }
        unsafe extern "system" fn UpdateGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::UpdateGame(this, core::mem::transmute(&guidinstanceid)).into()
            }
        }
        unsafe extern "system" fn VerifyAccess<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: *mut core::ffi::c_void, pfhasaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameExplorer_Impl::VerifyAccess(this, core::mem::transmute(&bstrgdfbinarypath)) {
                    Ok(ok__) => {
                        pfhasaccess.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, OFFSET>,
            RemoveGame: RemoveGame::<Identity, OFFSET>,
            UpdateGame: UpdateGame::<Identity, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameExplorer {}
windows_core::imp::define_interface!(IGameExplorer2, IGameExplorer2_Vtbl, 0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
windows_core::imp::interface_hierarchy!(IGameExplorer2, windows_core::IUnknown);
impl IGameExplorer2 {
    pub unsafe fn InstallGame<P0, P1>(&self, binarygdfpath: P0, installdirectory: P1, installscope: GAME_INSTALL_SCOPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), installdirectory.param().abi(), installscope) }
    }
    pub unsafe fn UninstallGame<P0>(&self, binarygdfpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UninstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi()) }
    }
    pub unsafe fn CheckAccess<P0>(&self, binarygdfpath: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckAccess)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, GAME_INSTALL_SCOPE) -> windows_core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IGameExplorer2_Impl: windows_core::IUnknownImpl {
    fn InstallGame(&self, binarygdfpath: &windows_core::PCWSTR, installdirectory: &windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::Result<()>;
    fn UninstallGame(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CheckAccess(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: IGameExplorer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallGame<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, installdirectory: windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer2_Impl::InstallGame(this, core::mem::transmute(&binarygdfpath), core::mem::transmute(&installdirectory), core::mem::transmute_copy(&installscope)).into()
            }
        }
        unsafe extern "system" fn UninstallGame<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer2_Impl::UninstallGame(this, core::mem::transmute(&binarygdfpath)).into()
            }
        }
        unsafe extern "system" fn CheckAccess<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, phasaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameExplorer2_Impl::CheckAccess(this, core::mem::transmute(&binarygdfpath)) {
                    Ok(ok__) => {
                        phasaccess.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, OFFSET>,
            UninstallGame: UninstallGame::<Identity, OFFSET>,
            CheckAccess: CheckAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameExplorer2 {}
windows_core::imp::define_interface!(IGameStatistics, IGameStatistics_Vtbl, 0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
windows_core::imp::interface_hierarchy!(IGameStatistics, windows_core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCategoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxNameLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxNameLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxValueLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxValueLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxCategories(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxStatsPerCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCategoryTitle<P1>(&self, categoryindex: u16, title: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, title.param().abi()) }
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, pname as _, pvalue as _) }
    }
    pub unsafe fn SetStatistic<P2, P3>(&self, categoryindex: u16, statindex: u16, name: P2, value: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, name.param().abi(), value.param().abi()) }
    }
    pub unsafe fn Save(&self, trackchanges: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), trackchanges.into()) }
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastPlayedCategory)(windows_core::Interface::as_raw(self), categoryindex) }
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPlayedCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxCategoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IGameStatistics_Impl: windows_core::IUnknownImpl {
    fn GetMaxCategoryLength(&self) -> windows_core::Result<u32>;
    fn GetMaxNameLength(&self) -> windows_core::Result<u32>;
    fn GetMaxValueLength(&self) -> windows_core::Result<u32>;
    fn GetMaxCategories(&self) -> windows_core::Result<u16>;
    fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16>;
    fn SetCategoryTitle(&self, categoryindex: u16, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR>;
    fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetStatistic(&self, categoryindex: u16, statindex: u16, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Save(&self, trackchanges: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::Result<()>;
    fn GetLastPlayedCategory(&self) -> windows_core::Result<u32>;
}
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: IGameStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxCategoryLength(this) {
                    Ok(ok__) => {
                        cch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxNameLength(this) {
                    Ok(ok__) => {
                        cch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxValueLength(this) {
                    Ok(ok__) => {
                        cch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxCategories(this) {
                    Ok(ok__) => {
                        pmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxStatsPerCategory(this) {
                    Ok(ok__) => {
                        pmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetCategoryTitle(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, ptitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetCategoryTitle(this, core::mem::transmute_copy(&categoryindex)) {
                    Ok(ok__) => {
                        ptitle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::GetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetStatistic<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackchanges: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::Save(this, core::mem::transmute_copy(&trackchanges)).into()
            }
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetLastPlayedCategory(this, core::mem::transmute_copy(&categoryindex)).into()
            }
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetLastPlayedCategory(this) {
                    Ok(ok__) => {
                        pcategoryindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, OFFSET>,
            GetStatistic: GetStatistic::<Identity, OFFSET>,
            SetStatistic: SetStatistic::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameStatistics {}
windows_core::imp::define_interface!(IGameStatisticsMgr, IGameStatisticsMgr_Vtbl, 0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
windows_core::imp::interface_hierarchy!(IGameStatisticsMgr, windows_core::IUnknown);
impl IGameStatisticsMgr {
    pub unsafe fn GetGameStatistics<P0>(&self, gdfbinarypath: P0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT) -> windows_core::Result<IGameStatistics>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi(), opentype, popenresult as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveGameStatistics<P0>(&self, gdfbinarypath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, GAMESTATS_OPEN_TYPE, *mut GAMESTATS_OPEN_RESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IGameStatisticsMgr_Impl: windows_core::IUnknownImpl {
    fn GetGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT) -> windows_core::Result<IGameStatistics>;
    fn RemoveGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGameStatistics<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatisticsMgr_Impl::GetGameStatistics(this, core::mem::transmute(&gdfbinarypath), core::mem::transmute_copy(&opentype), core::mem::transmute_copy(&popenresult)) {
                    Ok(ok__) => {
                        ppistats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatisticsMgr_Impl::RemoveGameStatistics(this, core::mem::transmute(&gdfbinarypath)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameStatisticsMgr {}
