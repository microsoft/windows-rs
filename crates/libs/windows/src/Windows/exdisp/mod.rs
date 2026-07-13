pub type BrowserNavConstants = i32;
pub const CSC_NAVIGATEBACK: CommandStateChangeConstants = 2;
pub const CSC_NAVIGATEFORWARD: CommandStateChangeConstants = 1;
pub const CSC_UPDATECOMMANDS: CommandStateChangeConstants = -1;
pub const CScriptErrorList: windows_core::GUID = windows_core::GUID::from_u128(0xefd01300_160f_11d2_bb2e_00805ff7efca);
pub type CommandStateChangeConstants = i32;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DShellNameSpaceEvents, DShellNameSpaceEvents_Vtbl, 0x55136806_b2de_11d1_b9f2_00a0c98bc547);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DShellNameSpaceEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DShellNameSpaceEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DShellNameSpaceEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DShellNameSpaceEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DShellNameSpaceEvents_Vtbl {
    pub const fn new<Identity: DShellNameSpaceEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DShellNameSpaceEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DShellNameSpaceEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DShellWindowsEvents, DShellWindowsEvents_Vtbl, 0xfe4106e0_399a_11d0_a48c_00a0c90a8f39);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DShellWindowsEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DShellWindowsEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DShellWindowsEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DShellWindowsEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DShellWindowsEvents_Vtbl {
    pub const fn new<Identity: DShellWindowsEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DShellWindowsEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DShellWindowsEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DWebBrowserEvents, DWebBrowserEvents_Vtbl, 0xeab22ac2_30c1_11cf_a7eb_0000c05bae0b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DWebBrowserEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DWebBrowserEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DWebBrowserEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DWebBrowserEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DWebBrowserEvents_Vtbl {
    pub const fn new<Identity: DWebBrowserEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWebBrowserEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DWebBrowserEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DWebBrowserEvents2, DWebBrowserEvents2_Vtbl, 0x34a715a0_6587_11d0_924a_0020afc7ac4d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DWebBrowserEvents2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DWebBrowserEvents2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DWebBrowserEvents2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DWebBrowserEvents2_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DWebBrowserEvents2_Vtbl {
    pub const fn new<Identity: DWebBrowserEvents2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWebBrowserEvents2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DWebBrowserEvents2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IScriptErrorList, IScriptErrorList_Vtbl, 0xf3470f24_15fd_11d2_bb2e_00805ff7efca);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IScriptErrorList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IScriptErrorList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IScriptErrorList {
    pub unsafe fn advanceError(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).advanceError)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn retreatError(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).retreatError)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn canAdvanceError(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).canAdvanceError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn canRetreatError(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).canRetreatError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getErrorLine(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getErrorChar(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorChar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getErrorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getErrorMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getErrorUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getAlwaysShowLockState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAlwaysShowLockState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getDetailsPaneOpen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getDetailsPaneOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn setDetailsPaneOpen(&self, fdetailspaneopen: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setDetailsPaneOpen)(windows_core::Interface::as_raw(self), fdetailspaneopen.into()) }
    }
    pub unsafe fn getPerErrorDisplay(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getPerErrorDisplay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn setPerErrorDisplay(&self, fpererrordisplay: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setPerErrorDisplay)(windows_core::Interface::as_raw(self), fpererrordisplay.into()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IScriptErrorList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub advanceError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub retreatError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub canAdvanceError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub canRetreatError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub getErrorLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getErrorChar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getErrorMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getErrorUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAlwaysShowLockState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub getDetailsPaneOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub setDetailsPaneOpen: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub getPerErrorDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub setPerErrorDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IScriptErrorList_Impl: super::oaidl::IDispatch_Impl {
    fn advanceError(&self) -> windows_core::Result<()>;
    fn retreatError(&self) -> windows_core::Result<()>;
    fn canAdvanceError(&self) -> windows_core::Result<windows_core::BOOL>;
    fn canRetreatError(&self) -> windows_core::Result<windows_core::BOOL>;
    fn getErrorLine(&self) -> windows_core::Result<i32>;
    fn getErrorChar(&self) -> windows_core::Result<i32>;
    fn getErrorCode(&self) -> windows_core::Result<i32>;
    fn getErrorMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn getErrorUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn getAlwaysShowLockState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn getDetailsPaneOpen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn setDetailsPaneOpen(&self, fdetailspaneopen: windows_core::BOOL) -> windows_core::Result<()>;
    fn getPerErrorDisplay(&self) -> windows_core::Result<windows_core::BOOL>;
    fn setPerErrorDisplay(&self, fpererrordisplay: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IScriptErrorList_Vtbl {
    pub const fn new<Identity: IScriptErrorList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn advanceError<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptErrorList_Impl::advanceError(this).into()
            }
        }
        unsafe extern "system" fn retreatError<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptErrorList_Impl::retreatError(this).into()
            }
        }
        unsafe extern "system" fn canAdvanceError<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcanadvance: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::canAdvanceError(this) {
                    Ok(ok__) => {
                        pfcanadvance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn canRetreatError<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcanretreat: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::canRetreatError(this) {
                    Ok(ok__) => {
                        pfcanretreat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getErrorLine<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plline: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getErrorLine(this) {
                    Ok(ok__) => {
                        plline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getErrorChar<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plchar: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getErrorChar(this) {
                    Ok(ok__) => {
                        plchar.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getErrorCode<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getErrorCode(this) {
                    Ok(ok__) => {
                        plcode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getErrorMsg<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getErrorMsg(this) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getErrorUrl<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getErrorUrl(this) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAlwaysShowLockState<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfalwaysshowlocked: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getAlwaysShowLockState(this) {
                    Ok(ok__) => {
                        pfalwaysshowlocked.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getDetailsPaneOpen<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdetailspaneopen: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getDetailsPaneOpen(this) {
                    Ok(ok__) => {
                        pfdetailspaneopen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setDetailsPaneOpen<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdetailspaneopen: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptErrorList_Impl::setDetailsPaneOpen(this, core::mem::transmute_copy(&fdetailspaneopen)).into()
            }
        }
        unsafe extern "system" fn getPerErrorDisplay<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpererrordisplay: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptErrorList_Impl::getPerErrorDisplay(this) {
                    Ok(ok__) => {
                        pfpererrordisplay.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setPerErrorDisplay<Identity: IScriptErrorList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpererrordisplay: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptErrorList_Impl::setPerErrorDisplay(this, core::mem::transmute_copy(&fpererrordisplay)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            advanceError: advanceError::<Identity, OFFSET>,
            retreatError: retreatError::<Identity, OFFSET>,
            canAdvanceError: canAdvanceError::<Identity, OFFSET>,
            canRetreatError: canRetreatError::<Identity, OFFSET>,
            getErrorLine: getErrorLine::<Identity, OFFSET>,
            getErrorChar: getErrorChar::<Identity, OFFSET>,
            getErrorCode: getErrorCode::<Identity, OFFSET>,
            getErrorMsg: getErrorMsg::<Identity, OFFSET>,
            getErrorUrl: getErrorUrl::<Identity, OFFSET>,
            getAlwaysShowLockState: getAlwaysShowLockState::<Identity, OFFSET>,
            getDetailsPaneOpen: getDetailsPaneOpen::<Identity, OFFSET>,
            setDetailsPaneOpen: setDetailsPaneOpen::<Identity, OFFSET>,
            getPerErrorDisplay: getPerErrorDisplay::<Identity, OFFSET>,
            setPerErrorDisplay: setPerErrorDisplay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScriptErrorList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IScriptErrorList {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellFavoritesNameSpace, IShellFavoritesNameSpace_Vtbl, 0x55136804_b2de_11d1_b9f2_00a0c98bc547);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellFavoritesNameSpace {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellFavoritesNameSpace, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellFavoritesNameSpace {
    pub unsafe fn MoveSelectionUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveSelectionUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn MoveSelectionDown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveSelectionDown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ResetSort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetSort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn NewFolder(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NewFolder)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Synchronize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Synchronize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Import(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Import)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Export(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Export)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InvokeContextMenuCommand(&self, strcommand: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeContextMenuCommand)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strcommand)) }
    }
    pub unsafe fn MoveSelectionTo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveSelectionTo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SubscriptionsEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubscriptionsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CreateSubscriptionForSelection(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSubscriptionForSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeleteSubscriptionForSelection(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeleteSubscriptionForSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRoot(&self, bstrfullpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfullpath)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellFavoritesNameSpace_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub MoveSelectionUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveSelectionDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetSort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NewFolder: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Synchronize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Import: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Export: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeContextMenuCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveSelectionTo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SubscriptionsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SubscriptionsEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub CreateSubscriptionForSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CreateSubscriptionForSelection: usize,
    #[cfg(feature = "wtypes")]
    pub DeleteSubscriptionForSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeleteSubscriptionForSelection: usize,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellFavoritesNameSpace_Impl: super::oaidl::IDispatch_Impl {
    fn MoveSelectionUp(&self) -> windows_core::Result<()>;
    fn MoveSelectionDown(&self) -> windows_core::Result<()>;
    fn ResetSort(&self) -> windows_core::Result<()>;
    fn NewFolder(&self) -> windows_core::Result<()>;
    fn Synchronize(&self) -> windows_core::Result<()>;
    fn Import(&self) -> windows_core::Result<()>;
    fn Export(&self) -> windows_core::Result<()>;
    fn InvokeContextMenuCommand(&self, strcommand: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MoveSelectionTo(&self) -> windows_core::Result<()>;
    fn SubscriptionsEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CreateSubscriptionForSelection(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn DeleteSubscriptionForSelection(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetRoot(&self, bstrfullpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellFavoritesNameSpace_Vtbl {
    pub const fn new<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveSelectionUp<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::MoveSelectionUp(this).into()
            }
        }
        unsafe extern "system" fn MoveSelectionDown<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::MoveSelectionDown(this).into()
            }
        }
        unsafe extern "system" fn ResetSort<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::ResetSort(this).into()
            }
        }
        unsafe extern "system" fn NewFolder<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::NewFolder(this).into()
            }
        }
        unsafe extern "system" fn Synchronize<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::Synchronize(this).into()
            }
        }
        unsafe extern "system" fn Import<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::Import(this).into()
            }
        }
        unsafe extern "system" fn Export<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::Export(this).into()
            }
        }
        unsafe extern "system" fn InvokeContextMenuCommand<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcommand: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::InvokeContextMenuCommand(this, core::mem::transmute(&strcommand)).into()
            }
        }
        unsafe extern "system" fn MoveSelectionTo<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::MoveSelectionTo(this).into()
            }
        }
        unsafe extern "system" fn SubscriptionsEnabled<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFavoritesNameSpace_Impl::SubscriptionsEnabled(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSubscriptionForSelection<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFavoritesNameSpace_Impl::CreateSubscriptionForSelection(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSubscriptionForSelection<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFavoritesNameSpace_Impl::DeleteSubscriptionForSelection(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRoot<Identity: IShellFavoritesNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfullpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFavoritesNameSpace_Impl::SetRoot(this, core::mem::transmute(&bstrfullpath)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MoveSelectionUp: MoveSelectionUp::<Identity, OFFSET>,
            MoveSelectionDown: MoveSelectionDown::<Identity, OFFSET>,
            ResetSort: ResetSort::<Identity, OFFSET>,
            NewFolder: NewFolder::<Identity, OFFSET>,
            Synchronize: Synchronize::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            Export: Export::<Identity, OFFSET>,
            InvokeContextMenuCommand: InvokeContextMenuCommand::<Identity, OFFSET>,
            MoveSelectionTo: MoveSelectionTo::<Identity, OFFSET>,
            SubscriptionsEnabled: SubscriptionsEnabled::<Identity, OFFSET>,
            CreateSubscriptionForSelection: CreateSubscriptionForSelection::<Identity, OFFSET>,
            DeleteSubscriptionForSelection: DeleteSubscriptionForSelection::<Identity, OFFSET>,
            SetRoot: SetRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFavoritesNameSpace as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellFavoritesNameSpace {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellNameSpace, IShellNameSpace_Vtbl, 0xe572d3c9_37be_4ae2_825d_d521763e3108);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellNameSpace {
    type Target = IShellFavoritesNameSpace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellNameSpace, windows_core::IUnknown, super::oaidl::IDispatch, IShellFavoritesNameSpace);
#[cfg(feature = "oaidl")]
impl IShellNameSpace {
    pub unsafe fn EnumOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnumOptions(&self, lval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnumOptions)(windows_core::Interface::as_raw(self), lval) }
    }
    pub unsafe fn SelectedItem(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSelectedItem<P0>(&self, pitem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedItem)(windows_core::Interface::as_raw(self), pitem.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Root(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Root)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetRoot(&self, var: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var)) }
    }
    pub unsafe fn Depth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Depth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDepth(&self, idepth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDepth)(windows_core::Interface::as_raw(self), idepth) }
    }
    pub unsafe fn Mode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMode(&self, umode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), umode) }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn SetTVFlags(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTVFlags)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn TVFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TVFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Columns(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Columns)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetColumns(&self, bstrcolumns: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColumns)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcolumns)) }
    }
    pub unsafe fn CountViewTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountViewTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetViewType(&self, itype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewType)(windows_core::Interface::as_raw(self), itype) }
    }
    pub unsafe fn SelectedItems(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Expand(&self, var: &super::oaidl::VARIANT, idepth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var), idepth) }
    }
    pub unsafe fn UnselectAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnselectAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellNameSpace_Vtbl {
    pub base__: IShellFavoritesNameSpace_Vtbl,
    pub EnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Root: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetRoot: usize,
    pub Depth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetTVFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TVFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Columns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CountViewTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetViewType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SelectedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Expand: usize,
    pub UnselectAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellNameSpace_Impl: IShellFavoritesNameSpace_Impl {
    fn EnumOptions(&self) -> windows_core::Result<i32>;
    fn SetEnumOptions(&self, lval: i32) -> windows_core::Result<()>;
    fn SelectedItem(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn SetSelectedItem(&self, pitem: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn Root(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetRoot(&self, var: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Depth(&self) -> windows_core::Result<i32>;
    fn SetDepth(&self, idepth: i32) -> windows_core::Result<()>;
    fn Mode(&self) -> windows_core::Result<u32>;
    fn SetMode(&self, umode: u32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<u32>;
    fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn SetTVFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn TVFlags(&self) -> windows_core::Result<u32>;
    fn Columns(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetColumns(&self, bstrcolumns: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CountViewTypes(&self) -> windows_core::Result<i32>;
    fn SetViewType(&self, itype: i32) -> windows_core::Result<()>;
    fn SelectedItems(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Expand(&self, var: &super::oaidl::VARIANT, idepth: i32) -> windows_core::Result<()>;
    fn UnselectAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellNameSpace_Vtbl {
    pub const fn new<Identity: IShellNameSpace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOptions<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfenumflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::EnumOptions(this) {
                    Ok(ok__) => {
                        pgrfenumflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnumOptions<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetEnumOptions(this, core::mem::transmute_copy(&lval)).into()
            }
        }
        unsafe extern "system" fn SelectedItem<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::SelectedItem(this) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedItem<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetSelectedItem(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        unsafe extern "system" fn Root<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::Root(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRoot<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetRoot(this, core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn Depth<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidepth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::Depth(this) {
                    Ok(ok__) => {
                        pidepth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepth<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idepth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetDepth(this, core::mem::transmute_copy(&idepth)).into()
            }
        }
        unsafe extern "system" fn Mode<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pumode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::Mode(this) {
                    Ok(ok__) => {
                        pumode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMode<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetMode(this, core::mem::transmute_copy(&umode)).into()
            }
        }
        unsafe extern "system" fn Flags<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::Flags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetTVFlags<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetTVFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn TVFlags<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::TVFlags(this) {
                    Ok(ok__) => {
                        dwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Columns<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcolumns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::Columns(this) {
                    Ok(ok__) => {
                        bstrcolumns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColumns<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcolumns: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetColumns(this, core::mem::transmute(&bstrcolumns)).into()
            }
        }
        unsafe extern "system" fn CountViewTypes<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitypes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::CountViewTypes(this) {
                    Ok(ok__) => {
                        pitypes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetViewType<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::SetViewType(this, core::mem::transmute_copy(&itype)).into()
            }
        }
        unsafe extern "system" fn SelectedItems<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellNameSpace_Impl::SelectedItems(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Expand<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::oaidl::VARIANT, idepth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::Expand(this, core::mem::transmute(&var), core::mem::transmute_copy(&idepth)).into()
            }
        }
        unsafe extern "system" fn UnselectAll<Identity: IShellNameSpace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellNameSpace_Impl::UnselectAll(this).into()
            }
        }
        Self {
            base__: IShellFavoritesNameSpace_Vtbl::new::<Identity, OFFSET>(),
            EnumOptions: EnumOptions::<Identity, OFFSET>,
            SetEnumOptions: SetEnumOptions::<Identity, OFFSET>,
            SelectedItem: SelectedItem::<Identity, OFFSET>,
            SetSelectedItem: SetSelectedItem::<Identity, OFFSET>,
            Root: Root::<Identity, OFFSET>,
            SetRoot: SetRoot::<Identity, OFFSET>,
            Depth: Depth::<Identity, OFFSET>,
            SetDepth: SetDepth::<Identity, OFFSET>,
            Mode: Mode::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            SetTVFlags: SetTVFlags::<Identity, OFFSET>,
            TVFlags: TVFlags::<Identity, OFFSET>,
            Columns: Columns::<Identity, OFFSET>,
            SetColumns: SetColumns::<Identity, OFFSET>,
            CountViewTypes: CountViewTypes::<Identity, OFFSET>,
            SetViewType: SetViewType::<Identity, OFFSET>,
            SelectedItems: SelectedItems::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
            UnselectAll: UnselectAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellNameSpace as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellFavoritesNameSpace as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellNameSpace {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper, IShellUIHelper_Vtbl, 0x729fe2f8_1ea8_11d1_8f85_00c04fc2fbe1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellUIHelper {
    pub unsafe fn ResetFirstBootMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetFirstBootMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ResetSafeMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetSafeMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshOfflineDesktop)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddFavorite(&self, url: &windows_core::BSTR, title: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFavorite)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), title.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn AddChannel(&self, url: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddChannel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddDesktopComponent(&self, url: &windows_core::BSTR, r#type: &windows_core::BSTR, left: Option<*const super::oaidl::VARIANT>, top: Option<*const super::oaidl::VARIANT>, width: Option<*const super::oaidl::VARIANT>, height: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDesktopComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), core::mem::transmute_copy(r#type), left.unwrap_or(core::mem::zeroed()) as _, top.unwrap_or(core::mem::zeroed()) as _, width.unwrap_or(core::mem::zeroed()) as _, height.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsSubscribed(&self, url: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSubscribed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn NavigateAndFind(&self, url: &windows_core::BSTR, strquery: &windows_core::BSTR, vartargetframe: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NavigateAndFind)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), core::mem::transmute_copy(strquery), core::mem::transmute(vartargetframe)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ImportExportFavorites(&self, fimport: super::wtypes::VARIANT_BOOL, strimpexppath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportExportFavorites)(windows_core::Interface::as_raw(self), fimport, core::mem::transmute_copy(strimpexppath)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AutoCompleteSaveForm)(windows_core::Interface::as_raw(self), form.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AutoScan(&self, strsearch: &windows_core::BSTR, strfailureurl: &windows_core::BSTR, pvartargetframe: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AutoScan)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsearch), core::mem::transmute_copy(strfailureurl), pvartargetframe.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AutoCompleteAttach)(windows_core::Interface::as_raw(self), reserved.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ShowBrowserUI(&self, bstrname: &windows_core::BSTR, pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowBrowserUI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub ResetFirstBootMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetSafeMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshOfflineDesktop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddFavorite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddFavorite: usize,
    pub AddChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddDesktopComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddDesktopComponent: usize,
    #[cfg(feature = "wtypes")]
    pub IsSubscribed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsSubscribed: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub NavigateAndFind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    NavigateAndFind: usize,
    #[cfg(feature = "wtypes")]
    pub ImportExportFavorites: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ImportExportFavorites: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AutoCompleteSaveForm: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AutoCompleteSaveForm: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AutoScan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AutoScan: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AutoCompleteAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AutoCompleteAttach: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ShowBrowserUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ShowBrowserUI: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper_Impl: super::oaidl::IDispatch_Impl {
    fn ResetFirstBootMode(&self) -> windows_core::Result<()>;
    fn ResetSafeMode(&self) -> windows_core::Result<()>;
    fn RefreshOfflineDesktop(&self) -> windows_core::Result<()>;
    fn AddFavorite(&self, url: &windows_core::BSTR, title: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AddChannel(&self, url: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddDesktopComponent(&self, url: &windows_core::BSTR, r#type: &windows_core::BSTR, left: *const super::oaidl::VARIANT, top: *const super::oaidl::VARIANT, width: *const super::oaidl::VARIANT, height: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn IsSubscribed(&self, url: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn NavigateAndFind(&self, url: &windows_core::BSTR, strquery: &windows_core::BSTR, vartargetframe: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ImportExportFavorites(&self, fimport: super::wtypes::VARIANT_BOOL, strimpexppath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AutoCompleteSaveForm(&self, form: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AutoScan(&self, strsearch: &windows_core::BSTR, strfailureurl: &windows_core::BSTR, pvartargetframe: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AutoCompleteAttach(&self, reserved: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ShowBrowserUI(&self, bstrname: &windows_core::BSTR, pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper_Vtbl {
    pub const fn new<Identity: IShellUIHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetFirstBootMode<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::ResetFirstBootMode(this).into()
            }
        }
        unsafe extern "system" fn ResetSafeMode<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::ResetSafeMode(this).into()
            }
        }
        unsafe extern "system" fn RefreshOfflineDesktop<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::RefreshOfflineDesktop(this).into()
            }
        }
        unsafe extern "system" fn AddFavorite<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, title: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AddFavorite(this, core::mem::transmute(&url), core::mem::transmute_copy(&title)).into()
            }
        }
        unsafe extern "system" fn AddChannel<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AddChannel(this, core::mem::transmute(&url)).into()
            }
        }
        unsafe extern "system" fn AddDesktopComponent<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void, left: *const super::oaidl::VARIANT, top: *const super::oaidl::VARIANT, width: *const super::oaidl::VARIANT, height: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AddDesktopComponent(this, core::mem::transmute(&url), core::mem::transmute(&r#type), core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn IsSubscribed<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper_Impl::IsSubscribed(this, core::mem::transmute(&url)) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NavigateAndFind<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, vartargetframe: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::NavigateAndFind(this, core::mem::transmute(&url), core::mem::transmute(&strquery), core::mem::transmute_copy(&vartargetframe)).into()
            }
        }
        unsafe extern "system" fn ImportExportFavorites<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fimport: super::wtypes::VARIANT_BOOL, strimpexppath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::ImportExportFavorites(this, core::mem::transmute_copy(&fimport), core::mem::transmute(&strimpexppath)).into()
            }
        }
        unsafe extern "system" fn AutoCompleteSaveForm<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, form: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AutoCompleteSaveForm(this, core::mem::transmute_copy(&form)).into()
            }
        }
        unsafe extern "system" fn AutoScan<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsearch: *mut core::ffi::c_void, strfailureurl: *mut core::ffi::c_void, pvartargetframe: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AutoScan(this, core::mem::transmute(&strsearch), core::mem::transmute(&strfailureurl), core::mem::transmute_copy(&pvartargetframe)).into()
            }
        }
        unsafe extern "system" fn AutoCompleteAttach<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper_Impl::AutoCompleteAttach(this, core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn ShowBrowserUI<Identity: IShellUIHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pvarin: *const super::oaidl::VARIANT, pvarout: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper_Impl::ShowBrowserUI(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&pvarin)) {
                    Ok(ok__) => {
                        pvarout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ResetFirstBootMode: ResetFirstBootMode::<Identity, OFFSET>,
            ResetSafeMode: ResetSafeMode::<Identity, OFFSET>,
            RefreshOfflineDesktop: RefreshOfflineDesktop::<Identity, OFFSET>,
            AddFavorite: AddFavorite::<Identity, OFFSET>,
            AddChannel: AddChannel::<Identity, OFFSET>,
            AddDesktopComponent: AddDesktopComponent::<Identity, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, OFFSET>,
            NavigateAndFind: NavigateAndFind::<Identity, OFFSET>,
            ImportExportFavorites: ImportExportFavorites::<Identity, OFFSET>,
            AutoCompleteSaveForm: AutoCompleteSaveForm::<Identity, OFFSET>,
            AutoScan: AutoScan::<Identity, OFFSET>,
            AutoCompleteAttach: AutoCompleteAttach::<Identity, OFFSET>,
            ShowBrowserUI: ShowBrowserUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper2, IShellUIHelper2_Vtbl, 0xa7fe6eda_1932_4281_b881_87b31b8bc52c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper2 {
    type Target = IShellUIHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper2, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper);
#[cfg(feature = "oaidl")]
impl IShellUIHelper2 {
    pub unsafe fn AddSearchProvider(&self, url: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSearchProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url)) }
    }
    pub unsafe fn RunOnceShown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RunOnceShown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SkipRunOnce(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SkipRunOnce)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CustomizeSettings(&self, fsqm: super::wtypes::VARIANT_BOOL, fphishing: super::wtypes::VARIANT_BOOL, bstrlocale: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CustomizeSettings)(windows_core::Interface::as_raw(self), fsqm, fphishing, core::mem::transmute_copy(bstrlocale)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SqmEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SqmEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PhishingEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PhishingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BrandImageUri(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BrandImageUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SkipTabsWelcome(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SkipTabsWelcome)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DiagnoseConnection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiagnoseConnection)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CustomizeClearType(&self, fset: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CustomizeClearType)(windows_core::Interface::as_raw(self), fset) }
    }
    pub unsafe fn IsSearchProviderInstalled(&self, url: &windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSearchProviderInstalled)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsSearchMigrated(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSearchMigrated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultSearchProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultSearchProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RunOnceRequiredSettingsComplete)(windows_core::Interface::as_raw(self), fcomplete) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RunOnceHasShown(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunOnceHasShown)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SearchGuideUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchGuideUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper2_Vtbl {
    pub base__: IShellUIHelper_Vtbl,
    pub AddSearchProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunOnceShown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SkipRunOnce: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CustomizeSettings: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, super::wtypes::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CustomizeSettings: usize,
    #[cfg(feature = "wtypes")]
    pub SqmEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SqmEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub PhishingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PhishingEnabled: usize,
    pub BrandImageUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SkipTabsWelcome: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiagnoseConnection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CustomizeClearType: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CustomizeClearType: usize,
    pub IsSearchProviderInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsSearchMigrated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsSearchMigrated: usize,
    pub DefaultSearchProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RunOnceRequiredSettingsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RunOnceRequiredSettingsComplete: usize,
    #[cfg(feature = "wtypes")]
    pub RunOnceHasShown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RunOnceHasShown: usize,
    pub SearchGuideUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper2_Impl: IShellUIHelper_Impl {
    fn AddSearchProvider(&self, url: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunOnceShown(&self) -> windows_core::Result<()>;
    fn SkipRunOnce(&self) -> windows_core::Result<()>;
    fn CustomizeSettings(&self, fsqm: super::wtypes::VARIANT_BOOL, fphishing: super::wtypes::VARIANT_BOOL, bstrlocale: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SqmEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn PhishingEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn BrandImageUri(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SkipTabsWelcome(&self) -> windows_core::Result<()>;
    fn DiagnoseConnection(&self) -> windows_core::Result<()>;
    fn CustomizeClearType(&self, fset: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsSearchProviderInstalled(&self, url: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn IsSearchMigrated(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn DefaultSearchProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RunOnceRequiredSettingsComplete(&self, fcomplete: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RunOnceHasShown(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SearchGuideUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper2_Vtbl {
    pub const fn new<Identity: IShellUIHelper2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSearchProvider<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::AddSearchProvider(this, core::mem::transmute(&url)).into()
            }
        }
        unsafe extern "system" fn RunOnceShown<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::RunOnceShown(this).into()
            }
        }
        unsafe extern "system" fn SkipRunOnce<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::SkipRunOnce(this).into()
            }
        }
        unsafe extern "system" fn CustomizeSettings<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsqm: super::wtypes::VARIANT_BOOL, fphishing: super::wtypes::VARIANT_BOOL, bstrlocale: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::CustomizeSettings(this, core::mem::transmute_copy(&fsqm), core::mem::transmute_copy(&fphishing), core::mem::transmute(&bstrlocale)).into()
            }
        }
        unsafe extern "system" fn SqmEnabled<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::SqmEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PhishingEnabled<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::PhishingEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BrandImageUri<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstruri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::BrandImageUri(this) {
                    Ok(ok__) => {
                        pbstruri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SkipTabsWelcome<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::SkipTabsWelcome(this).into()
            }
        }
        unsafe extern "system" fn DiagnoseConnection<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::DiagnoseConnection(this).into()
            }
        }
        unsafe extern "system" fn CustomizeClearType<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fset: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::CustomizeClearType(this, core::mem::transmute_copy(&fset)).into()
            }
        }
        unsafe extern "system" fn IsSearchProviderInstalled<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::IsSearchProviderInstalled(this, core::mem::transmute(&url)) {
                    Ok(ok__) => {
                        pdwresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSearchMigrated<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmigrated: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::IsSearchMigrated(this) {
                    Ok(ok__) => {
                        pfmigrated.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultSearchProvider<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::DefaultSearchProvider(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunOnceRequiredSettingsComplete<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcomplete: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper2_Impl::RunOnceRequiredSettingsComplete(this, core::mem::transmute_copy(&fcomplete)).into()
            }
        }
        unsafe extern "system" fn RunOnceHasShown<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfshown: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::RunOnceHasShown(this) {
                    Ok(ok__) => {
                        pfshown.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SearchGuideUrl<Identity: IShellUIHelper2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper2_Impl::SearchGuideUrl(this) {
                    Ok(ok__) => {
                        pbstrurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IShellUIHelper_Vtbl::new::<Identity, OFFSET>(),
            AddSearchProvider: AddSearchProvider::<Identity, OFFSET>,
            RunOnceShown: RunOnceShown::<Identity, OFFSET>,
            SkipRunOnce: SkipRunOnce::<Identity, OFFSET>,
            CustomizeSettings: CustomizeSettings::<Identity, OFFSET>,
            SqmEnabled: SqmEnabled::<Identity, OFFSET>,
            PhishingEnabled: PhishingEnabled::<Identity, OFFSET>,
            BrandImageUri: BrandImageUri::<Identity, OFFSET>,
            SkipTabsWelcome: SkipTabsWelcome::<Identity, OFFSET>,
            DiagnoseConnection: DiagnoseConnection::<Identity, OFFSET>,
            CustomizeClearType: CustomizeClearType::<Identity, OFFSET>,
            IsSearchProviderInstalled: IsSearchProviderInstalled::<Identity, OFFSET>,
            IsSearchMigrated: IsSearchMigrated::<Identity, OFFSET>,
            DefaultSearchProvider: DefaultSearchProvider::<Identity, OFFSET>,
            RunOnceRequiredSettingsComplete: RunOnceRequiredSettingsComplete::<Identity, OFFSET>,
            RunOnceHasShown: RunOnceHasShown::<Identity, OFFSET>,
            SearchGuideUrl: SearchGuideUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper3, IShellUIHelper3_Vtbl, 0x528df2ec_d419_40bc_9b6d_dcdbf9c1b25d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper3 {
    type Target = IShellUIHelper2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper3, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2);
#[cfg(feature = "oaidl")]
impl IShellUIHelper3 {
    pub unsafe fn AddService(&self, url: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url)) }
    }
    pub unsafe fn IsServiceInstalled(&self, url: &windows_core::BSTR, verb: &windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsServiceInstalled)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), core::mem::transmute_copy(verb), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn InPrivateFilteringEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InPrivateFilteringEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddToFavoritesBar(&self, url: &windows_core::BSTR, title: &windows_core::BSTR, r#type: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToFavoritesBar)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), core::mem::transmute_copy(title), r#type.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn BuildNewTabPage(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BuildNewTabPage)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRecentlyClosedVisible)(windows_core::Interface::as_raw(self), fvisible) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetActivitiesVisible(&self, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActivitiesVisible)(windows_core::Interface::as_raw(self), fvisible) }
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContentDiscoveryReset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSuggestedSitesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EnableSuggestedSites(&self, fenable: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableSuggestedSites)(windows_core::Interface::as_raw(self), fenable) }
    }
    pub unsafe fn NavigateToSuggestedSites(&self, bstrrelativeurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NavigateToSuggestedSites)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrelativeurl)) }
    }
    pub unsafe fn ShowTabsHelp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowTabsHelp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowInPrivateHelp)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper3_Vtbl {
    pub base__: IShellUIHelper2_Vtbl,
    pub AddService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsServiceInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub InPrivateFilteringEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    InPrivateFilteringEnabled: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddToFavoritesBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddToFavoritesBar: usize,
    pub BuildNewTabPage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetRecentlyClosedVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetRecentlyClosedVisible: usize,
    #[cfg(feature = "wtypes")]
    pub SetActivitiesVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetActivitiesVisible: usize,
    pub ContentDiscoveryReset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsSuggestedSitesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsSuggestedSitesEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub EnableSuggestedSites: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EnableSuggestedSites: usize,
    pub NavigateToSuggestedSites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowTabsHelp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowInPrivateHelp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper3_Impl: IShellUIHelper2_Impl {
    fn AddService(&self, url: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsServiceInstalled(&self, url: &windows_core::BSTR, verb: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn InPrivateFilteringEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn AddToFavoritesBar(&self, url: &windows_core::BSTR, title: &windows_core::BSTR, r#type: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn BuildNewTabPage(&self) -> windows_core::Result<()>;
    fn SetRecentlyClosedVisible(&self, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetActivitiesVisible(&self, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ContentDiscoveryReset(&self) -> windows_core::Result<()>;
    fn IsSuggestedSitesEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn EnableSuggestedSites(&self, fenable: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NavigateToSuggestedSites(&self, bstrrelativeurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShowTabsHelp(&self) -> windows_core::Result<()>;
    fn ShowInPrivateHelp(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper3_Vtbl {
    pub const fn new<Identity: IShellUIHelper3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddService<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::AddService(this, core::mem::transmute(&url)).into()
            }
        }
        unsafe extern "system" fn IsServiceInstalled<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, verb: *mut core::ffi::c_void, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper3_Impl::IsServiceInstalled(this, core::mem::transmute(&url), core::mem::transmute(&verb)) {
                    Ok(ok__) => {
                        pdwresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InPrivateFilteringEnabled<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper3_Impl::InPrivateFilteringEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddToFavoritesBar<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, title: *mut core::ffi::c_void, r#type: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::AddToFavoritesBar(this, core::mem::transmute(&url), core::mem::transmute(&title), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn BuildNewTabPage<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::BuildNewTabPage(this).into()
            }
        }
        unsafe extern "system" fn SetRecentlyClosedVisible<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::SetRecentlyClosedVisible(this, core::mem::transmute_copy(&fvisible)).into()
            }
        }
        unsafe extern "system" fn SetActivitiesVisible<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::SetActivitiesVisible(this, core::mem::transmute_copy(&fvisible)).into()
            }
        }
        unsafe extern "system" fn ContentDiscoveryReset<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::ContentDiscoveryReset(this).into()
            }
        }
        unsafe extern "system" fn IsSuggestedSitesEnabled<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper3_Impl::IsSuggestedSitesEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableSuggestedSites<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::EnableSuggestedSites(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn NavigateToSuggestedSites<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrelativeurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::NavigateToSuggestedSites(this, core::mem::transmute(&bstrrelativeurl)).into()
            }
        }
        unsafe extern "system" fn ShowTabsHelp<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::ShowTabsHelp(this).into()
            }
        }
        unsafe extern "system" fn ShowInPrivateHelp<Identity: IShellUIHelper3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper3_Impl::ShowInPrivateHelp(this).into()
            }
        }
        Self {
            base__: IShellUIHelper2_Vtbl::new::<Identity, OFFSET>(),
            AddService: AddService::<Identity, OFFSET>,
            IsServiceInstalled: IsServiceInstalled::<Identity, OFFSET>,
            InPrivateFilteringEnabled: InPrivateFilteringEnabled::<Identity, OFFSET>,
            AddToFavoritesBar: AddToFavoritesBar::<Identity, OFFSET>,
            BuildNewTabPage: BuildNewTabPage::<Identity, OFFSET>,
            SetRecentlyClosedVisible: SetRecentlyClosedVisible::<Identity, OFFSET>,
            SetActivitiesVisible: SetActivitiesVisible::<Identity, OFFSET>,
            ContentDiscoveryReset: ContentDiscoveryReset::<Identity, OFFSET>,
            IsSuggestedSitesEnabled: IsSuggestedSitesEnabled::<Identity, OFFSET>,
            EnableSuggestedSites: EnableSuggestedSites::<Identity, OFFSET>,
            NavigateToSuggestedSites: NavigateToSuggestedSites::<Identity, OFFSET>,
            ShowTabsHelp: ShowTabsHelp::<Identity, OFFSET>,
            ShowInPrivateHelp: ShowInPrivateHelp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper4, IShellUIHelper4_Vtbl, 0xb36e6a53_8073_499e_824c_d776330a333e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper4 {
    type Target = IShellUIHelper3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper4, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3);
#[cfg(feature = "oaidl")]
impl IShellUIHelper4 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn msIsSiteMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msIsSiteMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeShowThumbBar)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeAddThumbBarButton(&self, bstriconurl: &windows_core::BSTR, bstrtooltip: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msSiteModeAddThumbBarButton)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstriconurl), core::mem::transmute_copy(bstrtooltip), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton(&self, buttonid: &super::oaidl::VARIANT, fenabled: super::wtypes::VARIANT_BOOL, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeUpdateThumbBarButton)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(buttonid), fenabled, fvisible) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeSetIconOverlay(&self, iconurl: &windows_core::BSTR, pvardescription: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeSetIconOverlay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(iconurl), pvardescription.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeClearIconOverlay)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msAddSiteMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msAddSiteMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msSiteModeCreateJumpList(&self, bstrheader: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeCreateJumpList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrheader)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeAddJumpListItem(&self, bstrname: &windows_core::BSTR, bstractionuri: &windows_core::BSTR, bstriconuri: &windows_core::BSTR, pvarwindowtype: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeAddJumpListItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstractionuri), core::mem::transmute_copy(bstriconuri), pvarwindowtype.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeClearJumpList)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeShowJumpList)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeAddButtonStyle(&self, uibuttonid: &super::oaidl::VARIANT, bstriconurl: &windows_core::BSTR, bstrtooltip: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msSiteModeAddButtonStyle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uibuttonid), core::mem::transmute_copy(bstriconurl), core::mem::transmute_copy(bstrtooltip), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msSiteModeShowButtonStyle(&self, uibuttonid: &super::oaidl::VARIANT, uistyleid: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeShowButtonStyle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uibuttonid), core::mem::transmute_copy(uistyleid)) }
    }
    pub unsafe fn msSiteModeActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeActivate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msIsSiteModeFirstRun)(windows_core::Interface::as_raw(self), fpreservestate, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn msAddTrackingProtectionList(&self, url: &windows_core::BSTR, bstrfiltername: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msAddTrackingProtectionList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), core::mem::transmute_copy(bstrfiltername)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msTrackingProtectionEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msTrackingProtectionEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msActiveXFilteringEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msActiveXFilteringEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper4_Vtbl {
    pub base__: IShellUIHelper3_Vtbl,
    #[cfg(feature = "wtypes")]
    pub msIsSiteMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msIsSiteMode: usize,
    pub msSiteModeShowThumbBar: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeAddThumbBarButton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeAddThumbBarButton: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeUpdateThumbBarButton: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::wtypes::VARIANT_BOOL, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeUpdateThumbBarButton: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeSetIconOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeSetIconOverlay: usize,
    pub msSiteModeClearIconOverlay: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msAddSiteMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msSiteModeCreateJumpList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeAddJumpListItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeAddJumpListItem: usize,
    pub msSiteModeClearJumpList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msSiteModeShowJumpList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeAddButtonStyle: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeAddButtonStyle: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msSiteModeShowButtonStyle: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msSiteModeShowButtonStyle: usize,
    pub msSiteModeActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msIsSiteModeFirstRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msIsSiteModeFirstRun: usize,
    pub msAddTrackingProtectionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub msTrackingProtectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msTrackingProtectionEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub msActiveXFilteringEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msActiveXFilteringEnabled: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper4_Impl: IShellUIHelper3_Impl {
    fn msIsSiteMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn msSiteModeShowThumbBar(&self) -> windows_core::Result<()>;
    fn msSiteModeAddThumbBarButton(&self, bstriconurl: &windows_core::BSTR, bstrtooltip: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn msSiteModeUpdateThumbBarButton(&self, buttonid: &super::oaidl::VARIANT, fenabled: super::wtypes::VARIANT_BOOL, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn msSiteModeSetIconOverlay(&self, iconurl: &windows_core::BSTR, pvardescription: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msSiteModeClearIconOverlay(&self) -> windows_core::Result<()>;
    fn msAddSiteMode(&self) -> windows_core::Result<()>;
    fn msSiteModeCreateJumpList(&self, bstrheader: &windows_core::BSTR) -> windows_core::Result<()>;
    fn msSiteModeAddJumpListItem(&self, bstrname: &windows_core::BSTR, bstractionuri: &windows_core::BSTR, bstriconuri: &windows_core::BSTR, pvarwindowtype: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msSiteModeClearJumpList(&self) -> windows_core::Result<()>;
    fn msSiteModeShowJumpList(&self) -> windows_core::Result<()>;
    fn msSiteModeAddButtonStyle(&self, uibuttonid: &super::oaidl::VARIANT, bstriconurl: &windows_core::BSTR, bstrtooltip: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn msSiteModeShowButtonStyle(&self, uibuttonid: &super::oaidl::VARIANT, uistyleid: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msSiteModeActivate(&self) -> windows_core::Result<()>;
    fn msIsSiteModeFirstRun(&self, fpreservestate: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::oaidl::VARIANT>;
    fn msAddTrackingProtectionList(&self, url: &windows_core::BSTR, bstrfiltername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn msTrackingProtectionEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn msActiveXFilteringEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper4_Vtbl {
    pub const fn new<Identity: IShellUIHelper4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn msIsSiteMode<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsitemode: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msIsSiteMode(this) {
                    Ok(ok__) => {
                        pfsitemode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msSiteModeShowThumbBar<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeShowThumbBar(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeAddThumbBarButton<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstriconurl: *mut core::ffi::c_void, bstrtooltip: *mut core::ffi::c_void, pvarbuttonid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msSiteModeAddThumbBarButton(this, core::mem::transmute(&bstriconurl), core::mem::transmute(&bstrtooltip)) {
                    Ok(ok__) => {
                        pvarbuttonid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msSiteModeUpdateThumbBarButton<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buttonid: super::oaidl::VARIANT, fenabled: super::wtypes::VARIANT_BOOL, fvisible: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeUpdateThumbBarButton(this, core::mem::transmute(&buttonid), core::mem::transmute_copy(&fenabled), core::mem::transmute_copy(&fvisible)).into()
            }
        }
        unsafe extern "system" fn msSiteModeSetIconOverlay<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iconurl: *mut core::ffi::c_void, pvardescription: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeSetIconOverlay(this, core::mem::transmute(&iconurl), core::mem::transmute_copy(&pvardescription)).into()
            }
        }
        unsafe extern "system" fn msSiteModeClearIconOverlay<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeClearIconOverlay(this).into()
            }
        }
        unsafe extern "system" fn msAddSiteMode<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msAddSiteMode(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeCreateJumpList<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeCreateJumpList(this, core::mem::transmute(&bstrheader)).into()
            }
        }
        unsafe extern "system" fn msSiteModeAddJumpListItem<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstractionuri: *mut core::ffi::c_void, bstriconuri: *mut core::ffi::c_void, pvarwindowtype: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeAddJumpListItem(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstractionuri), core::mem::transmute(&bstriconuri), core::mem::transmute_copy(&pvarwindowtype)).into()
            }
        }
        unsafe extern "system" fn msSiteModeClearJumpList<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeClearJumpList(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeShowJumpList<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeShowJumpList(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeAddButtonStyle<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uibuttonid: super::oaidl::VARIANT, bstriconurl: *mut core::ffi::c_void, bstrtooltip: *mut core::ffi::c_void, pvarstyleid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msSiteModeAddButtonStyle(this, core::mem::transmute(&uibuttonid), core::mem::transmute(&bstriconurl), core::mem::transmute(&bstrtooltip)) {
                    Ok(ok__) => {
                        pvarstyleid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msSiteModeShowButtonStyle<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uibuttonid: super::oaidl::VARIANT, uistyleid: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeShowButtonStyle(this, core::mem::transmute(&uibuttonid), core::mem::transmute(&uistyleid)).into()
            }
        }
        unsafe extern "system" fn msSiteModeActivate<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msSiteModeActivate(this).into()
            }
        }
        unsafe extern "system" fn msIsSiteModeFirstRun<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpreservestate: super::wtypes::VARIANT_BOOL, puifirstrun: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msIsSiteModeFirstRun(this, core::mem::transmute_copy(&fpreservestate)) {
                    Ok(ok__) => {
                        puifirstrun.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msAddTrackingProtectionList<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, bstrfiltername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper4_Impl::msAddTrackingProtectionList(this, core::mem::transmute(&url), core::mem::transmute(&bstrfiltername)).into()
            }
        }
        unsafe extern "system" fn msTrackingProtectionEnabled<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msTrackingProtectionEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msActiveXFilteringEnabled<Identity: IShellUIHelper4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper4_Impl::msActiveXFilteringEnabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IShellUIHelper3_Vtbl::new::<Identity, OFFSET>(),
            msIsSiteMode: msIsSiteMode::<Identity, OFFSET>,
            msSiteModeShowThumbBar: msSiteModeShowThumbBar::<Identity, OFFSET>,
            msSiteModeAddThumbBarButton: msSiteModeAddThumbBarButton::<Identity, OFFSET>,
            msSiteModeUpdateThumbBarButton: msSiteModeUpdateThumbBarButton::<Identity, OFFSET>,
            msSiteModeSetIconOverlay: msSiteModeSetIconOverlay::<Identity, OFFSET>,
            msSiteModeClearIconOverlay: msSiteModeClearIconOverlay::<Identity, OFFSET>,
            msAddSiteMode: msAddSiteMode::<Identity, OFFSET>,
            msSiteModeCreateJumpList: msSiteModeCreateJumpList::<Identity, OFFSET>,
            msSiteModeAddJumpListItem: msSiteModeAddJumpListItem::<Identity, OFFSET>,
            msSiteModeClearJumpList: msSiteModeClearJumpList::<Identity, OFFSET>,
            msSiteModeShowJumpList: msSiteModeShowJumpList::<Identity, OFFSET>,
            msSiteModeAddButtonStyle: msSiteModeAddButtonStyle::<Identity, OFFSET>,
            msSiteModeShowButtonStyle: msSiteModeShowButtonStyle::<Identity, OFFSET>,
            msSiteModeActivate: msSiteModeActivate::<Identity, OFFSET>,
            msIsSiteModeFirstRun: msIsSiteModeFirstRun::<Identity, OFFSET>,
            msAddTrackingProtectionList: msAddTrackingProtectionList::<Identity, OFFSET>,
            msTrackingProtectionEnabled: msTrackingProtectionEnabled::<Identity, OFFSET>,
            msActiveXFilteringEnabled: msActiveXFilteringEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper4 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper4 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper5, IShellUIHelper5_Vtbl, 0xa2a08b09_103d_4d3f_b91c_ea455ca82efa);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper5 {
    type Target = IShellUIHelper4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper5, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3, IShellUIHelper4);
#[cfg(feature = "oaidl")]
impl IShellUIHelper5 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msProvisionNetworks(&self, bstrprovisioningxml: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msProvisionNetworks)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprovisioningxml), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn msReportSafeUrl(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msReportSafeUrl)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeRefreshBadge)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msSiteModeClearBadge)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msDiagnoseConnectionUILess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msLaunchNetworkClientHelp)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msChangeDefaultBrowser)(windows_core::Interface::as_raw(self), fchange) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper5_Vtbl {
    pub base__: IShellUIHelper4_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msProvisionNetworks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msProvisionNetworks: usize,
    pub msReportSafeUrl: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msSiteModeRefreshBadge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msSiteModeClearBadge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msDiagnoseConnectionUILess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msLaunchNetworkClientHelp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub msChangeDefaultBrowser: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msChangeDefaultBrowser: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper5_Impl: IShellUIHelper4_Impl {
    fn msProvisionNetworks(&self, bstrprovisioningxml: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn msReportSafeUrl(&self) -> windows_core::Result<()>;
    fn msSiteModeRefreshBadge(&self) -> windows_core::Result<()>;
    fn msSiteModeClearBadge(&self) -> windows_core::Result<()>;
    fn msDiagnoseConnectionUILess(&self) -> windows_core::Result<()>;
    fn msLaunchNetworkClientHelp(&self) -> windows_core::Result<()>;
    fn msChangeDefaultBrowser(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper5_Vtbl {
    pub const fn new<Identity: IShellUIHelper5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn msProvisionNetworks<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovisioningxml: *mut core::ffi::c_void, puiresult: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper5_Impl::msProvisionNetworks(this, core::mem::transmute(&bstrprovisioningxml)) {
                    Ok(ok__) => {
                        puiresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msReportSafeUrl<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msReportSafeUrl(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeRefreshBadge<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msSiteModeRefreshBadge(this).into()
            }
        }
        unsafe extern "system" fn msSiteModeClearBadge<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msSiteModeClearBadge(this).into()
            }
        }
        unsafe extern "system" fn msDiagnoseConnectionUILess<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msDiagnoseConnectionUILess(this).into()
            }
        }
        unsafe extern "system" fn msLaunchNetworkClientHelp<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msLaunchNetworkClientHelp(this).into()
            }
        }
        unsafe extern "system" fn msChangeDefaultBrowser<Identity: IShellUIHelper5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper5_Impl::msChangeDefaultBrowser(this, core::mem::transmute_copy(&fchange)).into()
            }
        }
        Self {
            base__: IShellUIHelper4_Vtbl::new::<Identity, OFFSET>(),
            msProvisionNetworks: msProvisionNetworks::<Identity, OFFSET>,
            msReportSafeUrl: msReportSafeUrl::<Identity, OFFSET>,
            msSiteModeRefreshBadge: msSiteModeRefreshBadge::<Identity, OFFSET>,
            msSiteModeClearBadge: msSiteModeClearBadge::<Identity, OFFSET>,
            msDiagnoseConnectionUILess: msDiagnoseConnectionUILess::<Identity, OFFSET>,
            msLaunchNetworkClientHelp: msLaunchNetworkClientHelp::<Identity, OFFSET>,
            msChangeDefaultBrowser: msChangeDefaultBrowser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper5 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<IShellUIHelper4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper5 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper6, IShellUIHelper6_Vtbl, 0x987a573e_46ee_4e89_96ab_ddf7f8fdc98c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper6 {
    type Target = IShellUIHelper5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper6, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3, IShellUIHelper4, IShellUIHelper5);
#[cfg(feature = "oaidl")]
impl IShellUIHelper6 {
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msStopPeriodicTileUpdate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msStartPeriodicTileUpdate(&self, pollinguris: &super::oaidl::VARIANT, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msStartPeriodicTileUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pollinguris), core::mem::transmute_copy(starttime), core::mem::transmute_copy(uiupdaterecurrence)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch(&self, pollinguris: &super::oaidl::VARIANT, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msStartPeriodicTileUpdateBatch)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pollinguris), core::mem::transmute_copy(starttime), core::mem::transmute_copy(uiupdaterecurrence)) }
    }
    pub unsafe fn msClearTile(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msClearTile)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msEnableTileNotificationQueue(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msEnableTileNotificationQueue)(windows_core::Interface::as_raw(self), fchange) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msPinnedSiteState(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).msPinnedSiteState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msEnableTileNotificationQueueForSquare150x150)(windows_core::Interface::as_raw(self), fchange) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msEnableTileNotificationQueueForWide310x150(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msEnableTileNotificationQueueForWide310x150)(windows_core::Interface::as_raw(self), fchange) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msEnableTileNotificationQueueForSquare310x310)(windows_core::Interface::as_raw(self), fchange) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msScheduledTileNotification(&self, bstrnotificationxml: &windows_core::BSTR, bstrnotificationid: &windows_core::BSTR, bstrnotificationtag: &windows_core::BSTR, starttime: &super::oaidl::VARIANT, expirationtime: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msScheduledTileNotification)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotificationxml), core::mem::transmute_copy(bstrnotificationid), core::mem::transmute_copy(bstrnotificationtag), core::mem::transmute_copy(starttime), core::mem::transmute_copy(expirationtime)) }
    }
    pub unsafe fn msRemoveScheduledTileNotification(&self, bstrnotificationid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msRemoveScheduledTileNotification)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnotificationid)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn msStartPeriodicBadgeUpdate(&self, pollinguri: &windows_core::BSTR, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msStartPeriodicBadgeUpdate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pollinguri), core::mem::transmute_copy(starttime), core::mem::transmute_copy(uiupdaterecurrence)) }
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msStopPeriodicBadgeUpdate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).msLaunchInternetOptions)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper6_Vtbl {
    pub base__: IShellUIHelper5_Vtbl,
    pub msStopPeriodicTileUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msStartPeriodicTileUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msStartPeriodicTileUpdate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msStartPeriodicTileUpdateBatch: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msStartPeriodicTileUpdateBatch: usize,
    pub msClearTile: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub msEnableTileNotificationQueue: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msEnableTileNotificationQueue: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msPinnedSiteState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msPinnedSiteState: usize,
    #[cfg(feature = "wtypes")]
    pub msEnableTileNotificationQueueForSquare150x150: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msEnableTileNotificationQueueForSquare150x150: usize,
    #[cfg(feature = "wtypes")]
    pub msEnableTileNotificationQueueForWide310x150: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msEnableTileNotificationQueueForWide310x150: usize,
    #[cfg(feature = "wtypes")]
    pub msEnableTileNotificationQueueForSquare310x310: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    msEnableTileNotificationQueueForSquare310x310: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msScheduledTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msScheduledTileNotification: usize,
    pub msRemoveScheduledTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub msStartPeriodicBadgeUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    msStartPeriodicBadgeUpdate: usize,
    pub msStopPeriodicBadgeUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub msLaunchInternetOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper6_Impl: IShellUIHelper5_Impl {
    fn msStopPeriodicTileUpdate(&self) -> windows_core::Result<()>;
    fn msStartPeriodicTileUpdate(&self, pollinguris: &super::oaidl::VARIANT, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msStartPeriodicTileUpdateBatch(&self, pollinguris: &super::oaidl::VARIANT, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msClearTile(&self) -> windows_core::Result<()>;
    fn msEnableTileNotificationQueue(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn msPinnedSiteState(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn msEnableTileNotificationQueueForWide310x150(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn msScheduledTileNotification(&self, bstrnotificationxml: &windows_core::BSTR, bstrnotificationid: &windows_core::BSTR, bstrnotificationtag: &windows_core::BSTR, starttime: &super::oaidl::VARIANT, expirationtime: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msRemoveScheduledTileNotification(&self, bstrnotificationid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn msStartPeriodicBadgeUpdate(&self, pollinguri: &windows_core::BSTR, starttime: &super::oaidl::VARIANT, uiupdaterecurrence: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn msStopPeriodicBadgeUpdate(&self) -> windows_core::Result<()>;
    fn msLaunchInternetOptions(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper6_Vtbl {
    pub const fn new<Identity: IShellUIHelper6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn msStopPeriodicTileUpdate<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msStopPeriodicTileUpdate(this).into()
            }
        }
        unsafe extern "system" fn msStartPeriodicTileUpdate<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollinguris: super::oaidl::VARIANT, starttime: super::oaidl::VARIANT, uiupdaterecurrence: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msStartPeriodicTileUpdate(this, core::mem::transmute(&pollinguris), core::mem::transmute(&starttime), core::mem::transmute(&uiupdaterecurrence)).into()
            }
        }
        unsafe extern "system" fn msStartPeriodicTileUpdateBatch<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollinguris: super::oaidl::VARIANT, starttime: super::oaidl::VARIANT, uiupdaterecurrence: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msStartPeriodicTileUpdateBatch(this, core::mem::transmute(&pollinguris), core::mem::transmute(&starttime), core::mem::transmute(&uiupdaterecurrence)).into()
            }
        }
        unsafe extern "system" fn msClearTile<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msClearTile(this).into()
            }
        }
        unsafe extern "system" fn msEnableTileNotificationQueue<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msEnableTileNotificationQueue(this, core::mem::transmute_copy(&fchange)).into()
            }
        }
        unsafe extern "system" fn msPinnedSiteState<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsitestate: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper6_Impl::msPinnedSiteState(this) {
                    Ok(ok__) => {
                        pvarsitestate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn msEnableTileNotificationQueueForSquare150x150<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msEnableTileNotificationQueueForSquare150x150(this, core::mem::transmute_copy(&fchange)).into()
            }
        }
        unsafe extern "system" fn msEnableTileNotificationQueueForWide310x150<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msEnableTileNotificationQueueForWide310x150(this, core::mem::transmute_copy(&fchange)).into()
            }
        }
        unsafe extern "system" fn msEnableTileNotificationQueueForSquare310x310<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchange: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msEnableTileNotificationQueueForSquare310x310(this, core::mem::transmute_copy(&fchange)).into()
            }
        }
        unsafe extern "system" fn msScheduledTileNotification<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotificationxml: *mut core::ffi::c_void, bstrnotificationid: *mut core::ffi::c_void, bstrnotificationtag: *mut core::ffi::c_void, starttime: super::oaidl::VARIANT, expirationtime: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msScheduledTileNotification(this, core::mem::transmute(&bstrnotificationxml), core::mem::transmute(&bstrnotificationid), core::mem::transmute(&bstrnotificationtag), core::mem::transmute(&starttime), core::mem::transmute(&expirationtime)).into()
            }
        }
        unsafe extern "system" fn msRemoveScheduledTileNotification<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotificationid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msRemoveScheduledTileNotification(this, core::mem::transmute(&bstrnotificationid)).into()
            }
        }
        unsafe extern "system" fn msStartPeriodicBadgeUpdate<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollinguri: *mut core::ffi::c_void, starttime: super::oaidl::VARIANT, uiupdaterecurrence: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msStartPeriodicBadgeUpdate(this, core::mem::transmute(&pollinguri), core::mem::transmute(&starttime), core::mem::transmute(&uiupdaterecurrence)).into()
            }
        }
        unsafe extern "system" fn msStopPeriodicBadgeUpdate<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msStopPeriodicBadgeUpdate(this).into()
            }
        }
        unsafe extern "system" fn msLaunchInternetOptions<Identity: IShellUIHelper6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper6_Impl::msLaunchInternetOptions(this).into()
            }
        }
        Self {
            base__: IShellUIHelper5_Vtbl::new::<Identity, OFFSET>(),
            msStopPeriodicTileUpdate: msStopPeriodicTileUpdate::<Identity, OFFSET>,
            msStartPeriodicTileUpdate: msStartPeriodicTileUpdate::<Identity, OFFSET>,
            msStartPeriodicTileUpdateBatch: msStartPeriodicTileUpdateBatch::<Identity, OFFSET>,
            msClearTile: msClearTile::<Identity, OFFSET>,
            msEnableTileNotificationQueue: msEnableTileNotificationQueue::<Identity, OFFSET>,
            msPinnedSiteState: msPinnedSiteState::<Identity, OFFSET>,
            msEnableTileNotificationQueueForSquare150x150: msEnableTileNotificationQueueForSquare150x150::<Identity, OFFSET>,
            msEnableTileNotificationQueueForWide310x150: msEnableTileNotificationQueueForWide310x150::<Identity, OFFSET>,
            msEnableTileNotificationQueueForSquare310x310: msEnableTileNotificationQueueForSquare310x310::<Identity, OFFSET>,
            msScheduledTileNotification: msScheduledTileNotification::<Identity, OFFSET>,
            msRemoveScheduledTileNotification: msRemoveScheduledTileNotification::<Identity, OFFSET>,
            msStartPeriodicBadgeUpdate: msStartPeriodicBadgeUpdate::<Identity, OFFSET>,
            msStopPeriodicBadgeUpdate: msStopPeriodicBadgeUpdate::<Identity, OFFSET>,
            msLaunchInternetOptions: msLaunchInternetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper6 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<IShellUIHelper4 as windows_core::Interface>::IID || iid == &<IShellUIHelper5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper6 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper7, IShellUIHelper7_Vtbl, 0x60e567c8_9573_4ab2_a264_637c6c161cb1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper7 {
    type Target = IShellUIHelper6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper7, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3, IShellUIHelper4, IShellUIHelper5, IShellUIHelper6);
#[cfg(feature = "oaidl")]
impl IShellUIHelper7 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetExperimentalFlag(&self, bstrflagstring: &windows_core::BSTR, vfflag: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExperimentalFlag)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrflagstring), vfflag) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetExperimentalFlag(&self, bstrflagstring: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExperimentalFlag)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrflagstring), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExperimentalValue(&self, bstrvaluestring: &windows_core::BSTR, dwvalue: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExperimentalValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvaluestring), dwvalue) }
    }
    pub unsafe fn GetExperimentalValue(&self, bstrvaluestring: &windows_core::BSTR) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExperimentalValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrvaluestring), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetAllExperimentalFlagsAndValues)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNeedIEAutoLaunchFlag)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR, flag: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNeedIEAutoLaunchFlag)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), flag) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn HasNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasNeedIEAutoLaunchFlag)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LaunchIE(&self, bstrurl: &windows_core::BSTR, automated: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LaunchIE)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), automated) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper7_Vtbl {
    pub base__: IShellUIHelper6_Vtbl,
    #[cfg(feature = "wtypes")]
    pub SetExperimentalFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetExperimentalFlag: usize,
    #[cfg(feature = "wtypes")]
    pub GetExperimentalFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetExperimentalFlag: usize,
    pub SetExperimentalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExperimentalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ResetAllExperimentalFlagsAndValues: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub GetNeedIEAutoLaunchFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetNeedIEAutoLaunchFlag: usize,
    #[cfg(feature = "wtypes")]
    pub SetNeedIEAutoLaunchFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetNeedIEAutoLaunchFlag: usize,
    #[cfg(feature = "wtypes")]
    pub HasNeedIEAutoLaunchFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    HasNeedIEAutoLaunchFlag: usize,
    #[cfg(feature = "wtypes")]
    pub LaunchIE: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LaunchIE: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper7_Impl: IShellUIHelper6_Impl {
    fn SetExperimentalFlag(&self, bstrflagstring: &windows_core::BSTR, vfflag: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetExperimentalFlag(&self, bstrflagstring: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetExperimentalValue(&self, bstrvaluestring: &windows_core::BSTR, dwvalue: u32) -> windows_core::Result<()>;
    fn GetExperimentalValue(&self, bstrvaluestring: &windows_core::BSTR) -> windows_core::Result<u32>;
    fn ResetAllExperimentalFlagsAndValues(&self) -> windows_core::Result<()>;
    fn GetNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR, flag: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HasNeedIEAutoLaunchFlag(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn LaunchIE(&self, bstrurl: &windows_core::BSTR, automated: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper7_Vtbl {
    pub const fn new<Identity: IShellUIHelper7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetExperimentalFlag<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrflagstring: *mut core::ffi::c_void, vfflag: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper7_Impl::SetExperimentalFlag(this, core::mem::transmute(&bstrflagstring), core::mem::transmute_copy(&vfflag)).into()
            }
        }
        unsafe extern "system" fn GetExperimentalFlag<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrflagstring: *mut core::ffi::c_void, vfflag: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper7_Impl::GetExperimentalFlag(this, core::mem::transmute(&bstrflagstring)) {
                    Ok(ok__) => {
                        vfflag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExperimentalValue<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluestring: *mut core::ffi::c_void, dwvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper7_Impl::SetExperimentalValue(this, core::mem::transmute(&bstrvaluestring), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn GetExperimentalValue<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvaluestring: *mut core::ffi::c_void, pdwvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper7_Impl::GetExperimentalValue(this, core::mem::transmute(&bstrvaluestring)) {
                    Ok(ok__) => {
                        pdwvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetAllExperimentalFlagsAndValues<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper7_Impl::ResetAllExperimentalFlagsAndValues(this).into()
            }
        }
        unsafe extern "system" fn GetNeedIEAutoLaunchFlag<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, flag: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper7_Impl::GetNeedIEAutoLaunchFlag(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        flag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNeedIEAutoLaunchFlag<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, flag: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper7_Impl::SetNeedIEAutoLaunchFlag(this, core::mem::transmute(&bstrurl), core::mem::transmute_copy(&flag)).into()
            }
        }
        unsafe extern "system" fn HasNeedIEAutoLaunchFlag<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, exists: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper7_Impl::HasNeedIEAutoLaunchFlag(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        exists.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LaunchIE<Identity: IShellUIHelper7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, automated: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper7_Impl::LaunchIE(this, core::mem::transmute(&bstrurl), core::mem::transmute_copy(&automated)).into()
            }
        }
        Self {
            base__: IShellUIHelper6_Vtbl::new::<Identity, OFFSET>(),
            SetExperimentalFlag: SetExperimentalFlag::<Identity, OFFSET>,
            GetExperimentalFlag: GetExperimentalFlag::<Identity, OFFSET>,
            SetExperimentalValue: SetExperimentalValue::<Identity, OFFSET>,
            GetExperimentalValue: GetExperimentalValue::<Identity, OFFSET>,
            ResetAllExperimentalFlagsAndValues: ResetAllExperimentalFlagsAndValues::<Identity, OFFSET>,
            GetNeedIEAutoLaunchFlag: GetNeedIEAutoLaunchFlag::<Identity, OFFSET>,
            SetNeedIEAutoLaunchFlag: SetNeedIEAutoLaunchFlag::<Identity, OFFSET>,
            HasNeedIEAutoLaunchFlag: HasNeedIEAutoLaunchFlag::<Identity, OFFSET>,
            LaunchIE: LaunchIE::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper7 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<IShellUIHelper4 as windows_core::Interface>::IID || iid == &<IShellUIHelper5 as windows_core::Interface>::IID || iid == &<IShellUIHelper6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper7 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper8, IShellUIHelper8_Vtbl, 0x66debcf2_05b0_4f07_b49b_b96241a65db2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper8 {
    type Target = IShellUIHelper7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper8, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3, IShellUIHelper4, IShellUIHelper5, IShellUIHelper6, IShellUIHelper7);
#[cfg(feature = "oaidl")]
impl IShellUIHelper8 {
    pub unsafe fn GetCVListData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCVListData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCVListLocalData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCVListLocalData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetEMIEListData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEMIEListData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetEMIEListLocalData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEMIEListLocalData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OpenFavoritesPane(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenFavoritesPane)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OpenFavoritesSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenFavoritesSettings)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LaunchInHVSI(&self, bstrurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LaunchInHVSI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper8_Vtbl {
    pub base__: IShellUIHelper7_Vtbl,
    pub GetCVListData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCVListLocalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEMIEListData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEMIEListLocalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenFavoritesPane: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenFavoritesSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchInHVSI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper8_Impl: IShellUIHelper7_Impl {
    fn GetCVListData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCVListLocalData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetEMIEListData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetEMIEListLocalData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OpenFavoritesPane(&self) -> windows_core::Result<()>;
    fn OpenFavoritesSettings(&self) -> windows_core::Result<()>;
    fn LaunchInHVSI(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper8_Vtbl {
    pub const fn new<Identity: IShellUIHelper8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCVListData<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper8_Impl::GetCVListData(this) {
                    Ok(ok__) => {
                        pbstrresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCVListLocalData<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper8_Impl::GetCVListLocalData(this) {
                    Ok(ok__) => {
                        pbstrresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEMIEListData<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper8_Impl::GetEMIEListData(this) {
                    Ok(ok__) => {
                        pbstrresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEMIEListLocalData<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper8_Impl::GetEMIEListLocalData(this) {
                    Ok(ok__) => {
                        pbstrresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenFavoritesPane<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper8_Impl::OpenFavoritesPane(this).into()
            }
        }
        unsafe extern "system" fn OpenFavoritesSettings<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper8_Impl::OpenFavoritesSettings(this).into()
            }
        }
        unsafe extern "system" fn LaunchInHVSI<Identity: IShellUIHelper8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellUIHelper8_Impl::LaunchInHVSI(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        Self {
            base__: IShellUIHelper7_Vtbl::new::<Identity, OFFSET>(),
            GetCVListData: GetCVListData::<Identity, OFFSET>,
            GetCVListLocalData: GetCVListLocalData::<Identity, OFFSET>,
            GetEMIEListData: GetEMIEListData::<Identity, OFFSET>,
            GetEMIEListLocalData: GetEMIEListLocalData::<Identity, OFFSET>,
            OpenFavoritesPane: OpenFavoritesPane::<Identity, OFFSET>,
            OpenFavoritesSettings: OpenFavoritesSettings::<Identity, OFFSET>,
            LaunchInHVSI: LaunchInHVSI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper8 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<IShellUIHelper4 as windows_core::Interface>::IID || iid == &<IShellUIHelper5 as windows_core::Interface>::IID || iid == &<IShellUIHelper6 as windows_core::Interface>::IID || iid == &<IShellUIHelper7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper8 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellUIHelper9, IShellUIHelper9_Vtbl, 0x6cdf73b0_7f2f_451f_bc0f_63e0f3284e54);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellUIHelper9 {
    type Target = IShellUIHelper8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellUIHelper9, windows_core::IUnknown, super::oaidl::IDispatch, IShellUIHelper, IShellUIHelper2, IShellUIHelper3, IShellUIHelper4, IShellUIHelper5, IShellUIHelper6, IShellUIHelper7, IShellUIHelper8);
#[cfg(feature = "oaidl")]
impl IShellUIHelper9 {
    pub unsafe fn GetOSSku(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOSSku)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper9_Vtbl {
    pub base__: IShellUIHelper8_Vtbl,
    pub GetOSSku: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellUIHelper9_Impl: IShellUIHelper8_Impl {
    fn GetOSSku(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellUIHelper9_Vtbl {
    pub const fn new<Identity: IShellUIHelper9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOSSku<Identity: IShellUIHelper9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellUIHelper9_Impl::GetOSSku(this) {
                    Ok(ok__) => {
                        pdwresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IShellUIHelper8_Vtbl::new::<Identity, OFFSET>(), GetOSSku: GetOSSku::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellUIHelper9 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellUIHelper as windows_core::Interface>::IID || iid == &<IShellUIHelper2 as windows_core::Interface>::IID || iid == &<IShellUIHelper3 as windows_core::Interface>::IID || iid == &<IShellUIHelper4 as windows_core::Interface>::IID || iid == &<IShellUIHelper5 as windows_core::Interface>::IID || iid == &<IShellUIHelper6 as windows_core::Interface>::IID || iid == &<IShellUIHelper7 as windows_core::Interface>::IID || iid == &<IShellUIHelper8 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellUIHelper9 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellWindows, IShellWindows_Vtbl, 0x85cb6900_4d95_11cf_960c_0080c7f4ee85);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellWindows {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellWindows, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellWindows {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Register<P0>(&self, pid: P0, hwnd: i32, swclass: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), pid.param().abi(), hwnd, swclass, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RegisterPending(&self, lthreadid: i32, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterPending)(windows_core::Interface::as_raw(self), lthreadid, core::mem::transmute(pvarloc), core::mem::transmute(pvarlocroot), swclass, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Revoke(&self, lcookie: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), lcookie) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn OnNavigate(&self, lcookie: i32, pvarloc: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNavigate)(windows_core::Interface::as_raw(self), lcookie, core::mem::transmute(pvarloc)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn OnActivated(&self, lcookie: i32, factive: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnActivated)(windows_core::Interface::as_raw(self), lcookie, factive) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindWindowSW(&self, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32, phwnd: *mut i32, swfwoptions: i32) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindWindowSW)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarloc), core::mem::transmute(pvarlocroot), swclass, phwnd as _, swfwoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnCreated<P1>(&self, lcookie: i32, punk: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCreated)(windows_core::Interface::as_raw(self), lcookie, punk.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ProcessAttachDetach(&self, fattach: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessAttachDetach)(windows_core::Interface::as_raw(self), fattach) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellWindows_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub RegisterPending: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    RegisterPending: usize,
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub OnNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    OnNavigate: usize,
    #[cfg(feature = "wtypes")]
    pub OnActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    OnActivated: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub FindWindowSW: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, i32, *mut i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    FindWindowSW: usize,
    pub OnCreated: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ProcessAttachDetach: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ProcessAttachDetach: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellWindows_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Register(&self, pid: windows_core::Ref<super::oaidl::IDispatch>, hwnd: i32, swclass: i32) -> windows_core::Result<i32>;
    fn RegisterPending(&self, lthreadid: i32, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32) -> windows_core::Result<i32>;
    fn Revoke(&self, lcookie: i32) -> windows_core::Result<()>;
    fn OnNavigate(&self, lcookie: i32, pvarloc: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn OnActivated(&self, lcookie: i32, factive: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FindWindowSW(&self, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32, phwnd: *mut i32, swfwoptions: i32) -> windows_core::Result<super::oaidl::IDispatch>;
    fn OnCreated(&self, lcookie: i32, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ProcessAttachDetach(&self, fattach: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellWindows_Vtbl {
    pub const fn new<Identity: IShellWindows_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Register<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut core::ffi::c_void, hwnd: i32, swclass: i32, plcookie: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::Register(this, core::mem::transmute_copy(&pid), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&swclass)) {
                    Ok(ok__) => {
                        plcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterPending<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lthreadid: i32, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32, plcookie: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::RegisterPending(this, core::mem::transmute_copy(&lthreadid), core::mem::transmute_copy(&pvarloc), core::mem::transmute_copy(&pvarlocroot), core::mem::transmute_copy(&swclass)) {
                    Ok(ok__) => {
                        plcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Revoke<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellWindows_Impl::Revoke(this, core::mem::transmute_copy(&lcookie)).into()
            }
        }
        unsafe extern "system" fn OnNavigate<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: i32, pvarloc: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellWindows_Impl::OnNavigate(this, core::mem::transmute_copy(&lcookie), core::mem::transmute_copy(&pvarloc)).into()
            }
        }
        unsafe extern "system" fn OnActivated<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: i32, factive: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellWindows_Impl::OnActivated(this, core::mem::transmute_copy(&lcookie), core::mem::transmute_copy(&factive)).into()
            }
        }
        unsafe extern "system" fn FindWindowSW<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarloc: *const super::oaidl::VARIANT, pvarlocroot: *const super::oaidl::VARIANT, swclass: i32, phwnd: *mut i32, swfwoptions: i32, ppdispout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellWindows_Impl::FindWindowSW(this, core::mem::transmute_copy(&pvarloc), core::mem::transmute_copy(&pvarlocroot), core::mem::transmute_copy(&swclass), core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&swfwoptions)) {
                    Ok(ok__) => {
                        ppdispout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnCreated<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcookie: i32, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellWindows_Impl::OnCreated(this, core::mem::transmute_copy(&lcookie), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn ProcessAttachDetach<Identity: IShellWindows_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fattach: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellWindows_Impl::ProcessAttachDetach(this, core::mem::transmute_copy(&fattach)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            RegisterPending: RegisterPending::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            OnNavigate: OnNavigate::<Identity, OFFSET>,
            OnActivated: OnActivated::<Identity, OFFSET>,
            FindWindowSW: FindWindowSW::<Identity, OFFSET>,
            OnCreated: OnCreated::<Identity, OFFSET>,
            ProcessAttachDetach: ProcessAttachDetach::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellWindows as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellWindows {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebBrowser, IWebBrowser_Vtbl, 0xeab22ac1_30c1_11cf_a7eb_0000c05bae0b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebBrowser {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebBrowser, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IWebBrowser {
    pub unsafe fn GoBack(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GoBack)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GoForward(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GoForward)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GoHome(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GoHome)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GoSearch(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GoSearch)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Navigate(&self, url: &windows_core::BSTR, flags: Option<*const super::oaidl::VARIANT>, targetframename: Option<*const super::oaidl::VARIANT>, postdata: Option<*const super::oaidl::VARIANT>, headers: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(url), flags.unwrap_or(core::mem::zeroed()) as _, targetframename.unwrap_or(core::mem::zeroed()) as _, postdata.unwrap_or(core::mem::zeroed()) as _, headers.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Refresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Refresh2(&self, level: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh2)(windows_core::Interface::as_raw(self), level.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Container(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Document(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Document)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TopLevelContainer(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TopLevelContainer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Left(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Left)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLeft(&self, left: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLeft)(windows_core::Interface::as_raw(self), left) }
    }
    pub unsafe fn Top(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Top)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTop(&self, top: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTop)(windows_core::Interface::as_raw(self), top) }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWidth(&self, width: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), width) }
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHeight(&self, height: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHeight)(windows_core::Interface::as_raw(self), height) }
    }
    pub unsafe fn LocationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LocationURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocationURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Busy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Busy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowser_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GoBack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoForward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoHome: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoSearch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Navigate: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Refresh2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Refresh2: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Document: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub TopLevelContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TopLevelContainer: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Top: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LocationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocationURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Busy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Busy: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebBrowser_Impl: super::oaidl::IDispatch_Impl {
    fn GoBack(&self) -> windows_core::Result<()>;
    fn GoForward(&self) -> windows_core::Result<()>;
    fn GoHome(&self) -> windows_core::Result<()>;
    fn GoSearch(&self) -> windows_core::Result<()>;
    fn Navigate(&self, url: &windows_core::BSTR, flags: *const super::oaidl::VARIANT, targetframename: *const super::oaidl::VARIANT, postdata: *const super::oaidl::VARIANT, headers: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Refresh2(&self, level: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Container(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Document(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn TopLevelContainer(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Left(&self) -> windows_core::Result<i32>;
    fn SetLeft(&self, left: i32) -> windows_core::Result<()>;
    fn Top(&self) -> windows_core::Result<i32>;
    fn SetTop(&self, top: i32) -> windows_core::Result<()>;
    fn Width(&self) -> windows_core::Result<i32>;
    fn SetWidth(&self, width: i32) -> windows_core::Result<()>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn SetHeight(&self, height: i32) -> windows_core::Result<()>;
    fn LocationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LocationURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Busy(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebBrowser_Vtbl {
    pub const fn new<Identity: IWebBrowser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GoBack<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::GoBack(this).into()
            }
        }
        unsafe extern "system" fn GoForward<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::GoForward(this).into()
            }
        }
        unsafe extern "system" fn GoHome<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::GoHome(this).into()
            }
        }
        unsafe extern "system" fn GoSearch<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::GoSearch(this).into()
            }
        }
        unsafe extern "system" fn Navigate<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *mut core::ffi::c_void, flags: *const super::oaidl::VARIANT, targetframename: *const super::oaidl::VARIANT, postdata: *const super::oaidl::VARIANT, headers: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::Navigate(this, core::mem::transmute(&url), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&targetframename), core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&headers)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Refresh2<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::Refresh2(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Application<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Application(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Container<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Container(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Document<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Document(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TopLevelContainer<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::TopLevelContainer(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Left<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Left(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLeft<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::SetLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn Top<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Top(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTop<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::SetTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn Width<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Width(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWidth<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::SetWidth(this, core::mem::transmute_copy(&width)).into()
            }
        }
        unsafe extern "system" fn Height<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Height(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeight<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, height: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser_Impl::SetHeight(this, core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn LocationName<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locationname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::LocationName(this) {
                    Ok(ok__) => {
                        locationname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocationURL<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locationurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::LocationURL(this) {
                    Ok(ok__) => {
                        locationurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Busy<Identity: IWebBrowser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser_Impl::Busy(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GoBack: GoBack::<Identity, OFFSET>,
            GoForward: GoForward::<Identity, OFFSET>,
            GoHome: GoHome::<Identity, OFFSET>,
            GoSearch: GoSearch::<Identity, OFFSET>,
            Navigate: Navigate::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Refresh2: Refresh2::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Container: Container::<Identity, OFFSET>,
            Document: Document::<Identity, OFFSET>,
            TopLevelContainer: TopLevelContainer::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Left: Left::<Identity, OFFSET>,
            SetLeft: SetLeft::<Identity, OFFSET>,
            Top: Top::<Identity, OFFSET>,
            SetTop: SetTop::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
            SetWidth: SetWidth::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            SetHeight: SetHeight::<Identity, OFFSET>,
            LocationName: LocationName::<Identity, OFFSET>,
            LocationURL: LocationURL::<Identity, OFFSET>,
            Busy: Busy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowser as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebBrowser {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebBrowser2, IWebBrowser2_Vtbl, 0xd30c1661_cdaf_11d0_8a3e_00c04fc9e26e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebBrowser2 {
    type Target = IWebBrowserApp;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebBrowser2, windows_core::IUnknown, super::oaidl::IDispatch, IWebBrowser, IWebBrowserApp);
#[cfg(feature = "oaidl")]
impl IWebBrowser2 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Navigate2(&self, url: *const super::oaidl::VARIANT, flags: Option<*const super::oaidl::VARIANT>, targetframename: Option<*const super::oaidl::VARIANT>, postdata: Option<*const super::oaidl::VARIANT>, headers: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Navigate2)(windows_core::Interface::as_raw(self), core::mem::transmute(url), flags.unwrap_or(core::mem::zeroed()) as _, targetframename.unwrap_or(core::mem::zeroed()) as _, postdata.unwrap_or(core::mem::zeroed()) as _, headers.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "docobj")]
    pub unsafe fn QueryStatusWB(&self, cmdid: super::docobj::OLECMDID) -> windows_core::Result<super::docobj::OLECMDF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryStatusWB)(windows_core::Interface::as_raw(self), cmdid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "docobj", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExecWB(&self, cmdid: super::docobj::OLECMDID, cmdexecopt: super::docobj::OLECMDEXECOPT, pvain: Option<*const super::oaidl::VARIANT>, pvaout: Option<*mut super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExecWB)(windows_core::Interface::as_raw(self), cmdid, cmdexecopt, pvain.unwrap_or(core::mem::zeroed()) as _, pvaout.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ShowBrowserBar(&self, pvaclsid: *const super::oaidl::VARIANT, pvarshow: Option<*const super::oaidl::VARIANT>, pvarsize: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowBrowserBar)(windows_core::Interface::as_raw(self), core::mem::transmute(pvaclsid), pvarshow.unwrap_or(core::mem::zeroed()) as _, pvarsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "ocidl")]
    pub unsafe fn ReadyState(&self) -> windows_core::Result<super::ocidl::READYSTATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Offline(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetOffline(&self, boffline: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOffline)(windows_core::Interface::as_raw(self), boffline) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Silent(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Silent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetSilent(&self, bsilent: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSilent)(windows_core::Interface::as_raw(self), bsilent) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RegisterAsBrowser(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterAsBrowser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetRegisterAsBrowser(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRegisterAsBrowser)(windows_core::Interface::as_raw(self), bregister) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RegisterAsDropTarget(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterAsDropTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetRegisterAsDropTarget(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRegisterAsDropTarget)(windows_core::Interface::as_raw(self), bregister) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TheaterMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TheaterMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetTheaterMode(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTheaterMode)(windows_core::Interface::as_raw(self), bregister) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AddressBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddressBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAddressBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressBar)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Resizable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resizable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetResizable(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResizable)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowser2_Vtbl {
    pub base__: IWebBrowserApp_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Navigate2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Navigate2: usize,
    #[cfg(feature = "docobj")]
    pub QueryStatusWB: unsafe extern "system" fn(*mut core::ffi::c_void, super::docobj::OLECMDID, *mut super::docobj::OLECMDF) -> windows_core::HRESULT,
    #[cfg(not(feature = "docobj"))]
    QueryStatusWB: usize,
    #[cfg(all(feature = "docobj", feature = "wtypes", feature = "wtypesbase"))]
    pub ExecWB: unsafe extern "system" fn(*mut core::ffi::c_void, super::docobj::OLECMDID, super::docobj::OLECMDEXECOPT, *const super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "docobj", feature = "wtypes", feature = "wtypesbase")))]
    ExecWB: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ShowBrowserBar: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ShowBrowserBar: usize,
    #[cfg(feature = "ocidl")]
    pub ReadyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ocidl::READYSTATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "ocidl"))]
    ReadyState: usize,
    #[cfg(feature = "wtypes")]
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Offline: usize,
    #[cfg(feature = "wtypes")]
    pub SetOffline: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetOffline: usize,
    #[cfg(feature = "wtypes")]
    pub Silent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Silent: usize,
    #[cfg(feature = "wtypes")]
    pub SetSilent: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetSilent: usize,
    #[cfg(feature = "wtypes")]
    pub RegisterAsBrowser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RegisterAsBrowser: usize,
    #[cfg(feature = "wtypes")]
    pub SetRegisterAsBrowser: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetRegisterAsBrowser: usize,
    #[cfg(feature = "wtypes")]
    pub RegisterAsDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RegisterAsDropTarget: usize,
    #[cfg(feature = "wtypes")]
    pub SetRegisterAsDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetRegisterAsDropTarget: usize,
    #[cfg(feature = "wtypes")]
    pub TheaterMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TheaterMode: usize,
    #[cfg(feature = "wtypes")]
    pub SetTheaterMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetTheaterMode: usize,
    #[cfg(feature = "wtypes")]
    pub AddressBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AddressBar: usize,
    #[cfg(feature = "wtypes")]
    pub SetAddressBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAddressBar: usize,
    #[cfg(feature = "wtypes")]
    pub Resizable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Resizable: usize,
    #[cfg(feature = "wtypes")]
    pub SetResizable: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetResizable: usize,
}
#[cfg(all(feature = "basetsd", feature = "docobj", feature = "oaidl", feature = "ocidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebBrowser2_Impl: IWebBrowserApp_Impl {
    fn Navigate2(&self, url: *const super::oaidl::VARIANT, flags: *const super::oaidl::VARIANT, targetframename: *const super::oaidl::VARIANT, postdata: *const super::oaidl::VARIANT, headers: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn QueryStatusWB(&self, cmdid: super::docobj::OLECMDID) -> windows_core::Result<super::docobj::OLECMDF>;
    fn ExecWB(&self, cmdid: super::docobj::OLECMDID, cmdexecopt: super::docobj::OLECMDEXECOPT, pvain: *const super::oaidl::VARIANT, pvaout: *mut super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ShowBrowserBar(&self, pvaclsid: *const super::oaidl::VARIANT, pvarshow: *const super::oaidl::VARIANT, pvarsize: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ReadyState(&self) -> windows_core::Result<super::ocidl::READYSTATE>;
    fn Offline(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetOffline(&self, boffline: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Silent(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetSilent(&self, bsilent: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RegisterAsBrowser(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetRegisterAsBrowser(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RegisterAsDropTarget(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetRegisterAsDropTarget(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TheaterMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetTheaterMode(&self, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AddressBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAddressBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Resizable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetResizable(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "docobj", feature = "oaidl", feature = "ocidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebBrowser2_Vtbl {
    pub const fn new<Identity: IWebBrowser2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate2<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: *const super::oaidl::VARIANT, flags: *const super::oaidl::VARIANT, targetframename: *const super::oaidl::VARIANT, postdata: *const super::oaidl::VARIANT, headers: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::Navigate2(this, core::mem::transmute_copy(&url), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&targetframename), core::mem::transmute_copy(&postdata), core::mem::transmute_copy(&headers)).into()
            }
        }
        unsafe extern "system" fn QueryStatusWB<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmdid: super::docobj::OLECMDID, pcmdf: *mut super::docobj::OLECMDF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::QueryStatusWB(this, core::mem::transmute_copy(&cmdid)) {
                    Ok(ok__) => {
                        pcmdf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecWB<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmdid: super::docobj::OLECMDID, cmdexecopt: super::docobj::OLECMDEXECOPT, pvain: *const super::oaidl::VARIANT, pvaout: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::ExecWB(this, core::mem::transmute_copy(&cmdid), core::mem::transmute_copy(&cmdexecopt), core::mem::transmute_copy(&pvain), core::mem::transmute_copy(&pvaout)).into()
            }
        }
        unsafe extern "system" fn ShowBrowserBar<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaclsid: *const super::oaidl::VARIANT, pvarshow: *const super::oaidl::VARIANT, pvarsize: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::ShowBrowserBar(this, core::mem::transmute_copy(&pvaclsid), core::mem::transmute_copy(&pvarshow), core::mem::transmute_copy(&pvarsize)).into()
            }
        }
        unsafe extern "system" fn ReadyState<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreadystate: *mut super::ocidl::READYSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::ReadyState(this) {
                    Ok(ok__) => {
                        plreadystate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Offline<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboffline: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::Offline(this) {
                    Ok(ok__) => {
                        pboffline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOffline<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boffline: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetOffline(this, core::mem::transmute_copy(&boffline)).into()
            }
        }
        unsafe extern "system" fn Silent<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsilent: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::Silent(this) {
                    Ok(ok__) => {
                        pbsilent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSilent<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsilent: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetSilent(this, core::mem::transmute_copy(&bsilent)).into()
            }
        }
        unsafe extern "system" fn RegisterAsBrowser<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregister: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::RegisterAsBrowser(this) {
                    Ok(ok__) => {
                        pbregister.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegisterAsBrowser<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetRegisterAsBrowser(this, core::mem::transmute_copy(&bregister)).into()
            }
        }
        unsafe extern "system" fn RegisterAsDropTarget<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregister: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::RegisterAsDropTarget(this) {
                    Ok(ok__) => {
                        pbregister.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegisterAsDropTarget<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetRegisterAsDropTarget(this, core::mem::transmute_copy(&bregister)).into()
            }
        }
        unsafe extern "system" fn TheaterMode<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregister: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::TheaterMode(this) {
                    Ok(ok__) => {
                        pbregister.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTheaterMode<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bregister: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetTheaterMode(this, core::mem::transmute_copy(&bregister)).into()
            }
        }
        unsafe extern "system" fn AddressBar<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::AddressBar(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAddressBar<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetAddressBar(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Resizable<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowser2_Impl::Resizable(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResizable<Identity: IWebBrowser2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowser2_Impl::SetResizable(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IWebBrowserApp_Vtbl::new::<Identity, OFFSET>(),
            Navigate2: Navigate2::<Identity, OFFSET>,
            QueryStatusWB: QueryStatusWB::<Identity, OFFSET>,
            ExecWB: ExecWB::<Identity, OFFSET>,
            ShowBrowserBar: ShowBrowserBar::<Identity, OFFSET>,
            ReadyState: ReadyState::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            SetOffline: SetOffline::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            RegisterAsBrowser: RegisterAsBrowser::<Identity, OFFSET>,
            SetRegisterAsBrowser: SetRegisterAsBrowser::<Identity, OFFSET>,
            RegisterAsDropTarget: RegisterAsDropTarget::<Identity, OFFSET>,
            SetRegisterAsDropTarget: SetRegisterAsDropTarget::<Identity, OFFSET>,
            TheaterMode: TheaterMode::<Identity, OFFSET>,
            SetTheaterMode: SetTheaterMode::<Identity, OFFSET>,
            AddressBar: AddressBar::<Identity, OFFSET>,
            SetAddressBar: SetAddressBar::<Identity, OFFSET>,
            Resizable: Resizable::<Identity, OFFSET>,
            SetResizable: SetResizable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowser2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWebBrowser as windows_core::Interface>::IID || iid == &<IWebBrowserApp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "docobj", feature = "oaidl", feature = "ocidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebBrowser2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebBrowserApp, IWebBrowserApp_Vtbl, 0x0002df05_0000_0000_c000_000000000046);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebBrowserApp {
    type Target = IWebBrowser;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebBrowserApp, windows_core::IUnknown, super::oaidl::IDispatch, IWebBrowser);
#[cfg(feature = "oaidl")]
impl IWebBrowserApp {
    pub unsafe fn Quit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Quit)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ClientToWindow(&self, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClientToWindow)(windows_core::Interface::as_raw(self), pcx as _, pcy as _) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutProperty(&self, property: &windows_core::BSTR, vtvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), core::mem::transmute_copy(vtvalue)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperty(&self, property: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "basetsd")]
    pub unsafe fn HWND(&self) -> windows_core::Result<super::basetsd::SHANDLE_PTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HWND)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FullName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Visible(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetVisible(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVisible)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn StatusBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetStatusBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatusBar)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn StatusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStatusText(&self, statustext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatusText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(statustext)) }
    }
    pub unsafe fn ToolBar(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToolBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetToolBar(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetToolBar)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MenuBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MenuBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetMenuBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMenuBar)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn FullScreen(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFullScreen(&self, bfullscreen: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullScreen)(windows_core::Interface::as_raw(self), bfullscreen) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserApp_Vtbl {
    pub base__: IWebBrowser_Vtbl,
    pub Quit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientToWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PutProperty: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetProperty: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "basetsd")]
    pub HWND: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::basetsd::SHANDLE_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    HWND: usize,
    pub FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Visible: usize,
    #[cfg(feature = "wtypes")]
    pub SetVisible: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetVisible: usize,
    #[cfg(feature = "wtypes")]
    pub StatusBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    StatusBar: usize,
    #[cfg(feature = "wtypes")]
    pub SetStatusBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetStatusBar: usize,
    pub StatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ToolBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetToolBar: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub MenuBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MenuBar: usize,
    #[cfg(feature = "wtypes")]
    pub SetMenuBar: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetMenuBar: usize,
    #[cfg(feature = "wtypes")]
    pub FullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    FullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub SetFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFullScreen: usize,
}
#[cfg(all(feature = "basetsd", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebBrowserApp_Impl: IWebBrowser_Impl {
    fn Quit(&self) -> windows_core::Result<()>;
    fn ClientToWindow(&self, pcx: *mut i32, pcy: *mut i32) -> windows_core::Result<()>;
    fn PutProperty(&self, property: &windows_core::BSTR, vtvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetProperty(&self, property: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HWND(&self) -> windows_core::Result<super::basetsd::SHANDLE_PTR>;
    fn FullName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Visible(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetVisible(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn StatusBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetStatusBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn StatusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStatusText(&self, statustext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ToolBar(&self) -> windows_core::Result<i32>;
    fn SetToolBar(&self, value: i32) -> windows_core::Result<()>;
    fn MenuBar(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetMenuBar(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FullScreen(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetFullScreen(&self, bfullscreen: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebBrowserApp_Vtbl {
    pub const fn new<Identity: IWebBrowserApp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Quit<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::Quit(this).into()
            }
        }
        unsafe extern "system" fn ClientToWindow<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::ClientToWindow(this, core::mem::transmute_copy(&pcx), core::mem::transmute_copy(&pcy)).into()
            }
        }
        unsafe extern "system" fn PutProperty<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, vtvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::PutProperty(this, core::mem::transmute(&property), core::mem::transmute(&vtvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, pvtvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::GetProperty(this, core::mem::transmute(&property)) {
                    Ok(ok__) => {
                        pvtvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HWND<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::basetsd::SHANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::HWND(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FullName<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::FullName(this) {
                    Ok(ok__) => {
                        fullname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::Path(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Visible<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::Visible(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVisible<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetVisible(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StatusBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::StatusBar(this) {
                    Ok(ok__) => {
                        pbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStatusBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetStatusBar(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StatusText<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::StatusText(this) {
                    Ok(ok__) => {
                        statustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStatusText<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statustext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetStatusText(this, core::mem::transmute(&statustext)).into()
            }
        }
        unsafe extern "system" fn ToolBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::ToolBar(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetToolBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetToolBar(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn MenuBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::MenuBar(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMenuBar<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetMenuBar(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn FullScreen<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserApp_Impl::FullScreen(this) {
                    Ok(ok__) => {
                        pbfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullScreen<Identity: IWebBrowserApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserApp_Impl::SetFullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
            }
        }
        Self {
            base__: IWebBrowser_Vtbl::new::<Identity, OFFSET>(),
            Quit: Quit::<Identity, OFFSET>,
            ClientToWindow: ClientToWindow::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            HWND: HWND::<Identity, OFFSET>,
            FullName: FullName::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            SetVisible: SetVisible::<Identity, OFFSET>,
            StatusBar: StatusBar::<Identity, OFFSET>,
            SetStatusBar: SetStatusBar::<Identity, OFFSET>,
            StatusText: StatusText::<Identity, OFFSET>,
            SetStatusText: SetStatusText::<Identity, OFFSET>,
            ToolBar: ToolBar::<Identity, OFFSET>,
            SetToolBar: SetToolBar::<Identity, OFFSET>,
            MenuBar: MenuBar::<Identity, OFFSET>,
            SetMenuBar: SetMenuBar::<Identity, OFFSET>,
            FullScreen: FullScreen::<Identity, OFFSET>,
            SetFullScreen: SetFullScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowserApp as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWebBrowser as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebBrowserApp {}
pub const InternetExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x0002df01_0000_0000_c000_000000000046);
pub const InternetExplorerMedium: windows_core::GUID = windows_core::GUID::from_u128(0xd5e8041d_920f_45e9_b8fb_b1deb82c6e5e);
pub type NewProcessCauseConstants = i32;
pub const ProtectedModeRedirect: NewProcessCauseConstants = 1;
pub const REFRESH_COMPLETELY: RefreshConstants = 3;
pub const REFRESH_IFEXPIRED: RefreshConstants = 1;
pub const REFRESH_NORMAL: RefreshConstants = 0;
pub type RefreshConstants = i32;
pub const SWC_3RDPARTY: ShellWindowTypeConstants = 2;
pub const SWC_BROWSER: ShellWindowTypeConstants = 1;
pub const SWC_CALLBACK: ShellWindowTypeConstants = 4;
pub const SWC_DESKTOP: ShellWindowTypeConstants = 8;
pub const SWC_EXPLORER: ShellWindowTypeConstants = 0;
pub const SWFO_COOKIEPASSED: ShellWindowFindWindowOptions = 4;
pub const SWFO_INCLUDEPENDING: ShellWindowFindWindowOptions = 2;
pub const SWFO_NEEDDISPATCH: ShellWindowFindWindowOptions = 1;
pub type SecureLockIconConstants = i32;
pub const ShellBrowserWindow: windows_core::GUID = windows_core::GUID::from_u128(0xc08afd90_f2a1_11d1_8455_00a0c91f3880);
pub const ShellNameSpace: windows_core::GUID = windows_core::GUID::from_u128(0x55136805_b2de_11d1_b9f2_00a0c98bc547);
pub const ShellUIHelper: windows_core::GUID = windows_core::GUID::from_u128(0x64ab4bb7_111e_11d1_8f79_00c04fc2fbe1);
pub type ShellWindowFindWindowOptions = i32;
pub type ShellWindowTypeConstants = i32;
pub const ShellWindows: windows_core::GUID = windows_core::GUID::from_u128(0x9ba05972_f6a8_11cf_a442_00a0c90a8f39);
pub const WebBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x8856f961_340a_11d0_a96b_00c04fd705a2);
pub const WebBrowser_V1: windows_core::GUID = windows_core::GUID::from_u128(0xeab22ac3_30c1_11cf_a7eb_0000c05bae0b);
pub const navAllowAutosearch: BrowserNavConstants = 16;
pub const navBlockRedirectsXDomain: BrowserNavConstants = 32768;
pub const navBrowserBar: BrowserNavConstants = 32;
pub const navCheckDontShowHVSINeedHost: i32 = -2147483648;
pub const navCheckDontShowNeedHVSI: u32 = 1073741824;
pub const navCheckDontShowNeedIE: u32 = 536870912;
pub const navDeferUnload: BrowserNavConstants = 262144;
pub const navDisableDownloadSave: u32 = 67108864;
pub const navEnforceRestricted: BrowserNavConstants = 128;
pub const navHomepageNavigate: BrowserNavConstants = 8388608;
pub const navHostNavigation: BrowserNavConstants = 33554432;
pub const navHyperlink: BrowserNavConstants = 64;
pub const navKeepWordWheelText: BrowserNavConstants = 8192;
pub const navNewWindowsManaged: BrowserNavConstants = 256;
pub const navNoHistory: BrowserNavConstants = 2;
pub const navNoReadFromCache: BrowserNavConstants = 4;
pub const navNoWriteToCache: BrowserNavConstants = 8;
pub const navOpenInBackgroundTab: BrowserNavConstants = 4096;
pub const navOpenInNewTab: BrowserNavConstants = 2048;
pub const navOpenInNewWindow: BrowserNavConstants = 1;
pub const navOpenNewForegroundTab: BrowserNavConstants = 65536;
pub const navRefresh: BrowserNavConstants = 16777216;
pub const navReserved1: BrowserNavConstants = 4194304;
pub const navReserved2: BrowserNavConstants = 67108864;
pub const navReserved3: BrowserNavConstants = 134217728;
pub const navReserved4: BrowserNavConstants = 268435456;
pub const navReserved5: BrowserNavConstants = 536870912;
pub const navReserved6: BrowserNavConstants = 1073741824;
pub const navReserved7: BrowserNavConstants = -2147483648;
pub const navServerRedirectedVtabSwitch: u32 = 134217728;
pub const navSpeculative: BrowserNavConstants = 524288;
pub const navSuggestNewTab: BrowserNavConstants = 2097152;
pub const navSuggestNewWindow: BrowserNavConstants = 1048576;
pub const navTravelLogScreenshot: BrowserNavConstants = 131072;
pub const navTrustedForActiveX: BrowserNavConstants = 1024;
pub const navUntrustedForDownload: BrowserNavConstants = 512;
pub const navUserInitiatedAction: u32 = 4194304;
pub const navVirtualTab: BrowserNavConstants = 16384;
pub const navVtabSwitchNotUserInitiatedAction: u32 = 268435456;
pub const secureLockIconMixed: SecureLockIconConstants = 1;
pub const secureLockIconSecure128Bit: SecureLockIconConstants = 6;
pub const secureLockIconSecure40Bit: SecureLockIconConstants = 3;
pub const secureLockIconSecure56Bit: SecureLockIconConstants = 4;
pub const secureLockIconSecureFortezza: SecureLockIconConstants = 5;
pub const secureLockIconSecureUnknownBits: SecureLockIconConstants = 2;
pub const secureLockIconUnsecure: SecureLockIconConstants = 0;
