windows_core::imp::define_interface!(IEnumTfLangBarItems, IEnumTfLangBarItems_Vtbl, 0x583f34d0_de25_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfLangBarItems, windows_core::IUnknown);
impl IEnumTfLangBarItems {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, ulcount: super::ULONG, ppitem: *mut Option<ITfLangBarItem>, pcfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppitem), pcfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, ulcount: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLangBarItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumTfLangBarItems_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn Next(&self, ulcount: super::ULONG, ppitem: *mut Option<ITfLangBarItem>, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumTfLangBarItems_Vtbl {
    pub const fn new<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfLangBarItems_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, ppitem: *mut *mut core::ffi::c_void, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLangBarItems_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLangBarItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfLangBarItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLangBarItems_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumTfLangBarItems as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumTfLangBarItems {}
windows_core::imp::define_interface!(ITfLangBarEventSink, ITfLangBarEventSink_Vtbl, 0x18a4e900_e0ae_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarEventSink, windows_core::IUnknown);
impl ITfLangBarEventSink {
    #[cfg(feature = "shared")]
    pub unsafe fn OnSetFocus(&self, dwthreadid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), dwthreadid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnThreadTerminate(&self, dwthreadid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnThreadTerminate)(windows_core::Interface::as_raw(self), dwthreadid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnThreadItemChange(&self, dwthreadid: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnThreadItemChange)(windows_core::Interface::as_raw(self), dwthreadid) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn OnModalInput(&self, dwthreadid: super::DWORD, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnModalInput)(windows_core::Interface::as_raw(self), dwthreadid, umsg, wparam, lparam) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ShowFloating(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowFloating)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemFloatingRect)(windows_core::Interface::as_raw(self), dwthreadid, rguid, prc as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnSetFocus: usize,
    #[cfg(feature = "shared")]
    pub OnThreadTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnThreadTerminate: usize,
    #[cfg(feature = "shared")]
    pub OnThreadItemChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnThreadItemChange: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub OnModalInput: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::UINT, super::WPARAM, super::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    OnModalInput: usize,
    #[cfg(feature = "shared")]
    pub ShowFloating: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ShowFloating: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetItemFloatingRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetItemFloatingRect: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub trait ITfLangBarEventSink_Impl: windows_core::IUnknownImpl {
    fn OnSetFocus(&self, dwthreadid: super::DWORD) -> windows_core::Result<()>;
    fn OnThreadTerminate(&self, dwthreadid: super::DWORD) -> windows_core::Result<()>;
    fn OnThreadItemChange(&self, dwthreadid: super::DWORD) -> windows_core::Result<()>;
    fn OnModalInput(&self, dwthreadid: super::DWORD, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl ITfLangBarEventSink_Vtbl {
    pub const fn new<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetFocus<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&dwthreadid)).into()
            }
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&dwthreadid)).into()
            }
        }
        unsafe extern "system" fn OnThreadItemChange<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::OnThreadItemChange(this, core::mem::transmute_copy(&dwthreadid)).into()
            }
        }
        unsafe extern "system" fn OnModalInput<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::OnModalInput(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn ShowFloating<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ITfLangBarEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarEventSink_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&prc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Identity, OFFSET>,
            OnModalInput: OnModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ITfLangBarEventSink {}
windows_core::imp::define_interface!(ITfLangBarItem, ITfLangBarItem_Vtbl, 0x73540d69_edeb_4ee9_96c9_23aa30b25916);
windows_core::imp::interface_hierarchy!(ITfLangBarItem, windows_core::IUnknown);
impl ITfLangBarItem {
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), pinfo as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatus(&self, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pdwstatus as _) }
    }
    pub unsafe fn Show(&self, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    pub unsafe fn GetTooltipString(&self, pbstrtooltip: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTooltipString)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtooltip)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "winnt")))]
    GetInfo: usize,
    #[cfg(feature = "shared")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatus: usize,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTooltipString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
pub trait ITfLangBarItem_Impl: windows_core::IUnknownImpl {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::Result<()>;
    fn GetStatus(&self, pdwstatus: *mut super::DWORD) -> windows_core::Result<()>;
    fn Show(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetTooltipString(&self, pbstrtooltip: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
impl ITfLangBarItem_Vtbl {
    pub const fn new<Identity: ITfLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfo<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItem_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItem_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn Show<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItem_Impl::Show(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn GetTooltipString<Identity: ITfLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtooltip: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItem_Impl::GetTooltipString(this, core::mem::transmute_copy(&pbstrtooltip)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            GetTooltipString: GetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItem {}
windows_core::imp::define_interface!(ITfLangBarItemBalloon, ITfLangBarItemBalloon_Vtbl, 0x01c2d285_d3c7_4b7b_b5b5_d97411d0c283);
impl core::ops::Deref for ITfLangBarItemBalloon {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBalloon, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBalloon {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, pt, prcarea) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, psz as _) }
    }
    pub unsafe fn GetBalloonInfo(&self, pinfo: *mut TF_LBBALLOONINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBalloonInfo)(windows_core::Interface::as_raw(self), pinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBalloon_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::POINT, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnClick: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SIZE, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetPreferredSize: usize,
    pub GetBalloonInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_LBBALLOONINFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfLangBarItemBalloon_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::POINT, prcarea: *const super::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::Result<()>;
    fn GetBalloonInfo(&self, pinfo: *mut TF_LBBALLOONINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfLangBarItemBalloon_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBalloon_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
            }
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBalloon_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault), core::mem::transmute_copy(&psz)).into()
            }
        }
        unsafe extern "system" fn GetBalloonInfo<Identity: ITfLangBarItemBalloon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBalloon_Impl::GetBalloonInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBalloon as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItemBalloon {}
windows_core::imp::define_interface!(ITfLangBarItemBitmap, ITfLangBarItemBitmap_Vtbl, 0x73830352_d722_4179_ada5_f045c98df355);
impl core::ops::Deref for ITfLangBarItemBitmap {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBitmap, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBitmap {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, pt, prcarea) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, psz as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn DrawBitmap(&self, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bmwidth, bmheight, dwflags, phbmp as _, phbmpmask as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmap_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::POINT, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnClick: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SIZE, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetPreferredSize: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::DWORD, *mut super::HBITMAP, *mut super::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    DrawBitmap: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfLangBarItemBitmap_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::POINT, prcarea: *const super::RECT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::Result<()>;
    fn DrawBitmap(&self, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfLangBarItemBitmap_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmap_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
            }
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmap_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault), core::mem::transmute_copy(&psz)).into()
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ITfLangBarItemBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmap_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmap as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItemBitmap {}
windows_core::imp::define_interface!(ITfLangBarItemBitmapButton, ITfLangBarItemBitmapButton_Vtbl, 0xa26a0525_3fae_4fa0_89ee_88a964f9f1b5);
impl core::ops::Deref for ITfLangBarItemBitmapButton {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemBitmapButton, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemBitmapButton {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, pt, prcarea) }
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfMenu>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredSize)(windows_core::Interface::as_raw(self), pszdefault, psz as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn DrawBitmap(&self, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bmwidth, bmheight, dwflags, phbmp as _, phbmpmask as _) }
    }
    pub unsafe fn GetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmapButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::POINT, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnMenuSelect: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetPreferredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SIZE, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetPreferredSize: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::DWORD, *mut super::HBITMAP, *mut super::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    DrawBitmap: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfLangBarItemBitmapButton_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::POINT, prcarea: *const super::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: windows_core::Ref<ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::Result<()>;
    fn GetPreferredSize(&self, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::Result<()>;
    fn DrawBitmap(&self, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::Result<()>;
    fn GetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfLangBarItemBitmapButton_Vtbl {
    pub const fn new<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
            }
        }
        unsafe extern "system" fn InitMenu<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::InitMenu(this, core::mem::transmute_copy(&pmenu)).into()
            }
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
            }
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefault: *const super::SIZE, psz: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::GetPreferredSize(this, core::mem::transmute_copy(&pszdefault), core::mem::transmute_copy(&psz)).into()
            }
        }
        unsafe extern "system" fn DrawBitmap<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmwidth: super::LONG, bmheight: super::LONG, dwflags: super::DWORD, phbmp: *mut super::HBITMAP, phbmpmask: *mut super::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::DrawBitmap(this, core::mem::transmute_copy(&bmwidth), core::mem::transmute_copy(&bmheight), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&phbmp), core::mem::transmute_copy(&phbmpmask)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITfLangBarItemBitmapButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemBitmapButton_Impl::GetText(this, core::mem::transmute_copy(&pbstrtext)).into()
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemBitmapButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItemBitmapButton {}
windows_core::imp::define_interface!(ITfLangBarItemButton, ITfLangBarItemButton_Vtbl, 0x28c7f1d0_de25_11d2_afdd_00105a2799b5);
impl core::ops::Deref for ITfLangBarItemButton {
    type Target = ITfLangBarItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfLangBarItemButton, windows_core::IUnknown, ITfLangBarItem);
impl ITfLangBarItemButton {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClick)(windows_core::Interface::as_raw(self), click, pt, prcarea) }
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfMenu>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetIcon(&self, phicon: *mut super::HICON) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), phicon as _) }
    }
    pub unsafe fn GetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub OnClick: unsafe extern "system" fn(*mut core::ffi::c_void, TfLBIClick, super::POINT, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnMenuSelect: usize,
    #[cfg(feature = "windef")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetIcon: usize,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfLangBarItemButton_Impl: ITfLangBarItem_Impl {
    fn OnClick(&self, click: TfLBIClick, pt: &super::POINT, prcarea: *const super::RECT) -> windows_core::Result<()>;
    fn InitMenu(&self, pmenu: windows_core::Ref<ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::Result<()>;
    fn GetIcon(&self, phicon: *mut super::HICON) -> windows_core::Result<()>;
    fn GetText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfLangBarItemButton_Vtbl {
    pub const fn new<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClick<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, click: TfLBIClick, pt: super::POINT, prcarea: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemButton_Impl::OnClick(this, core::mem::transmute_copy(&click), core::mem::transmute(&pt), core::mem::transmute_copy(&prcarea)).into()
            }
        }
        unsafe extern "system" fn InitMenu<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemButton_Impl::InitMenu(this, core::mem::transmute_copy(&pmenu)).into()
            }
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemButton_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
            }
        }
        unsafe extern "system" fn GetIcon<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phicon: *mut super::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemButton_Impl::GetIcon(this, core::mem::transmute_copy(&phicon)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: ITfLangBarItemButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemButton_Impl::GetText(this, core::mem::transmute_copy(&pbstrtext)).into()
            }
        }
        Self {
            base__: ITfLangBarItem_Vtbl::new::<Identity, OFFSET>(),
            OnClick: OnClick::<Identity, OFFSET>,
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemButton as windows_core::Interface>::IID || iid == &<ITfLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItemButton {}
windows_core::imp::define_interface!(ITfLangBarItemMgr, ITfLangBarItemMgr_Vtbl, 0xba468c55_9956_4fb1_a59d_52a7dd7cc6aa);
windows_core::imp::interface_hierarchy!(ITfLangBarItemMgr, windows_core::IUnknown);
impl ITfLangBarItemMgr {
    pub unsafe fn EnumItems(&self) -> windows_core::Result<IEnumTfLangBarItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfLangBarItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddItem<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfLangBarItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn RemoveItem<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfLangBarItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AdviseItemSink<P0>(&self, punk: P0, pdwcookie: *mut super::DWORD, rguiditem: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfLangBarItemSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseItemSink)(windows_core::Interface::as_raw(self), punk.param().abi(), pdwcookie as _, rguiditem) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnadviseItemSink(&self, dwcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseItemSink)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemFloatingRect)(windows_core::Interface::as_raw(self), dwthreadid, rguid, prc as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetItemsStatus(&self, ulcount: super::ULONG, prgguid: *const windows_core::GUID, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemsStatus)(windows_core::Interface::as_raw(self), ulcount, prgguid, pdwstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetItemNum(&self, pulcount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemNum)(windows_core::Interface::as_raw(self), pulcount as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetItems(&self, ulcount: super::ULONG, ppitem: *mut Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut super::DWORD, pcfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItems)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppitem), pinfo as _, pdwstatus as _, pcfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AdviseItemsSink(&self, ulcount: super::ULONG, ppunk: *const Option<ITfLangBarItemSink>, pguiditem: *const windows_core::GUID, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseItemsSink)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppunk), pguiditem, pdwcookie as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnadviseItemsSink(&self, pdwcookie: &[super::DWORD]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseItemsSink)(windows_core::Interface::as_raw(self), pdwcookie.len().try_into().unwrap(), pdwcookie.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AdviseItemSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AdviseItemSink: usize,
    #[cfg(feature = "shared")]
    pub UnadviseItemSink: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnadviseItemSink: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetItemFloatingRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetItemFloatingRect: usize,
    #[cfg(feature = "shared")]
    pub GetItemsStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const windows_core::GUID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetItemsStatus: usize,
    #[cfg(feature = "shared")]
    pub GetItemNum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetItemNum: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
    pub GetItems: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut TF_LANGBARITEMINFO, *mut super::DWORD, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "winnt")))]
    GetItems: usize,
    #[cfg(feature = "shared")]
    pub AdviseItemsSink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const *mut core::ffi::c_void, *const windows_core::GUID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AdviseItemsSink: usize,
    #[cfg(feature = "shared")]
    pub UnadviseItemsSink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnadviseItemsSink: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfLangBarItemMgr_Impl: windows_core::IUnknownImpl {
    fn EnumItems(&self) -> windows_core::Result<IEnumTfLangBarItems>;
    fn GetItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfLangBarItem>;
    fn AddItem(&self, punk: windows_core::Ref<ITfLangBarItem>) -> windows_core::Result<()>;
    fn RemoveItem(&self, punk: windows_core::Ref<ITfLangBarItem>) -> windows_core::Result<()>;
    fn AdviseItemSink(&self, punk: windows_core::Ref<ITfLangBarItemSink>, pdwcookie: *mut super::DWORD, rguiditem: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnadviseItemSink(&self, dwcookie: super::DWORD) -> windows_core::Result<()>;
    fn GetItemFloatingRect(&self, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::Result<()>;
    fn GetItemsStatus(&self, ulcount: super::ULONG, prgguid: *const windows_core::GUID, pdwstatus: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetItemNum(&self, pulcount: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetItems(&self, ulcount: super::ULONG, ppitem: *mut Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut super::DWORD, pcfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn AdviseItemsSink(&self, ulcount: super::ULONG, ppunk: *const Option<ITfLangBarItemSink>, pguiditem: *const windows_core::GUID, pdwcookie: *mut super::DWORD) -> windows_core::Result<()>;
    fn UnadviseItemsSink(&self, ulcount: super::ULONG, pdwcookie: *const super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfLangBarItemMgr_Vtbl {
    pub const fn new<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumItems<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfLangBarItemMgr_Impl::EnumItems(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfLangBarItemMgr_Impl::GetItem(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::AddItem(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::RemoveItem(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn AdviseItemSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pdwcookie: *mut super::DWORD, rguiditem: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::AdviseItemSink(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&rguiditem)).into()
            }
        }
        unsafe extern "system" fn UnadviseItemSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::UnadviseItemSink(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, rguid: *const windows_core::GUID, prc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::GetItemFloatingRect(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&prc)).into()
            }
        }
        unsafe extern "system" fn GetItemsStatus<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, prgguid: *const windows_core::GUID, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::GetItemsStatus(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&prgguid), core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn GetItemNum<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::GetItemNum(this, core::mem::transmute_copy(&pulcount)).into()
            }
        }
        unsafe extern "system" fn GetItems<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, ppitem: *mut *mut core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut super::DWORD, pcfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::GetItems(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppitem), core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn AdviseItemsSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, ppunk: *const *mut core::ffi::c_void, pguiditem: *const windows_core::GUID, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::AdviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppunk), core::mem::transmute_copy(&pguiditem), core::mem::transmute_copy(&pdwcookie)).into()
            }
        }
        unsafe extern "system" fn UnadviseItemsSink<Identity: ITfLangBarItemMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: super::ULONG, pdwcookie: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemMgr_Impl::UnadviseItemsSink(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pdwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            AdviseItemSink: AdviseItemSink::<Identity, OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Identity, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, OFFSET>,
            GetItemsStatus: GetItemsStatus::<Identity, OFFSET>,
            GetItemNum: GetItemNum::<Identity, OFFSET>,
            GetItems: GetItems::<Identity, OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Identity, OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemMgr as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfLangBarItemMgr {}
windows_core::imp::define_interface!(ITfLangBarItemSink, ITfLangBarItemSink_Vtbl, 0x57dbe1a0_de25_11d2_afdd_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarItemSink, windows_core::IUnknown);
impl ITfLangBarItemSink {
    #[cfg(feature = "shared")]
    pub unsafe fn OnUpdate(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnUpdate: usize,
}
#[cfg(feature = "shared")]
pub trait ITfLangBarItemSink_Impl: windows_core::IUnknownImpl {
    fn OnUpdate(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITfLangBarItemSink_Vtbl {
    pub const fn new<Identity: ITfLangBarItemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdate<Identity: ITfLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarItemSink_Impl::OnUpdate(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdate: OnUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarItemSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITfLangBarItemSink {}
windows_core::imp::define_interface!(ITfLangBarMgr, ITfLangBarMgr_Vtbl, 0x87955690_e627_11d2_8ddb_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfLangBarMgr, windows_core::IUnknown);
impl ITfLangBarMgr {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn AdviseEventSink<P0>(&self, psink: P0, hwnd: super::HWND, dwflags: super::DWORD, pdwcookie: *const super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfLangBarEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseEventSink)(windows_core::Interface::as_raw(self), psink.param().abi(), hwnd, dwflags, pdwcookie) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UnadviseEventSink(&self, dwcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseEventSink)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetThreadMarshalInterface(&self, dwthreadid: super::DWORD, dwtype: super::DWORD, riid: *const super::IID, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetThreadMarshalInterface)(windows_core::Interface::as_raw(self), dwthreadid, dwtype, riid, core::mem::transmute(ppunk)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetThreadLangBarItemMgr(&self, dwthreadid: super::DWORD, pplbi: *mut Option<ITfLangBarItemMgr>, pdwthreadid: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetThreadLangBarItemMgr)(windows_core::Interface::as_raw(self), dwthreadid, core::mem::transmute(pplbi), pdwthreadid as _) }
    }
    #[cfg(all(feature = "msctf", feature = "shared"))]
    pub unsafe fn GetInputProcessorProfiles(&self, dwthreadid: super::DWORD, ppaip: *mut Option<super::ITfInputProcessorProfiles>, pdwthreadid: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputProcessorProfiles)(windows_core::Interface::as_raw(self), dwthreadid, core::mem::transmute(ppaip), pdwthreadid as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RestoreLastFocus(&self, pdwthreadid: *mut super::DWORD, fprev: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreLastFocus)(windows_core::Interface::as_raw(self), pdwthreadid as _, fprev.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetModalInput<P0>(&self, psink: P0, dwthreadid: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfLangBarEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetModalInput)(windows_core::Interface::as_raw(self), psink.param().abi(), dwthreadid, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ShowFloating(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowFloating)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetShowFloatingStatus(&self, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetShowFloatingStatus)(windows_core::Interface::as_raw(self), pdwflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub AdviseEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HWND, super::DWORD, *const super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    AdviseEventSink: usize,
    #[cfg(feature = "shared")]
    pub UnadviseEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UnadviseEventSink: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetThreadMarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetThreadMarshalInterface: usize,
    #[cfg(feature = "shared")]
    pub GetThreadLangBarItemMgr: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetThreadLangBarItemMgr: usize,
    #[cfg(all(feature = "msctf", feature = "shared"))]
    pub GetInputProcessorProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "msctf", feature = "shared")))]
    GetInputProcessorProfiles: usize,
    #[cfg(feature = "shared")]
    pub RestoreLastFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RestoreLastFocus: usize,
    #[cfg(feature = "shared")]
    pub SetModalInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetModalInput: usize,
    #[cfg(feature = "shared")]
    pub ShowFloating: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ShowFloating: usize,
    #[cfg(feature = "shared")]
    pub GetShowFloatingStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetShowFloatingStatus: usize,
}
#[cfg(all(feature = "guiddef", feature = "msctf", feature = "shared", feature = "windef"))]
pub trait ITfLangBarMgr_Impl: windows_core::IUnknownImpl {
    fn AdviseEventSink(&self, psink: windows_core::Ref<ITfLangBarEventSink>, hwnd: super::HWND, dwflags: super::DWORD, pdwcookie: *const super::DWORD) -> windows_core::Result<()>;
    fn UnadviseEventSink(&self, dwcookie: super::DWORD) -> windows_core::Result<()>;
    fn GetThreadMarshalInterface(&self, dwthreadid: super::DWORD, dwtype: super::DWORD, riid: *const super::IID, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetThreadLangBarItemMgr(&self, dwthreadid: super::DWORD, pplbi: windows_core::OutRef<ITfLangBarItemMgr>, pdwthreadid: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetInputProcessorProfiles(&self, dwthreadid: super::DWORD, ppaip: windows_core::OutRef<super::ITfInputProcessorProfiles>, pdwthreadid: *mut super::DWORD) -> windows_core::Result<()>;
    fn RestoreLastFocus(&self, pdwthreadid: *mut super::DWORD, fprev: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetModalInput(&self, psink: windows_core::Ref<ITfLangBarEventSink>, dwthreadid: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn ShowFloating(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetShowFloatingStatus(&self, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "msctf", feature = "shared", feature = "windef"))]
impl ITfLangBarMgr_Vtbl {
    pub const fn new<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseEventSink<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, hwnd: super::HWND, dwflags: super::DWORD, pdwcookie: *const super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::AdviseEventSink(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwcookie)).into()
            }
        }
        unsafe extern "system" fn UnadviseEventSink<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::UnadviseEventSink(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, dwtype: super::DWORD, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::GetThreadMarshalInterface(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, pplbi: *mut *mut core::ffi::c_void, pdwthreadid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::GetThreadLangBarItemMgr(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&pplbi), core::mem::transmute_copy(&pdwthreadid)).into()
            }
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: super::DWORD, ppaip: *mut *mut core::ffi::c_void, pdwthreadid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::GetInputProcessorProfiles(this, core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&ppaip), core::mem::transmute_copy(&pdwthreadid)).into()
            }
        }
        unsafe extern "system" fn RestoreLastFocus<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadid: *mut super::DWORD, fprev: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::RestoreLastFocus(this, core::mem::transmute_copy(&pdwthreadid), core::mem::transmute_copy(&fprev)).into()
            }
        }
        unsafe extern "system" fn SetModalInput<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, dwthreadid: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::SetModalInput(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&dwthreadid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ShowFloating<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::ShowFloating(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetShowFloatingStatus<Identity: ITfLangBarMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLangBarMgr_Impl::GetShowFloatingStatus(this, core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventSink: AdviseEventSink::<Identity, OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Identity, OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Identity, OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Identity, OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Identity, OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Identity, OFFSET>,
            SetModalInput: SetModalInput::<Identity, OFFSET>,
            ShowFloating: ShowFloating::<Identity, OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLangBarMgr as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "msctf", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ITfLangBarMgr {}
windows_core::imp::define_interface!(ITfMenu, ITfMenu_Vtbl, 0x6f8a98e4_aaa0_4f15_8c5b_07e0df0a3dd8);
windows_core::imp::interface_hierarchy!(ITfMenu, windows_core::IUnknown);
impl ITfMenu {
    #[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
    pub unsafe fn AddMenuItem(&self, uid: super::UINT, dwflags: super::DWORD, hbmp: super::HBITMAP, hbmpmask: super::HBITMAP, pch: Option<&[super::WCHAR]>, ppmenu: Option<*mut Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddMenuItem)(windows_core::Interface::as_raw(self), uid, dwflags, hbmp, hbmpmask, pch.map_or(core::ptr::null(), |slice| slice.as_ptr()), pch.map_or(0, |slice| slice.len().try_into().unwrap()), ppmenu.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
    pub AddMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::DWORD, super::HBITMAP, super::HBITMAP, *const super::WCHAR, super::ULONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "winnt")))]
    AddMenuItem: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfMenu_Impl: windows_core::IUnknownImpl {
    fn AddMenuItem(&self, uid: super::UINT, dwflags: super::DWORD, hbmp: super::HBITMAP, hbmpmask: super::HBITMAP, pch: *const super::WCHAR, cch: super::ULONG, ppmenu: *mut Option<ITfMenu>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfMenu_Vtbl {
    pub const fn new<Identity: ITfMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddMenuItem<Identity: ITfMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uid: super::UINT, dwflags: super::DWORD, hbmp: super::HBITMAP, hbmpmask: super::HBITMAP, pch: *const super::WCHAR, cch: super::ULONG, ppmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMenu_Impl::AddMenuItem(this, core::mem::transmute_copy(&uid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&hbmp), core::mem::transmute_copy(&hbmpmask), core::mem::transmute_copy(&pch), core::mem::transmute_copy(&cch), core::mem::transmute_copy(&ppmenu)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddMenuItem: AddMenuItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMenu as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfMenu {}
windows_core::imp::define_interface!(ITfSystemDeviceTypeLangBarItem, ITfSystemDeviceTypeLangBarItem_Vtbl, 0x45672eb9_9059_46a2_838d_4530355f6a77);
windows_core::imp::interface_hierarchy!(ITfSystemDeviceTypeLangBarItem, windows_core::IUnknown);
impl ITfSystemDeviceTypeLangBarItem {
    #[cfg(feature = "shared")]
    pub unsafe fn SetIconMode(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconMode)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetIconMode(&self, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIconMode)(windows_core::Interface::as_raw(self), pdwflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetIconMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetIconMode: usize,
    #[cfg(feature = "shared")]
    pub GetIconMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetIconMode: usize,
}
#[cfg(feature = "shared")]
pub trait ITfSystemDeviceTypeLangBarItem_Impl: windows_core::IUnknownImpl {
    fn SetIconMode(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn GetIconMode(&self, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub const fn new<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIconMode<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemDeviceTypeLangBarItem_Impl::SetIconMode(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetIconMode<Identity: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemDeviceTypeLangBarItem_Impl::GetIconMode(this, core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIconMode: SetIconMode::<Identity, OFFSET>,
            GetIconMode: GetIconMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemDeviceTypeLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITfSystemDeviceTypeLangBarItem {}
windows_core::imp::define_interface!(ITfSystemLangBarItem, ITfSystemLangBarItem_Vtbl, 0x1e13e9ec_6b33_4d4a_b5eb_8a92f029f356);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItem, windows_core::IUnknown);
impl ITfSystemLangBarItem {
    #[cfg(feature = "windef")]
    pub unsafe fn SetIcon(&self, hicon: super::HICON) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIcon)(windows_core::Interface::as_raw(self), hicon) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetTooltipString(&self, pchtooltip: &[super::WCHAR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTooltipString)(windows_core::Interface::as_raw(self), pchtooltip.as_ptr(), pchtooltip.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, super::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetIcon: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetTooltipString: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetTooltipString: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub trait ITfSystemLangBarItem_Impl: windows_core::IUnknownImpl {
    fn SetIcon(&self, hicon: super::HICON) -> windows_core::Result<()>;
    fn SetTooltipString(&self, pchtooltip: *const super::WCHAR, cch: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl ITfSystemLangBarItem_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIcon<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hicon: super::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItem_Impl::SetIcon(this, core::mem::transmute_copy(&hicon)).into()
            }
        }
        unsafe extern "system" fn SetTooltipString<Identity: ITfSystemLangBarItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchtooltip: *const super::WCHAR, cch: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItem_Impl::SetTooltipString(this, core::mem::transmute_copy(&pchtooltip), core::mem::transmute_copy(&cch)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetIcon: SetIcon::<Identity, OFFSET>,
            SetTooltipString: SetTooltipString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfSystemLangBarItem {}
windows_core::imp::define_interface!(ITfSystemLangBarItemSink, ITfSystemLangBarItemSink_Vtbl, 0x1449d9ab_13cf_4687_aa3e_8d8b18574396);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItemSink, windows_core::IUnknown);
impl ITfSystemLangBarItemSink {
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfMenu>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitMenu)(windows_core::Interface::as_raw(self), pmenu.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnMenuSelect)(windows_core::Interface::as_raw(self), wid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub OnMenuSelect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnMenuSelect: usize,
}
#[cfg(feature = "shared")]
pub trait ITfSystemLangBarItemSink_Impl: windows_core::IUnknownImpl {
    fn InitMenu(&self, pmenu: windows_core::Ref<ITfMenu>) -> windows_core::Result<()>;
    fn OnMenuSelect(&self, wid: super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITfSystemLangBarItemSink_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitMenu<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmenu: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItemSink_Impl::InitMenu(this, core::mem::transmute_copy(&pmenu)).into()
            }
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ITfSystemLangBarItemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItemSink_Impl::OnMenuSelect(this, core::mem::transmute_copy(&wid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitMenu: InitMenu::<Identity, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITfSystemLangBarItemSink {}
windows_core::imp::define_interface!(ITfSystemLangBarItemText, ITfSystemLangBarItemText_Vtbl, 0x5c4ce0e5_ba49_4b52_ac6b_3b397b4f701f);
windows_core::imp::interface_hierarchy!(ITfSystemLangBarItemText, windows_core::IUnknown);
impl ITfSystemLangBarItemText {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetItemText(&self, pch: &[super::WCHAR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItemText)(windows_core::Interface::as_raw(self), pch.as_ptr(), pch.len().try_into().unwrap()) }
    }
    pub unsafe fn GetItemText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemText)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetItemText: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WCHAR, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetItemText: usize,
    pub GetItemText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait ITfSystemLangBarItemText_Impl: windows_core::IUnknownImpl {
    fn SetItemText(&self, pch: *const super::WCHAR, cch: super::ULONG) -> windows_core::Result<()>;
    fn GetItemText(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl ITfSystemLangBarItemText_Vtbl {
    pub const fn new<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetItemText<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pch: *const super::WCHAR, cch: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItemText_Impl::SetItemText(this, core::mem::transmute_copy(&pch), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn GetItemText<Identity: ITfSystemLangBarItemText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSystemLangBarItemText_Impl::GetItemText(this, core::mem::transmute_copy(&pbstrtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetItemText: SetItemText::<Identity, OFFSET>,
            GetItemText: GetItemText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSystemLangBarItemText as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITfSystemLangBarItemText {}
pub const TF_DTLBI_USEPROFILEICON: i32 = 1;
pub const TF_FLOATINGLANGBAR_WNDTITLE: windows_core::PCSTR = windows_core::s!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEA: windows_core::PCSTR = windows_core::s!("TF_FloatingLangBar_WndTitle");
pub const TF_FLOATINGLANGBAR_WNDTITLEW: windows_core::PCWSTR = windows_core::w!("TF_FloatingLangBar_WndTitle");
#[cfg(feature = "shared")]
pub const TF_INVALIDMENUITEM: super::UINT = 4294967295;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: super::CLSID,
    pub guidItem: windows_core::GUID,
    pub dwStyle: super::DWORD,
    pub ulSort: super::ULONG,
    pub szDescription: [super::WCHAR; 32],
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
impl Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: core::mem::ManuallyDrop<windows_core::BSTR>,
}
pub const TF_LBI_BALLOON: i32 = 16;
pub const TF_LBI_BITMAP: i32 = 8;
pub const TF_LBI_BMPALL: i32 = 12;
pub const TF_LBI_BMPBTNALL: i32 = 14;
pub const TF_LBI_BMPF_VERTICAL: i32 = 1;
pub const TF_LBI_BTNALL: i32 = 7;
pub const TF_LBI_CLK_LEFT: TfLBIClick = 2;
pub const TF_LBI_CLK_RIGHT: TfLBIClick = 1;
pub const TF_LBI_CUSTOMUI: i32 = 32;
pub const TF_LBI_DESC_MAXLEN: i32 = 32;
pub const TF_LBI_ICON: i32 = 1;
pub const TF_LBI_STATUS: i32 = 65536;
pub const TF_LBI_STATUS_BTN_TOGGLED: i32 = 65536;
pub const TF_LBI_STATUS_DISABLED: i32 = 2;
pub const TF_LBI_STATUS_HIDDEN: i32 = 1;
pub const TF_LBI_STYLE_BTN_BUTTON: i32 = 65536;
pub const TF_LBI_STYLE_BTN_MENU: i32 = 131072;
pub const TF_LBI_STYLE_BTN_TOGGLE: i32 = 262144;
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: i32 = 16;
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: i32 = 1;
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: i32 = 4;
pub const TF_LBI_STYLE_SHOWNINTRAY: i32 = 2;
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: i32 = 8;
pub const TF_LBI_STYLE_TEXTCOLORICON: i32 = 32;
pub const TF_LBI_TEXT: i32 = 2;
pub const TF_LBI_TOOLTIP: i32 = 4;
pub const TF_LBMENUF_CHECKED: i32 = 1;
pub const TF_LBMENUF_GRAYED: i32 = 16;
pub const TF_LBMENUF_RADIOCHECKED: i32 = 8;
pub const TF_LBMENUF_SEPARATOR: i32 = 4;
pub const TF_LBMENUF_SUBMENU: i32 = 2;
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = 2;
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = 0;
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = 1;
pub const TF_SFT_DESKBAND: i32 = 2048;
pub const TF_SFT_DOCK: i32 = 2;
pub const TF_SFT_EXTRAICONSONMINIMIZED: i32 = 512;
pub const TF_SFT_HIDDEN: i32 = 8;
pub const TF_SFT_HIGHTRANSPARENCY: i32 = 64;
pub const TF_SFT_LABELS: i32 = 128;
pub const TF_SFT_LOWTRANSPARENCY: i32 = 32;
pub const TF_SFT_MINIMIZED: i32 = 4;
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: i32 = 1024;
pub const TF_SFT_NOLABELS: i32 = 256;
pub const TF_SFT_NOTRANSPARENCY: i32 = 16;
pub const TF_SFT_SHOWNORMAL: i32 = 1;
pub type TfLBBalloonStyle = i32;
pub type TfLBIClick = i32;
