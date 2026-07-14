pub type BINDSPEED = i32;
pub const BINDSPEED_IMMEDIATE: BINDSPEED = 3;
pub const BINDSPEED_INDEFINITE: BINDSPEED = 1;
pub const BINDSPEED_MODERATE: BINDSPEED = 2;
#[cfg(feature = "windef")]
pub type BORDERWIDTHS = super::windef::RECT;
pub const DD_DEFDRAGDELAY: u32 = 200;
pub const DD_DEFDRAGMINDIST: u32 = 2;
pub const DD_DEFSCROLLDELAY: u32 = 50;
pub const DD_DEFSCROLLINSET: u32 = 11;
pub const DD_DEFSCROLLINTERVAL: u32 = 50;
pub type DISCARDCACHE = i32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = 1;
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = 0;
pub const DROPEFFECT_BACKGROUNDTARGET: u32 = 536870912;
pub const DROPEFFECT_COPY: u32 = 1;
pub const DROPEFFECT_LINK: u32 = 4;
pub const DROPEFFECT_MOVE: u32 = 2;
pub const DROPEFFECT_NEWTARGET: u32 = 1073741824;
pub const DROPEFFECT_NONE: u32 = 0;
pub const DROPEFFECT_SCROLL: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type HOLEMENU = super::minwindef::HGLOBAL;
windows_core::imp::define_interface!(IContinue, IContinue_Vtbl, 0x0000012a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IContinue, windows_core::IUnknown);
impl IContinue {
    pub unsafe fn FContinue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FContinue)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContinue_Impl: windows_core::IUnknownImpl {
    fn FContinue(&self) -> windows_core::Result<()>;
}
impl IContinue_Vtbl {
    pub const fn new<Identity: IContinue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FContinue<Identity: IContinue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContinue_Impl::FContinue(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FContinue: FContinue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContinue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContinue {}
windows_core::imp::define_interface!(IDropSource, IDropSource_Vtbl, 0x00000121_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDropSource, windows_core::IUnknown);
impl IDropSource {
    pub unsafe fn QueryContinueDrag(&self, fescapepressed: bool, grfkeystate: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryContinueDrag)(windows_core::Interface::as_raw(self), fescapepressed.into(), grfkeystate) }
    }
    pub unsafe fn GiveFeedback(&self, dweffect: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GiveFeedback)(windows_core::Interface::as_raw(self), dweffect) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryContinueDrag: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GiveFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDropSource_Impl: windows_core::IUnknownImpl {
    fn QueryContinueDrag(&self, fescapepressed: windows_core::BOOL, grfkeystate: u32) -> windows_core::Result<()>;
    fn GiveFeedback(&self, dweffect: u32) -> windows_core::Result<()>;
}
impl IDropSource_Vtbl {
    pub const fn new<Identity: IDropSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryContinueDrag<Identity: IDropSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fescapepressed: windows_core::BOOL, grfkeystate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropSource_Impl::QueryContinueDrag(this, core::mem::transmute_copy(&fescapepressed), core::mem::transmute_copy(&grfkeystate)).into()
            }
        }
        unsafe extern "system" fn GiveFeedback<Identity: IDropSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffect: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropSource_Impl::GiveFeedback(this, core::mem::transmute_copy(&dweffect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryContinueDrag: QueryContinueDrag::<Identity, OFFSET>,
            GiveFeedback: GiveFeedback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDropSource {}
windows_core::imp::define_interface!(IDropSourceNotify, IDropSourceNotify_Vtbl, 0x0000012b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDropSourceNotify, windows_core::IUnknown);
impl IDropSourceNotify {
    #[cfg(feature = "windef")]
    pub unsafe fn DragEnterTarget(&self, hwndtarget: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragEnterTarget)(windows_core::Interface::as_raw(self), hwndtarget) }
    }
    pub unsafe fn DragLeaveTarget(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragLeaveTarget)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSourceNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub DragEnterTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DragEnterTarget: usize,
    pub DragLeaveTarget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IDropSourceNotify_Impl: windows_core::IUnknownImpl {
    fn DragEnterTarget(&self, hwndtarget: super::windef::HWND) -> windows_core::Result<()>;
    fn DragLeaveTarget(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDropSourceNotify_Vtbl {
    pub const fn new<Identity: IDropSourceNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DragEnterTarget<Identity: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndtarget: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropSourceNotify_Impl::DragEnterTarget(this, core::mem::transmute_copy(&hwndtarget)).into()
            }
        }
        unsafe extern "system" fn DragLeaveTarget<Identity: IDropSourceNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropSourceNotify_Impl::DragLeaveTarget(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnterTarget: DragEnterTarget::<Identity, OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropSourceNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDropSourceNotify {}
windows_core::imp::define_interface!(IDropTarget, IDropTarget_Vtbl, 0x00000122_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDropTarget, windows_core::IUnknown);
impl IDropTarget {
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn DragEnter<P0>(&self, pdataobj: P0, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).DragEnter)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, pt, pdweffect as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DragOver(&self, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragOver)(windows_core::Interface::as_raw(self), grfkeystate, pt, pdweffect as _) }
    }
    pub unsafe fn DragLeave(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragLeave)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn Drop<P0>(&self, pdataobj: P0, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Drop)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, pt, pdweffect as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub DragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::windef::POINTL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    DragEnter: usize,
    #[cfg(feature = "windef")]
    pub DragOver: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::POINTL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DragOver: usize,
    pub DragLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub Drop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::windef::POINTL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    Drop: usize,
}
#[cfg(all(feature = "objidl", feature = "windef"))]
pub trait IDropTarget_Impl: windows_core::IUnknownImpl {
    fn DragEnter(&self, pdataobj: windows_core::Ref<super::objidl::IDataObject>, grfkeystate: u32, pt: &super::windef::POINTL, pdweffect: *mut u32) -> windows_core::Result<()>;
    fn DragOver(&self, grfkeystate: u32, pt: &super::windef::POINTL, pdweffect: *mut u32) -> windows_core::Result<()>;
    fn DragLeave(&self) -> windows_core::Result<()>;
    fn Drop(&self, pdataobj: windows_core::Ref<super::objidl::IDataObject>, grfkeystate: u32, pt: &super::windef::POINTL, pdweffect: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl IDropTarget_Vtbl {
    pub const fn new<Identity: IDropTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DragEnter<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragEnter(this, core::mem::transmute_copy(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn DragOver<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragOver(this, core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn DragLeave<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragLeave(this).into()
            }
        }
        unsafe extern "system" fn Drop<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: u32, pt: super::windef::POINTL, pdweffect: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::Drop(this, core::mem::transmute_copy(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragOver: DragOver::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            Drop: Drop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "windef"))]
impl windows_core::RuntimeName for IDropTarget {}
windows_core::imp::define_interface!(IEnterpriseDropTarget, IEnterpriseDropTarget_Vtbl, 0x390e3878_fd55_4e18_819d_4682081c0cfd);
windows_core::imp::interface_hierarchy!(IEnterpriseDropTarget, windows_core::IUnknown);
impl IEnterpriseDropTarget {
    pub unsafe fn SetDropSourceEnterpriseId<P0>(&self, identity: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDropSourceEnterpriseId)(windows_core::Interface::as_raw(self), identity.param().abi()) }
    }
    pub unsafe fn IsEvaluatingEdpPolicy(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEvaluatingEdpPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseDropTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDropSourceEnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsEvaluatingEdpPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IEnterpriseDropTarget_Impl: windows_core::IUnknownImpl {
    fn SetDropSourceEnterpriseId(&self, identity: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsEvaluatingEdpPolicy(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IEnterpriseDropTarget_Vtbl {
    pub const fn new<Identity: IEnterpriseDropTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Identity: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, identity: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnterpriseDropTarget_Impl::SetDropSourceEnterpriseId(this, core::mem::transmute(&identity)).into()
            }
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Identity: IEnterpriseDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnterpriseDropTarget_Impl::IsEvaluatingEdpPolicy(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Identity, OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnterpriseDropTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnterpriseDropTarget {}
windows_core::imp::define_interface!(IEnumOLEVERB, IEnumOLEVERB_Vtbl, 0x00000104_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumOLEVERB, windows_core::IUnknown);
impl IEnumOLEVERB {
    pub unsafe fn Next(&self, rgelt: &mut [OLEVERB], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), rgelt.as_mut_ptr(), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
pub struct IEnumOLEVERB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut OLEVERB, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOLEVERB_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOLEVERB>;
}
impl IEnumOLEVERB_Vtbl {
    pub const fn new<Identity: IEnumOLEVERB_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOLEVERB_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOLEVERB_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOLEVERB_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOLEVERB_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOLEVERB_Impl::Clone(this) {
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
        iid == &<IEnumOLEVERB as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOLEVERB {}
windows_core::imp::define_interface!(IOleAdviseHolder, IOleAdviseHolder_Vtbl, 0x00000111_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleAdviseHolder, windows_core::IUnknown);
impl IOleAdviseHolder {
    #[cfg(feature = "objidl")]
    pub unsafe fn Advise<P0>(&self, padvise: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::objidl::IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), padvise.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SendOnRename<P0>(&self, pmk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendOnRename)(windows_core::Interface::as_raw(self), pmk.param().abi()) }
    }
    pub unsafe fn SendOnSave(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendOnSave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SendOnClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendOnClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleAdviseHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    EnumAdvise: usize,
    #[cfg(feature = "objidl")]
    pub SendOnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SendOnRename: usize,
    pub SendOnSave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait IOleAdviseHolder_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, padvise: windows_core::Ref<super::objidl::IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA>;
    fn SendOnRename(&self, pmk: windows_core::Ref<super::objidl::IMoniker>) -> windows_core::Result<()>;
    fn SendOnSave(&self) -> windows_core::Result<()>;
    fn SendOnClose(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IOleAdviseHolder_Vtbl {
    pub const fn new<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvise: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleAdviseHolder_Impl::Advise(this, core::mem::transmute_copy(&padvise)) {
                    Ok(ok__) => {
                        pdwconnection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleAdviseHolder_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumAdvise<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleAdviseHolder_Impl::EnumAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendOnRename<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleAdviseHolder_Impl::SendOnRename(this, core::mem::transmute_copy(&pmk)).into()
            }
        }
        unsafe extern "system" fn SendOnSave<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleAdviseHolder_Impl::SendOnSave(this).into()
            }
        }
        unsafe extern "system" fn SendOnClose<Identity: IOleAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleAdviseHolder_Impl::SendOnClose(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            SendOnRename: SendOnRename::<Identity, OFFSET>,
            SendOnSave: SendOnSave::<Identity, OFFSET>,
            SendOnClose: SendOnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleAdviseHolder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IOleAdviseHolder {}
windows_core::imp::define_interface!(IOleCache, IOleCache_Vtbl, 0x0000011e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleCache, windows_core::IUnknown);
impl IOleCache {
    #[cfg(all(feature = "objidl", feature = "wtypes"))]
    pub unsafe fn Cache(&self, pformatetc: *const super::objidl::FORMATETC, advf: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cache)(windows_core::Interface::as_raw(self), pformatetc, advf, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Uncache(&self, dwconnection: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Uncache)(windows_core::Interface::as_raw(self), dwconnection) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn EnumCache(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCache)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn InitCache<P0>(&self, pdataobject: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitCache)(windows_core::Interface::as_raw(self), pdataobject.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn SetData(&self, pformatetc: *const super::objidl::FORMATETC, pmedium: *const super::objidl::STGMEDIUM, frelease: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pformatetc, pmedium, frelease.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "wtypes"))]
    pub Cache: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::objidl::FORMATETC, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "wtypes")))]
    Cache: usize,
    pub Uncache: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub EnumCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    EnumCache: usize,
    #[cfg(feature = "objidl")]
    pub InitCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    InitCache: usize,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::objidl::FORMATETC, *const super::objidl::STGMEDIUM, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    SetData: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IOleCache_Impl: windows_core::IUnknownImpl {
    fn Cache(&self, pformatetc: *const super::objidl::FORMATETC, advf: u32) -> windows_core::Result<u32>;
    fn Uncache(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumCache(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA>;
    fn InitCache(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn SetData(&self, pformatetc: *const super::objidl::FORMATETC, pmedium: *const super::objidl::STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IOleCache_Vtbl {
    pub const fn new<Identity: IOleCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cache<Identity: IOleCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const super::objidl::FORMATETC, advf: u32, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleCache_Impl::Cache(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&advf)) {
                    Ok(ok__) => {
                        pdwconnection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Uncache<Identity: IOleCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCache_Impl::Uncache(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumCache<Identity: IOleCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumstatdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleCache_Impl::EnumCache(this) {
                    Ok(ok__) => {
                        ppenumstatdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitCache<Identity: IOleCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCache_Impl::InitCache(this, core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn SetData<Identity: IOleCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const super::objidl::FORMATETC, pmedium: *const super::objidl::STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCache_Impl::SetData(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&frelease)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cache: Cache::<Identity, OFFSET>,
            Uncache: Uncache::<Identity, OFFSET>,
            EnumCache: EnumCache::<Identity, OFFSET>,
            InitCache: InitCache::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCache as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IOleCache {}
windows_core::imp::define_interface!(IOleCache2, IOleCache2_Vtbl, 0x00000128_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleCache2 {
    type Target = IOleCache;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleCache2, windows_core::IUnknown, IOleCache);
impl IOleCache2 {
    #[cfg(feature = "objidl")]
    pub unsafe fn UpdateCache<P0>(&self, pdataobject: P0, grfupdf: u32, preserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateCache)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), grfupdf, preserved.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DiscardCache(&self, dwdiscardoptions: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiscardCache)(windows_core::Interface::as_raw(self), dwdiscardoptions) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache2_Vtbl {
    pub base__: IOleCache_Vtbl,
    #[cfg(feature = "objidl")]
    pub UpdateCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    UpdateCache: usize,
    pub DiscardCache: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IOleCache2_Impl: IOleCache_Impl {
    fn UpdateCache(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>, grfupdf: u32, preserved: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DiscardCache(&self, dwdiscardoptions: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IOleCache2_Vtbl {
    pub const fn new<Identity: IOleCache2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateCache<Identity: IOleCache2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, grfupdf: u32, preserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCache2_Impl::UpdateCache(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&grfupdf), core::mem::transmute_copy(&preserved)).into()
            }
        }
        unsafe extern "system" fn DiscardCache<Identity: IOleCache2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdiscardoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCache2_Impl::DiscardCache(this, core::mem::transmute_copy(&dwdiscardoptions)).into()
            }
        }
        Self { base__: IOleCache_Vtbl::new::<Identity, OFFSET>(), UpdateCache: UpdateCache::<Identity, OFFSET>, DiscardCache: DiscardCache::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCache2 as windows_core::Interface>::IID || iid == &<IOleCache as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IOleCache2 {}
windows_core::imp::define_interface!(IOleCacheControl, IOleCacheControl_Vtbl, 0x00000129_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleCacheControl, windows_core::IUnknown);
impl IOleCacheControl {
    #[cfg(feature = "objidl")]
    pub unsafe fn OnRun<P0>(&self, pdataobject: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRun)(windows_core::Interface::as_raw(self), pdataobject.param().abi()) }
    }
    pub unsafe fn OnStop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStop)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCacheControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub OnRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    OnRun: usize,
    pub OnStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait IOleCacheControl_Impl: windows_core::IUnknownImpl {
    fn OnRun(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn OnStop(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IOleCacheControl_Vtbl {
    pub const fn new<Identity: IOleCacheControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRun<Identity: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCacheControl_Impl::OnRun(this, core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn OnStop<Identity: IOleCacheControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleCacheControl_Impl::OnStop(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnRun: OnRun::<Identity, OFFSET>, OnStop: OnStop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleCacheControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IOleCacheControl {}
windows_core::imp::define_interface!(IOleClientSite, IOleClientSite_Vtbl, 0x00000118_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleClientSite, windows_core::IUnknown);
impl IOleClientSite {
    pub unsafe fn SaveObject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveObject)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> windows_core::Result<super::objidl::IMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), dwassign, dwwhichmoniker, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetContainer(&self) -> windows_core::Result<IOleContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ShowObject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnShowWindow(&self, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnShowWindow)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    pub unsafe fn RequestNewObjectLayout(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestNewObjectLayout)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleClientSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SaveObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetMoniker: usize,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnShowWindow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RequestNewObjectLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait IOleClientSite_Impl: windows_core::IUnknownImpl {
    fn SaveObject(&self) -> windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> windows_core::Result<super::objidl::IMoniker>;
    fn GetContainer(&self) -> windows_core::Result<IOleContainer>;
    fn ShowObject(&self) -> windows_core::Result<()>;
    fn OnShowWindow(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn RequestNewObjectLayout(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IOleClientSite_Vtbl {
    pub const fn new<Identity: IOleClientSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SaveObject<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleClientSite_Impl::SaveObject(this).into()
            }
        }
        unsafe extern "system" fn GetMoniker<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleClientSite_Impl::GetMoniker(this, core::mem::transmute_copy(&dwassign), core::mem::transmute_copy(&dwwhichmoniker)) {
                    Ok(ok__) => {
                        ppmk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContainer<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleClientSite_Impl::GetContainer(this) {
                    Ok(ok__) => {
                        ppcontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShowObject<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleClientSite_Impl::ShowObject(this).into()
            }
        }
        unsafe extern "system" fn OnShowWindow<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleClientSite_Impl::OnShowWindow(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn RequestNewObjectLayout<Identity: IOleClientSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleClientSite_Impl::RequestNewObjectLayout(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveObject: SaveObject::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
            ShowObject: ShowObject::<Identity, OFFSET>,
            OnShowWindow: OnShowWindow::<Identity, OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleClientSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IOleClientSite {}
windows_core::imp::define_interface!(IOleContainer, IOleContainer_Vtbl, 0x0000011b_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleContainer {
    type Target = IParseDisplayName;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleContainer, windows_core::IUnknown, IParseDisplayName);
impl IOleContainer {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn EnumObjects(&self, grfflags: u32) -> windows_core::Result<super::objidlbase::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), grfflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockContainer(&self, flock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockContainer)(windows_core::Interface::as_raw(self), flock.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleContainer_Vtbl {
    pub base__: IParseDisplayName_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    EnumObjects: usize,
    pub LockContainer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
pub trait IOleContainer_Impl: IParseDisplayName_Impl {
    fn EnumObjects(&self, grfflags: u32) -> windows_core::Result<super::objidlbase::IEnumUnknown>;
    fn LockContainer(&self, flock: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl IOleContainer_Vtbl {
    pub const fn new<Identity: IOleContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumObjects<Identity: IOleContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleContainer_Impl::EnumObjects(this, core::mem::transmute_copy(&grfflags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockContainer<Identity: IOleContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleContainer_Impl::LockContainer(this, core::mem::transmute_copy(&flock)).into()
            }
        }
        Self {
            base__: IParseDisplayName_Vtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            LockContainer: LockContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleContainer as windows_core::Interface>::IID || iid == &<IParseDisplayName as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl windows_core::RuntimeName for IOleContainer {}
windows_core::imp::define_interface!(IOleInPlaceActiveObject, IOleInPlaceActiveObject_Vtbl, 0x00000117_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceActiveObject {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceActiveObject, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceActiveObject {
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: Option<*const super::winuser::MSG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), lpmsg.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn OnFrameWindowActivate(&self, factivate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnFrameWindowActivate)(windows_core::Interface::as_raw(self), factivate.into()) }
    }
    pub unsafe fn OnDocWindowActivate(&self, factivate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnDocWindowActivate)(windows_core::Interface::as_raw(self), factivate.into()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ResizeBorder<P1>(&self, prcborder: *const super::windef::RECT, puiwindow: P1, fframewindow: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IOleInPlaceUIWindow>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResizeBorder)(windows_core::Interface::as_raw(self), prcborder, puiwindow.param().abi(), fframewindow.into()) }
    }
    pub unsafe fn EnableModeless(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceActiveObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    TranslateAccelerator: usize,
    pub OnFrameWindowActivate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnDocWindowActivate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub ResizeBorder: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ResizeBorder: usize,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IOleInPlaceActiveObject_Impl: IOleWindow_Impl {
    fn TranslateAccelerator(&self, lpmsg: *const super::winuser::MSG) -> windows_core::Result<()>;
    fn OnFrameWindowActivate(&self, factivate: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnDocWindowActivate(&self, factivate: windows_core::BOOL) -> windows_core::Result<()>;
    fn ResizeBorder(&self, prcborder: *const super::windef::RECT, puiwindow: windows_core::Ref<IOleInPlaceUIWindow>, fframewindow: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnableModeless(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IOleInPlaceActiveObject_Vtbl {
    pub const fn new<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TranslateAccelerator<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *const super::winuser::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceActiveObject_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&lpmsg)).into()
            }
        }
        unsafe extern "system" fn OnFrameWindowActivate<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factivate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceActiveObject_Impl::OnFrameWindowActivate(this, core::mem::transmute_copy(&factivate)).into()
            }
        }
        unsafe extern "system" fn OnDocWindowActivate<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factivate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceActiveObject_Impl::OnDocWindowActivate(this, core::mem::transmute_copy(&factivate)).into()
            }
        }
        unsafe extern "system" fn ResizeBorder<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcborder: *const super::windef::RECT, puiwindow: *mut core::ffi::c_void, fframewindow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceActiveObject_Impl::ResizeBorder(this, core::mem::transmute_copy(&prcborder), core::mem::transmute_copy(&puiwindow), core::mem::transmute_copy(&fframewindow)).into()
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: IOleInPlaceActiveObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceActiveObject_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Identity, OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Identity, OFFSET>,
            ResizeBorder: ResizeBorder::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceActiveObject as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IOleInPlaceActiveObject {}
windows_core::imp::define_interface!(IOleInPlaceFrame, IOleInPlaceFrame_Vtbl, 0x00000116_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceFrame {
    type Target = IOleInPlaceUIWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceFrame, windows_core::IUnknown, IOleWindow, IOleInPlaceUIWindow);
impl IOleInPlaceFrame {
    #[cfg(feature = "windef")]
    pub unsafe fn InsertMenus(&self, hmenushared: super::windef::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertMenus)(windows_core::Interface::as_raw(self), hmenushared, lpmenuwidths as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
    pub unsafe fn SetMenu(&self, hmenushared: super::windef::HMENU, holemenu: HOLEMENU, hwndactiveobject: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMenu)(windows_core::Interface::as_raw(self), hmenushared, holemenu, hwndactiveobject) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RemoveMenus(&self, hmenushared: super::windef::HMENU) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveMenus)(windows_core::Interface::as_raw(self), hmenushared) }
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatusText)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()) }
    }
    pub unsafe fn EnableModeless(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *const super::winuser::MSG, wid: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), lpmsg, wid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceFrame_Vtbl {
    pub base__: IOleInPlaceUIWindow_Vtbl,
    #[cfg(feature = "windef")]
    pub InsertMenus: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HMENU, *mut OLEMENUGROUPWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InsertMenus: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
    pub SetMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HMENU, HOLEMENU, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winnt")))]
    SetMenu: usize,
    #[cfg(feature = "windef")]
    pub RemoveMenus: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RemoveMenus: usize,
    pub SetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winuser::MSG, u16) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    TranslateAccelerator: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub trait IOleInPlaceFrame_Impl: IOleInPlaceUIWindow_Impl {
    fn InsertMenus(&self, hmenushared: super::windef::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::Result<()>;
    fn SetMenu(&self, hmenushared: super::windef::HMENU, holemenu: HOLEMENU, hwndactiveobject: super::windef::HWND) -> windows_core::Result<()>;
    fn RemoveMenus(&self, hmenushared: super::windef::HMENU) -> windows_core::Result<()>;
    fn SetStatusText(&self, pszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnableModeless(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, lpmsg: *const super::winuser::MSG, wid: u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl IOleInPlaceFrame_Vtbl {
    pub const fn new<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertMenus<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::windef::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::InsertMenus(this, core::mem::transmute_copy(&hmenushared), core::mem::transmute_copy(&lpmenuwidths)).into()
            }
        }
        unsafe extern "system" fn SetMenu<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::windef::HMENU, holemenu: HOLEMENU, hwndactiveobject: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::SetMenu(this, core::mem::transmute_copy(&hmenushared), core::mem::transmute_copy(&holemenu), core::mem::transmute_copy(&hwndactiveobject)).into()
            }
        }
        unsafe extern "system" fn RemoveMenus<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmenushared: super::windef::HMENU) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::RemoveMenus(this, core::mem::transmute_copy(&hmenushared)).into()
            }
        }
        unsafe extern "system" fn SetStatusText<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::SetStatusText(this, core::mem::transmute(&pszstatustext)).into()
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: IOleInPlaceFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *const super::winuser::MSG, wid: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceFrame_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&lpmsg), core::mem::transmute_copy(&wid)).into()
            }
        }
        Self {
            base__: IOleInPlaceUIWindow_Vtbl::new::<Identity, OFFSET>(),
            InsertMenus: InsertMenus::<Identity, OFFSET>,
            SetMenu: SetMenu::<Identity, OFFSET>,
            RemoveMenus: RemoveMenus::<Identity, OFFSET>,
            SetStatusText: SetStatusText::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceFrame as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID || iid == &<IOleInPlaceUIWindow as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl windows_core::RuntimeName for IOleInPlaceFrame {}
windows_core::imp::define_interface!(IOleInPlaceObject, IOleInPlaceObject_Vtbl, 0x00000113_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceObject {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceObject, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceObject {
    pub unsafe fn InPlaceDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InPlaceDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UIDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UIDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::windef::RECT, lprccliprect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetObjectRects)(windows_core::Interface::as_raw(self), lprcposrect, lprccliprect) }
    }
    pub unsafe fn ReactivateAndUndo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReactivateAndUndo)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub InPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetObjectRects: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetObjectRects: usize,
    pub ReactivateAndUndo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IOleInPlaceObject_Impl: IOleWindow_Impl {
    fn InPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn UIDeactivate(&self) -> windows_core::Result<()>;
    fn SetObjectRects(&self, lprcposrect: *const super::windef::RECT, lprccliprect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn ReactivateAndUndo(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IOleInPlaceObject_Vtbl {
    pub const fn new<Identity: IOleInPlaceObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InPlaceDeactivate<Identity: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceObject_Impl::InPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn UIDeactivate<Identity: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceObject_Impl::UIDeactivate(this).into()
            }
        }
        unsafe extern "system" fn SetObjectRects<Identity: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprcposrect: *const super::windef::RECT, lprccliprect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceObject_Impl::SetObjectRects(this, core::mem::transmute_copy(&lprcposrect), core::mem::transmute_copy(&lprccliprect)).into()
            }
        }
        unsafe extern "system" fn ReactivateAndUndo<Identity: IOleInPlaceObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceObject_Impl::ReactivateAndUndo(this).into()
            }
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            InPlaceDeactivate: InPlaceDeactivate::<Identity, OFFSET>,
            UIDeactivate: UIDeactivate::<Identity, OFFSET>,
            SetObjectRects: SetObjectRects::<Identity, OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceObject as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IOleInPlaceObject {}
windows_core::imp::define_interface!(IOleInPlaceSite, IOleInPlaceSite_Vtbl, 0x00000119_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceSite {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceSite, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceSite {
    pub unsafe fn CanInPlaceActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanInPlaceActivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnInPlaceActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInPlaceActivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnUIActivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUIActivate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut Option<IOleInPlaceFrame>, ppdoc: *mut Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::windef::RECT, lprccliprect: *mut super::windef::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWindowContext)(windows_core::Interface::as_raw(self), core::mem::transmute(ppframe), core::mem::transmute(ppdoc), lprcposrect as _, lprccliprect as _, lpframeinfo as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Scroll(&self, scrollextant: super::windef::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), scrollextant) }
    }
    pub unsafe fn OnUIDeactivate(&self, fundoable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUIDeactivate)(windows_core::Interface::as_raw(self), fundoable.into()) }
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnInPlaceDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DiscardUndoState(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiscardUndoState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeactivateAndUndo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeactivateAndUndo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnPosRectChange)(windows_core::Interface::as_raw(self), lprcposrect) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSite_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub CanInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetWindowContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::windef::RECT, *mut super::windef::RECT, *mut OLEINPLACEFRAMEINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindowContext: usize,
    #[cfg(feature = "windef")]
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Scroll: usize,
    pub OnUIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnInPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscardUndoState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeactivateAndUndo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub OnPosRectChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    OnPosRectChange: usize,
}
#[cfg(feature = "windef")]
pub trait IOleInPlaceSite_Impl: IOleWindow_Impl {
    fn CanInPlaceActivate(&self) -> windows_core::Result<()>;
    fn OnInPlaceActivate(&self) -> windows_core::Result<()>;
    fn OnUIActivate(&self) -> windows_core::Result<()>;
    fn GetWindowContext(&self, ppframe: windows_core::OutRef<IOleInPlaceFrame>, ppdoc: windows_core::OutRef<IOleInPlaceUIWindow>, lprcposrect: *mut super::windef::RECT, lprccliprect: *mut super::windef::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::Result<()>;
    fn Scroll(&self, scrollextant: &super::windef::SIZE) -> windows_core::Result<()>;
    fn OnUIDeactivate(&self, fundoable: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnInPlaceDeactivate(&self) -> windows_core::Result<()>;
    fn DiscardUndoState(&self) -> windows_core::Result<()>;
    fn DeactivateAndUndo(&self) -> windows_core::Result<()>;
    fn OnPosRectChange(&self, lprcposrect: *const super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IOleInPlaceSite_Vtbl {
    pub const fn new<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanInPlaceActivate<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::CanInPlaceActivate(this).into()
            }
        }
        unsafe extern "system" fn OnInPlaceActivate<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::OnInPlaceActivate(this).into()
            }
        }
        unsafe extern "system" fn OnUIActivate<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::OnUIActivate(this).into()
            }
        }
        unsafe extern "system" fn GetWindowContext<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppframe: *mut *mut core::ffi::c_void, ppdoc: *mut *mut core::ffi::c_void, lprcposrect: *mut super::windef::RECT, lprccliprect: *mut super::windef::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::GetWindowContext(this, core::mem::transmute_copy(&ppframe), core::mem::transmute_copy(&ppdoc), core::mem::transmute_copy(&lprcposrect), core::mem::transmute_copy(&lprccliprect), core::mem::transmute_copy(&lpframeinfo)).into()
            }
        }
        unsafe extern "system" fn Scroll<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollextant: super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::Scroll(this, core::mem::transmute(&scrollextant)).into()
            }
        }
        unsafe extern "system" fn OnUIDeactivate<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fundoable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::OnUIDeactivate(this, core::mem::transmute_copy(&fundoable)).into()
            }
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::OnInPlaceDeactivate(this).into()
            }
        }
        unsafe extern "system" fn DiscardUndoState<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::DiscardUndoState(this).into()
            }
        }
        unsafe extern "system" fn DeactivateAndUndo<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::DeactivateAndUndo(this).into()
            }
        }
        unsafe extern "system" fn OnPosRectChange<Identity: IOleInPlaceSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprcposrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceSite_Impl::OnPosRectChange(this, core::mem::transmute_copy(&lprcposrect)).into()
            }
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            CanInPlaceActivate: CanInPlaceActivate::<Identity, OFFSET>,
            OnInPlaceActivate: OnInPlaceActivate::<Identity, OFFSET>,
            OnUIActivate: OnUIActivate::<Identity, OFFSET>,
            GetWindowContext: GetWindowContext::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
            OnUIDeactivate: OnUIDeactivate::<Identity, OFFSET>,
            OnInPlaceDeactivate: OnInPlaceDeactivate::<Identity, OFFSET>,
            DiscardUndoState: DiscardUndoState::<Identity, OFFSET>,
            DeactivateAndUndo: DeactivateAndUndo::<Identity, OFFSET>,
            OnPosRectChange: OnPosRectChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceSite as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IOleInPlaceSite {}
windows_core::imp::define_interface!(IOleInPlaceUIWindow, IOleInPlaceUIWindow_Vtbl, 0x00000115_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceUIWindow {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceUIWindow, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceUIWindow {
    #[cfg(feature = "windef")]
    pub unsafe fn GetBorder(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: LPCBORDERWIDTHS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestBorderSpace)(windows_core::Interface::as_raw(self), pborderwidths) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: LPCBORDERWIDTHS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBorderSpace)(windows_core::Interface::as_raw(self), pborderwidths) }
    }
    pub unsafe fn SetActiveObject<P0, P1>(&self, pactiveobject: P0, pszobjname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleInPlaceActiveObject>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveObject)(windows_core::Interface::as_raw(self), pactiveobject.param().abi(), pszobjname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceUIWindow_Vtbl {
    pub base__: IOleWindow_Vtbl,
    #[cfg(feature = "windef")]
    pub GetBorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBorder: usize,
    #[cfg(feature = "windef")]
    pub RequestBorderSpace: unsafe extern "system" fn(*mut core::ffi::c_void, LPCBORDERWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestBorderSpace: usize,
    #[cfg(feature = "windef")]
    pub SetBorderSpace: unsafe extern "system" fn(*mut core::ffi::c_void, LPCBORDERWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBorderSpace: usize,
    pub SetActiveObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IOleInPlaceUIWindow_Impl: IOleWindow_Impl {
    fn GetBorder(&self) -> windows_core::Result<super::windef::RECT>;
    fn RequestBorderSpace(&self, pborderwidths: LPCBORDERWIDTHS) -> windows_core::Result<()>;
    fn SetBorderSpace(&self, pborderwidths: LPCBORDERWIDTHS) -> windows_core::Result<()>;
    fn SetActiveObject(&self, pactiveobject: windows_core::Ref<IOleInPlaceActiveObject>, pszobjname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IOleInPlaceUIWindow_Vtbl {
    pub const fn new<Identity: IOleInPlaceUIWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBorder<Identity: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprectborder: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleInPlaceUIWindow_Impl::GetBorder(this) {
                    Ok(ok__) => {
                        lprectborder.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestBorderSpace<Identity: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderwidths: LPCBORDERWIDTHS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceUIWindow_Impl::RequestBorderSpace(this, core::mem::transmute_copy(&pborderwidths)).into()
            }
        }
        unsafe extern "system" fn SetBorderSpace<Identity: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pborderwidths: LPCBORDERWIDTHS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceUIWindow_Impl::SetBorderSpace(this, core::mem::transmute_copy(&pborderwidths)).into()
            }
        }
        unsafe extern "system" fn SetActiveObject<Identity: IOleInPlaceUIWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactiveobject: *mut core::ffi::c_void, pszobjname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleInPlaceUIWindow_Impl::SetActiveObject(this, core::mem::transmute_copy(&pactiveobject), core::mem::transmute(&pszobjname)).into()
            }
        }
        Self {
            base__: IOleWindow_Vtbl::new::<Identity, OFFSET>(),
            GetBorder: GetBorder::<Identity, OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Identity, OFFSET>,
            SetBorderSpace: SetBorderSpace::<Identity, OFFSET>,
            SetActiveObject: SetActiveObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleInPlaceUIWindow as windows_core::Interface>::IID || iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IOleInPlaceUIWindow {}
windows_core::imp::define_interface!(IOleItemContainer, IOleItemContainer_Vtbl, 0x0000011c_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleItemContainer {
    type Target = IOleContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleItemContainer, windows_core::IUnknown, IParseDisplayName, IOleContainer);
impl IOleItemContainer {
    #[cfg(feature = "objidl")]
    pub unsafe fn GetObject<P0, P2, T>(&self, pszitem: P0, dwspeedneeded: u32, pbc: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::objidl::IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pszitem.param().abi(), dwspeedneeded, pbc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetObjectStorage<P0, P1, T>(&self, pszitem: P0, pbc: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::objidl::IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetObjectStorage)(windows_core::Interface::as_raw(self), pszitem.param().abi(), pbc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn IsRunning<P0>(&self, pszitem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pszitem.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleItemContainer_Vtbl {
    pub base__: IOleContainer_Vtbl,
    #[cfg(feature = "objidl")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetObject: usize,
    #[cfg(feature = "objidl")]
    pub GetObjectStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetObjectStorage: usize,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
pub trait IOleItemContainer_Impl: IOleContainer_Impl {
    fn GetObject(&self, pszitem: &windows_core::PCWSTR, dwspeedneeded: u32, pbc: windows_core::Ref<super::objidl::IBindCtx>, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectStorage(&self, pszitem: &windows_core::PCWSTR, pbc: windows_core::Ref<super::objidl::IBindCtx>, riid: *const windows_core::GUID, ppvstorage: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsRunning(&self, pszitem: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl IOleItemContainer_Vtbl {
    pub const fn new<Identity: IOleItemContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObject<Identity: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR, dwspeedneeded: u32, pbc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleItemContainer_Impl::GetObject(this, core::mem::transmute(&pszitem), core::mem::transmute_copy(&dwspeedneeded), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetObjectStorage<Identity: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvstorage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleItemContainer_Impl::GetObjectStorage(this, core::mem::transmute(&pszitem), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvstorage)).into()
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IOleItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleItemContainer_Impl::IsRunning(this, core::mem::transmute(&pszitem)).into()
            }
        }
        Self {
            base__: IOleContainer_Vtbl::new::<Identity, OFFSET>(),
            GetObject: GetObject::<Identity, OFFSET>,
            GetObjectStorage: GetObjectStorage::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleItemContainer as windows_core::Interface>::IID || iid == &<IParseDisplayName as windows_core::Interface>::IID || iid == &<IOleContainer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
impl windows_core::RuntimeName for IOleItemContainer {}
windows_core::imp::define_interface!(IOleLink, IOleLink_Vtbl, 0x0000011d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleLink, windows_core::IUnknown);
impl IOleLink {
    pub unsafe fn SetUpdateOptions(&self, dwupdateopt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUpdateOptions)(windows_core::Interface::as_raw(self), dwupdateopt) }
    }
    pub unsafe fn GetUpdateOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SetSourceMoniker<P0>(&self, pmk: P0, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSourceMoniker)(windows_core::Interface::as_raw(self), pmk.param().abi(), rclsid) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetSourceMoniker(&self) -> windows_core::Result<super::objidl::IMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceMoniker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSourceDisplayName<P0>(&self, pszstatustext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSourceDisplayName)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()) }
    }
    pub unsafe fn GetSourceDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn BindToSource<P1>(&self, bindflags: u32, pbc: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindToSource)(windows_core::Interface::as_raw(self), bindflags, pbc.param().abi()) }
    }
    pub unsafe fn BindIfRunning(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindIfRunning)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBoundSource(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UnbindSource(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnbindSource)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn Update<P0>(&self, pbc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), pbc.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleLink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub SetSourceMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SetSourceMoniker: usize,
    #[cfg(feature = "objidl")]
    pub GetSourceMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetSourceMoniker: usize,
    pub SetSourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub BindToSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    BindToSource: usize,
    pub BindIfRunning: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBoundSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Update: usize,
}
#[cfg(feature = "objidl")]
pub trait IOleLink_Impl: windows_core::IUnknownImpl {
    fn SetUpdateOptions(&self, dwupdateopt: u32) -> windows_core::Result<()>;
    fn GetUpdateOptions(&self) -> windows_core::Result<u32>;
    fn SetSourceMoniker(&self, pmk: windows_core::Ref<super::objidl::IMoniker>, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetSourceMoniker(&self) -> windows_core::Result<super::objidl::IMoniker>;
    fn SetSourceDisplayName(&self, pszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSourceDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn BindToSource(&self, bindflags: u32, pbc: windows_core::Ref<super::objidl::IBindCtx>) -> windows_core::Result<()>;
    fn BindIfRunning(&self) -> windows_core::Result<()>;
    fn GetBoundSource(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn UnbindSource(&self) -> windows_core::Result<()>;
    fn Update(&self, pbc: windows_core::Ref<super::objidl::IBindCtx>) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IOleLink_Vtbl {
    pub const fn new<Identity: IOleLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUpdateOptions<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwupdateopt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::SetUpdateOptions(this, core::mem::transmute_copy(&dwupdateopt)).into()
            }
        }
        unsafe extern "system" fn GetUpdateOptions<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwupdateopt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleLink_Impl::GetUpdateOptions(this) {
                    Ok(ok__) => {
                        pdwupdateopt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceMoniker<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::SetSourceMoniker(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&rclsid)).into()
            }
        }
        unsafe extern "system" fn GetSourceMoniker<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleLink_Impl::GetSourceMoniker(this) {
                    Ok(ok__) => {
                        ppmk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourceDisplayName<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::SetSourceDisplayName(this, core::mem::transmute(&pszstatustext)).into()
            }
        }
        unsafe extern "system" fn GetSourceDisplayName<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleLink_Impl::GetSourceDisplayName(this) {
                    Ok(ok__) => {
                        ppszdisplayname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BindToSource<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bindflags: u32, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::BindToSource(this, core::mem::transmute_copy(&bindflags), core::mem::transmute_copy(&pbc)).into()
            }
        }
        unsafe extern "system" fn BindIfRunning<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::BindIfRunning(this).into()
            }
        }
        unsafe extern "system" fn GetBoundSource<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleLink_Impl::GetBoundSource(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnbindSource<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::UnbindSource(this).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IOleLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleLink_Impl::Update(this, core::mem::transmute_copy(&pbc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUpdateOptions: SetUpdateOptions::<Identity, OFFSET>,
            GetUpdateOptions: GetUpdateOptions::<Identity, OFFSET>,
            SetSourceMoniker: SetSourceMoniker::<Identity, OFFSET>,
            GetSourceMoniker: GetSourceMoniker::<Identity, OFFSET>,
            SetSourceDisplayName: SetSourceDisplayName::<Identity, OFFSET>,
            GetSourceDisplayName: GetSourceDisplayName::<Identity, OFFSET>,
            BindToSource: BindToSource::<Identity, OFFSET>,
            BindIfRunning: BindIfRunning::<Identity, OFFSET>,
            GetBoundSource: GetBoundSource::<Identity, OFFSET>,
            UnbindSource: UnbindSource::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleLink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IOleLink {}
windows_core::imp::define_interface!(IOleObject, IOleObject_Vtbl, 0x00000112_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleObject, windows_core::IUnknown);
impl IOleObject {
    pub unsafe fn SetClientSite<P0>(&self, pclientsite: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleClientSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClientSite)(windows_core::Interface::as_raw(self), pclientsite.param().abi()) }
    }
    pub unsafe fn GetClientSite(&self) -> windows_core::Result<IOleClientSite> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, szcontainerapp: P0, szcontainerobj: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHostNames)(windows_core::Interface::as_raw(self), szcontainerapp.param().abi(), szcontainerobj.param().abi()) }
    }
    pub unsafe fn Close(&self, dwsaveoption: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), dwsaveoption) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SetMoniker<P1>(&self, dwwhichmoniker: u32, pmk: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMoniker)(windows_core::Interface::as_raw(self), dwwhichmoniker, pmk.param().abi()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> windows_core::Result<super::objidl::IMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), dwassign, dwwhichmoniker, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn InitFromData<P0>(&self, pdataobject: P0, fcreation: bool, dwreserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitFromData)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), fcreation.into(), dwreserved) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetClipboardData(&self, dwreserved: u32) -> windows_core::Result<super::objidl::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn DoVerb<P2>(&self, iverb: i32, lpmsg: *const super::winuser::MSG, pactivesite: P2, lindex: i32, hwndparent: super::windef::HWND, lprcposrect: *const super::windef::RECT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IOleClientSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoVerb)(windows_core::Interface::as_raw(self), iverb, lpmsg, pactivesite.param().abi(), lindex, hwndparent, lprcposrect) }
    }
    pub unsafe fn EnumVerbs(&self) -> windows_core::Result<IEnumOLEVERB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumVerbs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Update(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsUpToDate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsUpToDate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetUserClassID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUserType(&self, dwformoftype: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserType)(windows_core::Interface::as_raw(self), dwformoftype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetExtent(&self, dwdrawaspect: u32, psizel: *const super::windef::SIZEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, psizel) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetExtent(&self, dwdrawaspect: u32) -> windows_core::Result<super::windef::SIZEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn Advise<P0>(&self, padvsink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::objidl::IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), padvsink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMiscStatus(&self, dwaspect: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMiscStatus)(windows_core::Interface::as_raw(self), dwaspect, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn SetColorScheme(&self, plogpal: *const super::wingdi::LOGPALETTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorScheme)(windows_core::Interface::as_raw(self), plogpal) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub SetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SetMoniker: usize,
    #[cfg(feature = "objidl")]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetMoniker: usize,
    #[cfg(feature = "objidl")]
    pub InitFromData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    InitFromData: usize,
    #[cfg(feature = "objidl")]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub DoVerb: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::winuser::MSG, *mut core::ffi::c_void, i32, super::windef::HWND, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    DoVerb: usize,
    pub EnumVerbs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsUpToDate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetUserType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::windef::SIZEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetExtent: usize,
    #[cfg(feature = "windef")]
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::SIZEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetExtent: usize,
    #[cfg(feature = "objidl")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    EnumAdvise: usize,
    pub GetMiscStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub SetColorScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::LOGPALETTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    SetColorScheme: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "windef", feature = "wingdi", feature = "winuser"))]
pub trait IOleObject_Impl: windows_core::IUnknownImpl {
    fn SetClientSite(&self, pclientsite: windows_core::Ref<IOleClientSite>) -> windows_core::Result<()>;
    fn GetClientSite(&self) -> windows_core::Result<IOleClientSite>;
    fn SetHostNames(&self, szcontainerapp: &windows_core::PCWSTR, szcontainerobj: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self, dwsaveoption: u32) -> windows_core::Result<()>;
    fn SetMoniker(&self, dwwhichmoniker: u32, pmk: windows_core::Ref<super::objidl::IMoniker>) -> windows_core::Result<()>;
    fn GetMoniker(&self, dwassign: u32, dwwhichmoniker: u32) -> windows_core::Result<super::objidl::IMoniker>;
    fn InitFromData(&self, pdataobject: windows_core::Ref<super::objidl::IDataObject>, fcreation: windows_core::BOOL, dwreserved: u32) -> windows_core::Result<()>;
    fn GetClipboardData(&self, dwreserved: u32) -> windows_core::Result<super::objidl::IDataObject>;
    fn DoVerb(&self, iverb: i32, lpmsg: *const super::winuser::MSG, pactivesite: windows_core::Ref<IOleClientSite>, lindex: i32, hwndparent: super::windef::HWND, lprcposrect: *const super::windef::RECT) -> windows_core::Result<()>;
    fn EnumVerbs(&self) -> windows_core::Result<IEnumOLEVERB>;
    fn Update(&self) -> windows_core::Result<()>;
    fn IsUpToDate(&self) -> windows_core::Result<()>;
    fn GetUserClassID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetUserType(&self, dwformoftype: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn SetExtent(&self, dwdrawaspect: u32, psizel: *const super::windef::SIZEL) -> windows_core::Result<()>;
    fn GetExtent(&self, dwdrawaspect: u32) -> windows_core::Result<super::windef::SIZEL>;
    fn Advise(&self, padvsink: windows_core::Ref<super::objidl::IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<super::objidl::IEnumSTATDATA>;
    fn GetMiscStatus(&self, dwaspect: u32) -> windows_core::Result<u32>;
    fn SetColorScheme(&self, plogpal: *const super::wingdi::LOGPALETTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "windef", feature = "wingdi", feature = "winuser"))]
impl IOleObject_Vtbl {
    pub const fn new<Identity: IOleObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetClientSite<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientsite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::SetClientSite(this, core::mem::transmute_copy(&pclientsite)).into()
            }
        }
        unsafe extern "system" fn GetClientSite<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclientsite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetClientSite(this) {
                    Ok(ok__) => {
                        ppclientsite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHostNames<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcontainerapp: windows_core::PCWSTR, szcontainerobj: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::SetHostNames(this, core::mem::transmute(&szcontainerapp), core::mem::transmute(&szcontainerobj)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsaveoption: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::Close(this, core::mem::transmute_copy(&dwsaveoption)).into()
            }
        }
        unsafe extern "system" fn SetMoniker<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhichmoniker: u32, pmk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::SetMoniker(this, core::mem::transmute_copy(&dwwhichmoniker), core::mem::transmute_copy(&pmk)).into()
            }
        }
        unsafe extern "system" fn GetMoniker<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetMoniker(this, core::mem::transmute_copy(&dwassign), core::mem::transmute_copy(&dwwhichmoniker)) {
                    Ok(ok__) => {
                        ppmk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitFromData<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, fcreation: windows_core::BOOL, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::InitFromData(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&fcreation), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetClipboardData<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetClipboardData(this, core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoVerb<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iverb: i32, lpmsg: *const super::winuser::MSG, pactivesite: *mut core::ffi::c_void, lindex: i32, hwndparent: super::windef::HWND, lprcposrect: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::DoVerb(this, core::mem::transmute_copy(&iverb), core::mem::transmute_copy(&lpmsg), core::mem::transmute_copy(&pactivesite), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lprcposrect)).into()
            }
        }
        unsafe extern "system" fn EnumVerbs<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumoleverb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::EnumVerbs(this) {
                    Ok(ok__) => {
                        ppenumoleverb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Update<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::Update(this).into()
            }
        }
        unsafe extern "system" fn IsUpToDate<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::IsUpToDate(this).into()
            }
        }
        unsafe extern "system" fn GetUserClassID<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetUserClassID(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserType<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwformoftype: u32, pszusertype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetUserType(this, core::mem::transmute_copy(&dwformoftype)) {
                    Ok(ok__) => {
                        pszusertype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtent<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, psizel: *const super::windef::SIZEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::SetExtent(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&psizel)).into()
            }
        }
        unsafe extern "system" fn GetExtent<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, psizel: *mut super::windef::SIZEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetExtent(this, core::mem::transmute_copy(&dwdrawaspect)) {
                    Ok(ok__) => {
                        psizel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvsink: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::Advise(this, core::mem::transmute_copy(&padvsink)) {
                    Ok(ok__) => {
                        pdwconnection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumAdvise<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::EnumAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMiscStatus<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, pdwstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleObject_Impl::GetMiscStatus(this, core::mem::transmute_copy(&dwaspect)) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorScheme<Identity: IOleObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogpal: *const super::wingdi::LOGPALETTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleObject_Impl::SetColorScheme(this, core::mem::transmute_copy(&plogpal)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientSite: SetClientSite::<Identity, OFFSET>,
            GetClientSite: GetClientSite::<Identity, OFFSET>,
            SetHostNames: SetHostNames::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetMoniker: SetMoniker::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            InitFromData: InitFromData::<Identity, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, OFFSET>,
            DoVerb: DoVerb::<Identity, OFFSET>,
            EnumVerbs: EnumVerbs::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            IsUpToDate: IsUpToDate::<Identity, OFFSET>,
            GetUserClassID: GetUserClassID::<Identity, OFFSET>,
            GetUserType: GetUserType::<Identity, OFFSET>,
            SetExtent: SetExtent::<Identity, OFFSET>,
            GetExtent: GetExtent::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            GetMiscStatus: GetMiscStatus::<Identity, OFFSET>,
            SetColorScheme: SetColorScheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "windef", feature = "wingdi", feature = "winuser"))]
impl windows_core::RuntimeName for IOleObject {}
windows_core::imp::define_interface!(IOleWindow, IOleWindow_Vtbl, 0x00000114_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IOleWindow, windows_core::IUnknown);
impl IOleWindow {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ContextSensitiveHelp(&self, fentermode: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IOleWindow_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn ContextSensitiveHelp(&self, fentermode: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IOleWindow_Vtbl {
    pub const fn new<Identity: IOleWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IOleWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOleWindow_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: IOleWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fentermode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOleWindow_Impl::ContextSensitiveHelp(this, core::mem::transmute_copy(&fentermode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOleWindow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IOleWindow {}
windows_core::imp::define_interface!(IParseDisplayName, IParseDisplayName_Vtbl, 0x0000011a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IParseDisplayName, windows_core::IUnknown);
impl IParseDisplayName {
    #[cfg(feature = "objidl")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut Option<super::objidl::IMoniker>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IBindCtx>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pszdisplayname.param().abi(), pcheaten as _, core::mem::transmute(ppmkout)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IParseDisplayName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidl")]
    pub ParseDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    ParseDisplayName: usize,
}
#[cfg(feature = "objidl")]
pub trait IParseDisplayName_Impl: windows_core::IUnknownImpl {
    fn ParseDisplayName(&self, pbc: windows_core::Ref<super::objidl::IBindCtx>, pszdisplayname: &windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: windows_core::OutRef<super::objidl::IMoniker>) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IParseDisplayName_Vtbl {
    pub const fn new<Identity: IParseDisplayName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseDisplayName<Identity: IParseDisplayName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pszdisplayname: windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IParseDisplayName_Impl::ParseDisplayName(this, core::mem::transmute_copy(&pbc), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&ppmkout)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseDisplayName: ParseDisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IParseDisplayName as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IParseDisplayName {}
windows_core::imp::define_interface!(IViewObject, IViewObject_Vtbl, 0x0000010d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IViewObject, windows_core::IUnknown);
impl IViewObject {
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn Draw(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: Option<*const super::objidl::DVTARGETDEVICE>, hdctargetdev: Option<super::windef::HDC>, hdcdraw: super::windef::HDC, lprcbounds: Option<*const super::windef::RECTL>, lprcwbounds: Option<*const super::windef::RECTL>, pfncontinue: Option<*const u8>, dwcontinue: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd.unwrap_or(core::mem::zeroed()) as _, hdctargetdev.unwrap_or(core::mem::zeroed()) as _, hdcdraw, lprcbounds.unwrap_or(core::mem::zeroed()) as _, lprcwbounds.unwrap_or(core::mem::zeroed()) as _, pfncontinue.unwrap_or(core::mem::zeroed()) as _, dwcontinue) }
    }
    #[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
    pub unsafe fn GetColorSet(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: Option<*const super::objidl::DVTARGETDEVICE>, hictargetdev: Option<super::windef::HDC>) -> windows_core::Result<*mut super::wingdi::LOGPALETTE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorSet)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd.unwrap_or(core::mem::zeroed()) as _, hictargetdev.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Freeze(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unfreeze)(windows_core::Interface::as_raw(self), dwfreeze) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SetAdvise<P2>(&self, aspects: u32, advf: u32, padvsink: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::objidl::IAdviseSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdvise)(windows_core::Interface::as_raw(self), aspects, advf, padvsink.param().abi()) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetAdvise(&self, paspects: Option<*mut u32>, padvf: Option<*mut u32>, ppadvsink: *mut Option<super::objidl::IAdviseSink>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdvise)(windows_core::Interface::as_raw(self), paspects.unwrap_or(core::mem::zeroed()) as _, padvf.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppadvsink)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const core::ffi::c_void, *const super::objidl::DVTARGETDEVICE, super::windef::HDC, super::windef::HDC, *const super::windef::RECTL, *const super::windef::RECTL, *const u8, usize) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    Draw: usize,
    #[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
    pub GetColorSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const core::ffi::c_void, *const super::objidl::DVTARGETDEVICE, super::windef::HDC, *mut *mut super::wingdi::LOGPALETTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef", feature = "wingdi")))]
    GetColorSet: usize,
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unfreeze: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub SetAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SetAdvise: usize,
    #[cfg(feature = "objidl")]
    pub GetAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetAdvise: usize,
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
pub trait IViewObject_Impl: windows_core::IUnknownImpl {
    fn Draw(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: *const super::objidl::DVTARGETDEVICE, hdctargetdev: super::windef::HDC, hdcdraw: super::windef::HDC, lprcbounds: *const super::windef::RECTL, lprcwbounds: *const super::windef::RECTL, pfncontinue: *const u8, dwcontinue: usize) -> windows_core::Result<()>;
    fn GetColorSet(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: *const super::objidl::DVTARGETDEVICE, hictargetdev: super::windef::HDC) -> windows_core::Result<*mut super::wingdi::LOGPALETTE>;
    fn Freeze(&self, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn Unfreeze(&self, dwfreeze: u32) -> windows_core::Result<()>;
    fn SetAdvise(&self, aspects: u32, advf: u32, padvsink: windows_core::Ref<super::objidl::IAdviseSink>) -> windows_core::Result<()>;
    fn GetAdvise(&self, paspects: *mut u32, padvf: *mut u32, ppadvsink: windows_core::OutRef<super::objidl::IAdviseSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
impl IViewObject_Vtbl {
    pub const fn new<Identity: IViewObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Draw<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: *const super::objidl::DVTARGETDEVICE, hdctargetdev: super::windef::HDC, hdcdraw: super::windef::HDC, lprcbounds: *const super::windef::RECTL, lprcwbounds: *const super::windef::RECTL, pfncontinue: *const u8, dwcontinue: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObject_Impl::Draw(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hdctargetdev), core::mem::transmute_copy(&hdcdraw), core::mem::transmute_copy(&lprcbounds), core::mem::transmute_copy(&lprcwbounds), core::mem::transmute_copy(&pfncontinue), core::mem::transmute_copy(&dwcontinue)).into()
            }
        }
        unsafe extern "system" fn GetColorSet<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, ptd: *const super::objidl::DVTARGETDEVICE, hictargetdev: super::windef::HDC, ppcolorset: *mut *mut super::wingdi::LOGPALETTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObject_Impl::GetColorSet(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect), core::mem::transmute_copy(&ptd), core::mem::transmute_copy(&hictargetdev)) {
                    Ok(ok__) => {
                        ppcolorset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Freeze<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, pvaspect: *const core::ffi::c_void, pdwfreeze: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObject_Impl::Freeze(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvaspect)) {
                    Ok(ok__) => {
                        pdwfreeze.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unfreeze<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfreeze: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObject_Impl::Unfreeze(this, core::mem::transmute_copy(&dwfreeze)).into()
            }
        }
        unsafe extern "system" fn SetAdvise<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aspects: u32, advf: u32, padvsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObject_Impl::SetAdvise(this, core::mem::transmute_copy(&aspects), core::mem::transmute_copy(&advf), core::mem::transmute_copy(&padvsink)).into()
            }
        }
        unsafe extern "system" fn GetAdvise<Identity: IViewObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObject_Impl::GetAdvise(this, core::mem::transmute_copy(&paspects), core::mem::transmute_copy(&padvf), core::mem::transmute_copy(&ppadvsink)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, OFFSET>,
            GetColorSet: GetColorSet::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            Unfreeze: Unfreeze::<Identity, OFFSET>,
            SetAdvise: SetAdvise::<Identity, OFFSET>,
            GetAdvise: GetAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IViewObject {}
windows_core::imp::define_interface!(IViewObject2, IViewObject2_Vtbl, 0x00000127_0000_0000_c000_000000000046);
impl core::ops::Deref for IViewObject2 {
    type Target = IViewObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObject2, windows_core::IUnknown, IViewObject);
impl IViewObject2 {
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub unsafe fn GetExtent(&self, dwdrawaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE) -> windows_core::Result<super::windef::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, ptd, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject2_Vtbl {
    pub base__: IViewObject_Vtbl,
    #[cfg(all(feature = "objidl", feature = "windef"))]
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const super::objidl::DVTARGETDEVICE, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "windef")))]
    GetExtent: usize,
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
pub trait IViewObject2_Impl: IViewObject_Impl {
    fn GetExtent(&self, dwdrawaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE) -> windows_core::Result<super::windef::SIZE>;
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
impl IViewObject2_Vtbl {
    pub const fn new<Identity: IViewObject2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtent<Identity: IViewObject2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdrawaspect: u32, lindex: i32, ptd: *const super::objidl::DVTARGETDEVICE, lpsizel: *mut super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObject2_Impl::GetExtent(this, core::mem::transmute_copy(&dwdrawaspect), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ptd)) {
                    Ok(ok__) => {
                        lpsizel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IViewObject_Vtbl::new::<Identity, OFFSET>(), GetExtent: GetExtent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObject2 as windows_core::Interface>::IID || iid == &<IViewObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IViewObject2 {}
#[cfg(feature = "windef")]
pub type LINKSRCDESCRIPTOR = OBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type LPBORDERWIDTHS = super::windef::LPRECT;
#[cfg(feature = "windef")]
pub type LPCBORDERWIDTHS = super::windef::LPCRECT;
#[cfg(feature = "windef")]
pub type LPLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type LPOBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type LPOLEINPLACEFRAMEINFO = *mut OLEINPLACEFRAMEINFO;
pub type LPOLEMENUGROUPWIDTHS = *mut OLEMENUGROUPWIDTHS;
pub type LPOLERENDER = *mut OLERENDER;
pub type LPOLEUPDATE = *mut OLEUPDATE;
pub type LPOLEVERB = *mut OLEVERB;
pub const MK_ALT: u32 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: windows_core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::windef::SIZEL,
    pub pointl: super::windef::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
pub type OLECLOSE = i32;
pub const OLECLOSE_NOSAVE: OLECLOSE = 1;
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = 2;
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = 0;
pub type OLECONTF = i32;
pub const OLECONTF_EMBEDDINGS: OLECONTF = 1;
pub const OLECONTF_LINKS: OLECONTF = 2;
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = 16;
pub const OLECONTF_ONLYUSER: OLECONTF = 8;
pub const OLECONTF_OTHERS: OLECONTF = 4;
pub type OLEGETMONIKER = i32;
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = 2;
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = 1;
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = 4;
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: windows_core::BOOL,
    pub hwndFrame: super::windef::HWND,
    pub haccel: super::windef::HACCEL,
    pub cAccelEntries: u32,
}
pub type OLELINKBIND = i32;
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OLEMENUGROUPWIDTHS {
    pub width: [i32; 6],
}
impl Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLEMISC = i32;
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = 256;
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = 4096;
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = 8192;
pub const OLEMISC_ALIGNABLE: OLEMISC = 32768;
pub const OLEMISC_ALWAYSRUN: OLEMISC = 2048;
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = 32;
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = 16;
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = 524288;
pub const OLEMISC_IMEMODE: OLEMISC = 262144;
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = 4;
pub const OLEMISC_INSIDEOUT: OLEMISC = 128;
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = 1024;
pub const OLEMISC_ISLINKOBJECT: OLEMISC = 64;
pub const OLEMISC_NOUIACTIVATE: OLEMISC = 16384;
pub const OLEMISC_ONLYICONIC: OLEMISC = 2;
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = 1;
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = 512;
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = 131072;
pub const OLEMISC_SIMPLEFRAME: OLEMISC = 65536;
pub const OLEMISC_STATIC: OLEMISC = 8;
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = 2097152;
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = 1048576;
pub type OLERENDER = i32;
pub const OLERENDER_ASIS: OLERENDER = 3;
pub const OLERENDER_DRAW: OLERENDER = 1;
pub const OLERENDER_FORMAT: OLERENDER = 2;
pub const OLERENDER_NONE: OLERENDER = 0;
pub type OLEUPDATE = i32;
pub const OLEUPDATE_ALWAYS: OLEUPDATE = 1;
pub const OLEUPDATE_ONCALL: OLEUPDATE = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OLEVERB {
    pub lVerb: i32,
    pub lpszVerbName: windows_core::PWSTR,
    pub fuFlags: u32,
    pub grfAttribs: u32,
}
pub type OLEVERBATTRIB = i32;
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = 1;
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = 2;
pub type OLEWHICHMK = i32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = 1;
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = 3;
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = 2;
#[cfg(feature = "windef")]
pub type PLINKSRCDESCRIPTOR = *mut OBJECTDESCRIPTOR;
#[cfg(feature = "windef")]
pub type POBJECTDESCRIPTOR = *mut OBJECTDESCRIPTOR;
pub type POLEUPDATE = *mut OLEUPDATE;
pub const UPDFCACHE_ALL: u32 = 2147483647;
pub const UPDFCACHE_ALLBUTNODATACACHE: u32 = 2147483646;
pub const UPDFCACHE_IFBLANK: u32 = 16;
pub const UPDFCACHE_IFBLANKORONSAVECACHE: u32 = 18;
pub const UPDFCACHE_NODATACACHE: u32 = 1;
pub const UPDFCACHE_NORMALCACHE: u32 = 8;
pub const UPDFCACHE_ONLYIFBLANK: u32 = 2147483648;
pub const UPDFCACHE_ONSAVECACHE: u32 = 2;
pub const UPDFCACHE_ONSTOPCACHE: u32 = 4;
pub type USERCLASSTYPE = i32;
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = 3;
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = 1;
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = 2;
