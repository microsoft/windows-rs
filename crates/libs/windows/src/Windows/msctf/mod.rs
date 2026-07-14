windows_core::imp::define_interface!(IEnumITfCompositionView, IEnumITfCompositionView_Vtbl, 0x5efd22ba_7838_46cb_88e2_cadb14124f8f);
windows_core::imp::interface_hierarchy!(IEnumITfCompositionView, windows_core::IUnknown);
impl IEnumITfCompositionView {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgcompositionview: *mut Option<ITfCompositionView>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(rgcompositionview), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumITfCompositionView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumITfCompositionView_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn Next(&self, ulcount: u32, rgcompositionview: windows_core::OutRef<ITfCompositionView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumITfCompositionView_Vtbl {
    pub const fn new<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumITfCompositionView_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumITfCompositionView_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcompositionview), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumITfCompositionView_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumITfCompositionView_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumITfCompositionView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumITfCompositionView {}
windows_core::imp::define_interface!(IEnumTfContextViews, IEnumTfContextViews_Vtbl, 0xf0c0f8dd_cf38_44e1_bb0f_68cf0d551c78);
windows_core::imp::interface_hierarchy!(IEnumTfContextViews, windows_core::IUnknown);
impl IEnumTfContextViews {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgviews: *mut Option<ITfContextView>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(rgviews), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContextViews_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfContextViews_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn Next(&self, ulcount: u32, rgviews: windows_core::OutRef<ITfContextView>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfContextViews_Vtbl {
    pub const fn new<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfContextViews_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgviews: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContextViews_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgviews), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContextViews_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfContextViews_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContextViews_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfContextViews as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfContextViews {}
windows_core::imp::define_interface!(IEnumTfContexts, IEnumTfContexts_Vtbl, 0x8f1a7ea6_1654_4502_a86e_b2902344d507);
windows_core::imp::interface_hierarchy!(IEnumTfContexts, windows_core::IUnknown);
impl IEnumTfContexts {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgcontext: *mut Option<ITfContext>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(rgcontext), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContexts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfContexts_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfContexts>;
    fn Next(&self, ulcount: u32, rgcontext: windows_core::OutRef<ITfContext>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfContexts_Vtbl {
    pub const fn new<Identity: IEnumTfContexts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfContexts_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContexts_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgcontext), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContexts_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfContexts_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfContexts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfContexts {}
windows_core::imp::define_interface!(IEnumTfDisplayAttributeInfo, IEnumTfDisplayAttributeInfo_Vtbl, 0x7cef04d7_cb75_4e80_a7ab_5f5bc7d332de);
windows_core::imp::interface_hierarchy!(IEnumTfDisplayAttributeInfo, windows_core::IUnknown);
impl IEnumTfDisplayAttributeInfo {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rginfo: *mut Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(rginfo), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDisplayAttributeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfDisplayAttributeInfo_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(&self, ulcount: u32, rginfo: windows_core::OutRef<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfDisplayAttributeInfo_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rginfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDisplayAttributeInfo_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rginfo), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDisplayAttributeInfo_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDisplayAttributeInfo_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfDisplayAttributeInfo {}
windows_core::imp::define_interface!(IEnumTfDocumentMgrs, IEnumTfDocumentMgrs_Vtbl, 0xaa80e808_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(IEnumTfDocumentMgrs, windows_core::IUnknown);
impl IEnumTfDocumentMgrs {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgdocumentmgr: *mut Option<ITfDocumentMgr>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(rgdocumentmgr), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDocumentMgrs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfDocumentMgrs_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn Next(&self, ulcount: u32, rgdocumentmgr: windows_core::OutRef<ITfDocumentMgr>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfDocumentMgrs_Vtbl {
    pub const fn new<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfDocumentMgrs_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDocumentMgrs_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgdocumentmgr), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDocumentMgrs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfDocumentMgrs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfDocumentMgrs_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfDocumentMgrs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfDocumentMgrs {}
windows_core::imp::define_interface!(IEnumTfFunctionProviders, IEnumTfFunctionProviders_Vtbl, 0xe4b24db0_0990_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfFunctionProviders, windows_core::IUnknown);
impl IEnumTfFunctionProviders {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, ppcmdobj: *mut Option<ITfFunctionProvider>, pcfetch: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppcmdobj), pcfetch as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfFunctionProviders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfFunctionProviders_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn Next(&self, ulcount: u32, ppcmdobj: windows_core::OutRef<ITfFunctionProvider>, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfFunctionProviders_Vtbl {
    pub const fn new<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfFunctionProviders_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut core::ffi::c_void, pcfetch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfFunctionProviders_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppcmdobj), core::mem::transmute_copy(&pcfetch)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfFunctionProviders_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfFunctionProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfFunctionProviders_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfFunctionProviders as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfFunctionProviders {}
windows_core::imp::define_interface!(IEnumTfInputProcessorProfiles, IEnumTfInputProcessorProfiles_Vtbl, 0x71c6e74d_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(IEnumTfInputProcessorProfiles, windows_core::IUnknown);
impl IEnumTfInputProcessorProfiles {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, pprofile as _, pcfetch as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfInputProcessorProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_INPUTPROCESSORPROFILE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub trait IEnumTfInputProcessorProfiles_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl IEnumTfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfInputProcessorProfiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfInputProcessorProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfInputProcessorProfiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfInputProcessorProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for IEnumTfInputProcessorProfiles {}
windows_core::imp::define_interface!(IEnumTfLanguageProfiles, IEnumTfLanguageProfiles_Vtbl, 0x3d61bf11_ac5f_42c8_a4cb_931bcc28c744);
windows_core::imp::interface_hierarchy!(IEnumTfLanguageProfiles, windows_core::IUnknown);
impl IEnumTfLanguageProfiles {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, pprofile as _, pcfetch as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLanguageProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_LANGUAGEPROFILE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IEnumTfLanguageProfiles_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IEnumTfLanguageProfiles_Vtbl {
    pub const fn new<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfLanguageProfiles_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLanguageProfiles_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprofile), core::mem::transmute_copy(&pcfetch)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLanguageProfiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfLanguageProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfLanguageProfiles_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfLanguageProfiles as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IEnumTfLanguageProfiles {}
windows_core::imp::define_interface!(IEnumTfProperties, IEnumTfProperties_Vtbl, 0x19188cb0_aca9_11d2_afc5_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfProperties, windows_core::IUnknown);
impl IEnumTfProperties {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, ppprop: *mut Option<ITfProperty>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppprop), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfProperties_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfProperties>;
    fn Next(&self, ulcount: u32, ppprop: windows_core::OutRef<ITfProperty>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfProperties_Vtbl {
    pub const fn new<Identity: IEnumTfProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfProperties_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppprop: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfProperties_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppprop), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfProperties_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfProperties_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfProperties {}
windows_core::imp::define_interface!(IEnumTfPropertyValue, IEnumTfPropertyValue_Vtbl, 0x8ed8981b_7c10_4d7d_9fb3_ab72e9c75f72);
windows_core::imp::interface_hierarchy!(IEnumTfPropertyValue, windows_core::IUnknown);
impl IEnumTfPropertyValue {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rgvalues, pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfPropertyValue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TF_PROPERTYVAL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEnumTfPropertyValue_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfPropertyValue>;
    fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IEnumTfPropertyValue_Vtbl {
    pub const fn new<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfPropertyValue_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfPropertyValue_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgvalues), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfPropertyValue_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfPropertyValue_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfPropertyValue as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumTfPropertyValue {}
windows_core::imp::define_interface!(IEnumTfRanges, IEnumTfRanges_Vtbl, 0xf99d3f40_8e32_11d2_bf46_00105a2799b5);
windows_core::imp::interface_hierarchy!(IEnumTfRanges, windows_core::IUnknown);
impl IEnumTfRanges {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, pprange: *mut Option<ITfRange>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(pprange), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfRanges_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfRanges_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfRanges>;
    fn Next(&self, ulcount: u32, pprange: windows_core::OutRef<ITfRange>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfRanges_Vtbl {
    pub const fn new<Identity: IEnumTfRanges_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfRanges_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, pprange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfRanges_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfRanges_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfRanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfRanges_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfRanges as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfRanges {}
windows_core::imp::define_interface!(IEnumTfUIElements, IEnumTfUIElements_Vtbl, 0x887aa91e_acba_4931_84da_3c5208cf543f);
windows_core::imp::interface_hierarchy!(IEnumTfUIElements, windows_core::IUnknown);
impl IEnumTfUIElements {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, ppelement: *mut Option<ITfUIElement>, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, core::mem::transmute(ppelement), pcfetched as _) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfUIElements_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumTfUIElements_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumTfUIElements>;
    fn Next(&self, ulcount: u32, ppelement: windows_core::OutRef<ITfUIElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumTfUIElements_Vtbl {
    pub const fn new<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumTfUIElements_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, ppelement: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfUIElements_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&ppelement), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfUIElements_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumTfUIElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumTfUIElements_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
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
        iid == &<IEnumTfUIElements as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumTfUIElements {}
windows_core::imp::define_interface!(ITextStoreACPServices, ITextStoreACPServices_Vtbl, 0xaa80e901_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITextStoreACPServices, windows_core::IUnknown);
impl ITextStoreACPServices {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Serialize<P0, P1, P3>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), prange.param().abi(), phdr as _, pstream.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Unserialize<P0, P2, P3>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2, ploader: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
        P2: windows_core::Param<super::objidlbase::IStream>,
        P3: windows_core::Param<ITfPersistentPropertyLoaderACP>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unserialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), phdr, pstream.param().abi(), ploader.param().abi()) }
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
    {
        unsafe { (windows_core::Interface::vtable(self).ForceLoadProperty)(windows_core::Interface::as_raw(self), pprop.param().abi()) }
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRange)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Serialize: usize,
    #[cfg(feature = "objidlbase")]
    pub Unserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait ITextStoreACPServices_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pprop: windows_core::Ref<ITfProperty>, prange: windows_core::Ref<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: windows_core::Ref<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: windows_core::Ref<super::objidlbase::IStream>, ploader: windows_core::Ref<ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: windows_core::Ref<ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "objidlbase")]
impl ITextStoreACPServices_Vtbl {
    pub const fn new<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPServices_Impl::Serialize(this, core::mem::transmute_copy(&pprop), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&phdr), core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn Unserialize<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPServices_Impl::Unserialize(this, core::mem::transmute_copy(&pprop), core::mem::transmute_copy(&phdr), core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&ploader)).into()
            }
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextStoreACPServices_Impl::ForceLoadProperty(this, core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn CreateRange<Identity: ITextStoreACPServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextStoreACPServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextStoreACPServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ITextStoreACPServices {}
windows_core::imp::define_interface!(ITfActiveLanguageProfileNotifySink, ITfActiveLanguageProfileNotifySink_Vtbl, 0xb246cb75_a93e_4652_bf8c_b3fe0cfd7e57);
windows_core::imp::interface_hierarchy!(ITfActiveLanguageProfileNotifySink, windows_core::IUnknown);
impl ITfActiveLanguageProfileNotifySink {
    pub unsafe fn OnActivated(&self, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnActivated)(windows_core::Interface::as_raw(self), clsid, guidprofile, factivated.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfActiveLanguageProfileNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITfActiveLanguageProfileNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnActivated(&self, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ITfActiveLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivated<Identity: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, factivated: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfActiveLanguageProfileNotifySink_Impl::OnActivated(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&factivated)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfActiveLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfActiveLanguageProfileNotifySink {}
windows_core::imp::define_interface!(ITfCandidateListUIElement, ITfCandidateListUIElement_Vtbl, 0xea1ea138_19df_11d7_a6d2_00065b84435c);
impl core::ops::Deref for ITfCandidateListUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfCandidateListUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfCandidateListUIElement {
    pub unsafe fn GetUpdatedFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdatedFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPageIndex)(windows_core::Interface::as_raw(self), pindex as _, usize, pupagecnt as _) }
    }
    pub unsafe fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPageIndex)(windows_core::Interface::as_raw(self), pindex, upagecnt) }
    }
    pub unsafe fn GetCurrentPage(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetPageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub GetCurrentPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITfCandidateListUIElement_Impl: ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::Result<()>;
    fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> windows_core::Result<()>;
    fn GetCurrentPage(&self) -> windows_core::Result<u32>;
}
impl ITfCandidateListUIElement_Vtbl {
    pub const fn new<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetUpdatedFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetDocumentMgr(this) {
                    Ok(ok__) => {
                        ppdim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        puindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPageIndex<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCandidateListUIElement_Impl::GetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&usize), core::mem::transmute_copy(&pupagecnt)).into()
            }
        }
        unsafe extern "system" fn SetPageIndex<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCandidateListUIElement_Impl::SetPageIndex(this, core::mem::transmute_copy(&pindex), core::mem::transmute_copy(&upagecnt)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPage<Identity: ITfCandidateListUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCandidateListUIElement_Impl::GetCurrentPage(this) {
                    Ok(ok__) => {
                        pupage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetPageIndex: GetPageIndex::<Identity, OFFSET>,
            SetPageIndex: SetPageIndex::<Identity, OFFSET>,
            GetCurrentPage: GetCurrentPage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateListUIElement {}
windows_core::imp::define_interface!(ITfCandidateListUIElementBehavior, ITfCandidateListUIElementBehavior_Vtbl, 0x85fad185_58ce_497a_9460_355366b64b9a);
impl core::ops::Deref for ITfCandidateListUIElementBehavior {
    type Target = ITfCandidateListUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfCandidateListUIElementBehavior, windows_core::IUnknown, ITfUIElement, ITfCandidateListUIElement);
impl ITfCandidateListUIElementBehavior {
    pub unsafe fn SetSelection(&self, nindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nindex) }
    }
    pub unsafe fn Finalize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElementBehavior_Vtbl {
    pub base__: ITfCandidateListUIElement_Vtbl,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCandidateListUIElementBehavior_Impl: ITfCandidateListUIElement_Impl {
    fn SetSelection(&self, nindex: u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl ITfCandidateListUIElementBehavior_Vtbl {
    pub const fn new<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSelection<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCandidateListUIElementBehavior_Impl::SetSelection(this, core::mem::transmute_copy(&nindex)).into()
            }
        }
        unsafe extern "system" fn Finalize<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCandidateListUIElementBehavior_Impl::Finalize(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITfCandidateListUIElementBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCandidateListUIElementBehavior_Impl::Abort(this).into()
            }
        }
        Self {
            base__: ITfCandidateListUIElement_Vtbl::new::<Identity, OFFSET>(),
            SetSelection: SetSelection::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCandidateListUIElementBehavior as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID || iid == &<ITfCandidateListUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCandidateListUIElementBehavior {}
windows_core::imp::define_interface!(ITfCategoryMgr, ITfCategoryMgr_Vtbl, 0xc3acefb5_f69d_4905_938f_fcadcf4be830);
windows_core::imp::interface_hierarchy!(ITfCategoryMgr, windows_core::IUnknown);
impl ITfCategoryMgr {
    pub unsafe fn RegisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterCategory)(windows_core::Interface::as_raw(self), rclsid, rcatid, rguid) }
    }
    pub unsafe fn UnregisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterCategory)(windows_core::Interface::as_raw(self), rclsid, rcatid, rguid) }
    }
    #[cfg(feature = "comcat")]
    pub unsafe fn EnumCategoriesInItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<super::comcat::IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCategoriesInItem)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "comcat")]
    pub unsafe fn EnumItemsInCategory(&self, rcatid: *const windows_core::GUID) -> windows_core::Result<super::comcat::IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumItemsInCategory)(windows_core::Interface::as_raw(self), rcatid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindClosestCategory(&self, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindClosestCategory)(windows_core::Interface::as_raw(self), rguid, pcatid as _, ppcatidlist, ulcount) }
    }
    pub unsafe fn RegisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: *const u16, cch: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterGUIDDescription)(windows_core::Interface::as_raw(self), rclsid, rguid, pchdesc, cch) }
    }
    pub unsafe fn UnregisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterGUIDDescription)(windows_core::Interface::as_raw(self), rclsid, rguid) }
    }
    pub unsafe fn GetGUIDDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUIDDescription)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RegisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterGUIDDWORD)(windows_core::Interface::as_raw(self), rclsid, rguid, dw) }
    }
    pub unsafe fn UnregisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterGUIDDWORD)(windows_core::Interface::as_raw(self), rclsid, rguid) }
    }
    pub unsafe fn GetGUIDDWORD(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUIDDWORD)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<TfGuidAtom> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterGUID)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGUID(&self, guidatom: TfGuidAtom) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), guidatom, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqualTfGuidAtom(&self, guidatom: TfGuidAtom, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqualTfGuidAtom)(windows_core::Interface::as_raw(self), guidatom, rguid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCategoryMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "comcat")]
    pub EnumCategoriesInItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "comcat"))]
    EnumCategoriesInItem: usize,
    #[cfg(feature = "comcat")]
    pub EnumItemsInCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "comcat"))]
    EnumItemsInCategory: usize,
    pub FindClosestCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::GUID, *const *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RegisterGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const u16, u32) -> windows_core::HRESULT,
    pub UnregisterGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGUIDDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub UnregisterGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGUIDDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub RegisterGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut TfGuidAtom) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, TfGuidAtom, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsEqualTfGuidAtom: unsafe extern "system" fn(*mut core::ffi::c_void, TfGuidAtom, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "comcat")]
pub trait ITfCategoryMgr_Impl: windows_core::IUnknownImpl {
    fn RegisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterCategory(&self, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCategoriesInItem(&self, rguid: *const windows_core::GUID) -> windows_core::Result<super::comcat::IEnumGUID>;
    fn EnumItemsInCategory(&self, rcatid: *const windows_core::GUID) -> windows_core::Result<super::comcat::IEnumGUID>;
    fn FindClosestCategory(&self, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::Result<()>;
    fn RegisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: *const u16, cch: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDescription(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::Result<()>;
    fn UnregisterGUIDDWORD(&self, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGUIDDWORD(&self, rguid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RegisterGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<TfGuidAtom>;
    fn GetGUID(&self, guidatom: TfGuidAtom) -> windows_core::Result<windows_core::GUID>;
    fn IsEqualTfGuidAtom(&self, guidatom: TfGuidAtom, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "comcat")]
impl ITfCategoryMgr_Vtbl {
    pub const fn new<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::RegisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn UnregisterCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rcatid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::UnregisterCategory(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rcatid), core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn EnumCategoriesInItem<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::EnumCategoriesInItem(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumItemsInCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcatid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::EnumItemsInCategory(this, core::mem::transmute_copy(&rcatid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindClosestCategory<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pcatid: *mut windows_core::GUID, ppcatidlist: *const *const windows_core::GUID, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::FindClosestCategory(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pcatid), core::mem::transmute_copy(&ppcatidlist), core::mem::transmute_copy(&ulcount)).into()
            }
        }
        unsafe extern "system" fn RegisterGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, pchdesc: *const u16, cch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::RegisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pchdesc), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::UnregisterGUIDDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn GetGUIDDescription<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::GetGUIDDescription(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pbstrdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID, dw: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::RegisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&dw)).into()
            }
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCategoryMgr_Impl::UnregisterGUIDDWORD(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn GetGUIDDWORD<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pdw: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::GetGUIDDWORD(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pdw.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterGUID<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pguidatom: *mut TfGuidAtom) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::RegisterGUID(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pguidatom.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: TfGuidAtom, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::GetGUID(this, core::mem::transmute_copy(&guidatom)) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Identity: ITfCategoryMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidatom: TfGuidAtom, rguid: *const windows_core::GUID, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCategoryMgr_Impl::IsEqualTfGuidAtom(this, core::mem::transmute_copy(&guidatom), core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pfequal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCategory: RegisterCategory::<Identity, OFFSET>,
            UnregisterCategory: UnregisterCategory::<Identity, OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Identity, OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Identity, OFFSET>,
            FindClosestCategory: FindClosestCategory::<Identity, OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Identity, OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Identity, OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Identity, OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Identity, OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Identity, OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Identity, OFFSET>,
            RegisterGUID: RegisterGUID::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCategoryMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "comcat")]
impl windows_core::RuntimeName for ITfCategoryMgr {}
windows_core::imp::define_interface!(ITfCleanupContextDurationSink, ITfCleanupContextDurationSink_Vtbl, 0x45c35144_154e_4797_bed8_d33ae7bf8794);
windows_core::imp::interface_hierarchy!(ITfCleanupContextDurationSink, windows_core::IUnknown);
impl ITfCleanupContextDurationSink {
    pub unsafe fn OnStartCleanupContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStartCleanupContext)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnEndCleanupContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEndCleanupContext)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextDurationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCleanupContextDurationSink_Impl: windows_core::IUnknownImpl {
    fn OnStartCleanupContext(&self) -> windows_core::Result<()>;
    fn OnEndCleanupContext(&self) -> windows_core::Result<()>;
}
impl ITfCleanupContextDurationSink_Vtbl {
    pub const fn new<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartCleanupContext<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCleanupContextDurationSink_Impl::OnStartCleanupContext(this).into()
            }
        }
        unsafe extern "system" fn OnEndCleanupContext<Identity: ITfCleanupContextDurationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCleanupContextDurationSink_Impl::OnEndCleanupContext(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartCleanupContext: OnStartCleanupContext::<Identity, OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextDurationSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCleanupContextDurationSink {}
windows_core::imp::define_interface!(ITfCleanupContextSink, ITfCleanupContextSink_Vtbl, 0x01689689_7acb_4e9b_ab7c_7ea46b12b522);
windows_core::imp::interface_hierarchy!(ITfCleanupContextSink, windows_core::IUnknown);
impl ITfCleanupContextSink {
    pub unsafe fn OnCleanupContext<P1>(&self, ecwrite: TfEditCookie, pic: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCleanupContext)(windows_core::Interface::as_raw(self), ecwrite, pic.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCleanupContext: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCleanupContextSink_Impl: windows_core::IUnknownImpl {
    fn OnCleanupContext(&self, ecwrite: TfEditCookie, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
}
impl ITfCleanupContextSink_Vtbl {
    pub const fn new<Identity: ITfCleanupContextSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCleanupContext<Identity: ITfCleanupContextSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCleanupContextSink_Impl::OnCleanupContext(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pic)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCleanupContext: OnCleanupContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCleanupContextSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCleanupContextSink {}
windows_core::imp::define_interface!(ITfClientId, ITfClientId_Vtbl, 0xd60a7b49_1b9f_4be2_b702_47e9dc05dec3);
windows_core::imp::interface_hierarchy!(ITfClientId, windows_core::IUnknown);
impl ITfClientId {
    pub unsafe fn GetClientId(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<TfClientId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientId)(windows_core::Interface::as_raw(self), rclsid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfClientId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut TfClientId) -> windows_core::HRESULT,
}
pub trait ITfClientId_Impl: windows_core::IUnknownImpl {
    fn GetClientId(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<TfClientId>;
}
impl ITfClientId_Vtbl {
    pub const fn new<Identity: ITfClientId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientId<Identity: ITfClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, ptid: *mut TfClientId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfClientId_Impl::GetClientId(this, core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        ptid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClientId: GetClientId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfClientId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfClientId {}
windows_core::imp::define_interface!(ITfCompartment, ITfCompartment_Vtbl, 0xbb08f7a9_607a_4384_8623_056892b64371);
windows_core::imp::interface_hierarchy!(ITfCompartment, windows_core::IUnknown);
impl ITfCompartment {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, tid: TfClientId, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), tid, pvarvalue) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITfCompartment_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, tid: TfClientId, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetValue(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITfCompartment_Vtbl {
    pub const fn new<Identity: ITfCompartment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: ITfCompartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCompartment_Impl::SetValue(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: ITfCompartment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCompartment_Impl::GetValue(this) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetValue: SetValue::<Identity, OFFSET>, GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITfCompartment {}
windows_core::imp::define_interface!(ITfCompartmentEventSink, ITfCompartmentEventSink_Vtbl, 0x743abd5f_f26d_48df_8cc5_238492419b64);
windows_core::imp::interface_hierarchy!(ITfCompartmentEventSink, windows_core::IUnknown);
impl ITfCompartmentEventSink {
    pub unsafe fn OnChange(&self, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), rguid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfCompartmentEventSink_Impl: windows_core::IUnknownImpl {
    fn OnChange(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfCompartmentEventSink_Vtbl {
    pub const fn new<Identity: ITfCompartmentEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChange<Identity: ITfCompartmentEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCompartmentEventSink_Impl::OnChange(this, core::mem::transmute_copy(&rguid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompartmentEventSink {}
windows_core::imp::define_interface!(ITfCompartmentMgr, ITfCompartmentMgr_Vtbl, 0x7dcf57ac_18ad_438b_824d_979bffb74b7c);
windows_core::imp::interface_hierarchy!(ITfCompartmentMgr, windows_core::IUnknown);
impl ITfCompartmentMgr {
    pub unsafe fn GetCompartment(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfCompartment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompartment)(windows_core::Interface::as_raw(self), rguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ClearCompartment(&self, tid: TfClientId, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearCompartment)(windows_core::Interface::as_raw(self), tid, rguid) }
    }
    #[cfg(feature = "comcat")]
    pub unsafe fn EnumCompartments(&self) -> windows_core::Result<super::comcat::IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCompartments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "comcat")]
    pub EnumCompartments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "comcat"))]
    EnumCompartments: usize,
}
#[cfg(feature = "comcat")]
pub trait ITfCompartmentMgr_Impl: windows_core::IUnknownImpl {
    fn GetCompartment(&self, rguid: *const windows_core::GUID) -> windows_core::Result<ITfCompartment>;
    fn ClearCompartment(&self, tid: TfClientId, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumCompartments(&self) -> windows_core::Result<super::comcat::IEnumGUID>;
}
#[cfg(feature = "comcat")]
impl ITfCompartmentMgr_Vtbl {
    pub const fn new<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCompartment<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, ppcomp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCompartmentMgr_Impl::GetCompartment(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        ppcomp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearCompartment<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCompartmentMgr_Impl::ClearCompartment(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid)).into()
            }
        }
        unsafe extern "system" fn EnumCompartments<Identity: ITfCompartmentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCompartmentMgr_Impl::EnumCompartments(this) {
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
            GetCompartment: GetCompartment::<Identity, OFFSET>,
            ClearCompartment: ClearCompartment::<Identity, OFFSET>,
            EnumCompartments: EnumCompartments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompartmentMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "comcat")]
impl windows_core::RuntimeName for ITfCompartmentMgr {}
windows_core::imp::define_interface!(ITfComposition, ITfComposition_Vtbl, 0x20168d64_5a8f_4a5a_b7bd_cfa29f4d0fd9);
windows_core::imp::interface_hierarchy!(ITfComposition, windows_core::IUnknown);
impl ITfComposition {
    pub unsafe fn GetRange(&self) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ShiftStart<P1>(&self, ecwrite: TfEditCookie, pnewstart: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShiftStart)(windows_core::Interface::as_raw(self), ecwrite, pnewstart.param().abi()) }
    }
    pub unsafe fn ShiftEnd<P1>(&self, ecwrite: TfEditCookie, pnewend: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShiftEnd)(windows_core::Interface::as_raw(self), ecwrite, pnewend.param().abi()) }
    }
    pub unsafe fn EndComposition(&self, ecwrite: TfEditCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndComposition)(windows_core::Interface::as_raw(self), ecwrite) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfComposition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftStart: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndComposition: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie) -> windows_core::HRESULT,
}
pub trait ITfComposition_Impl: windows_core::IUnknownImpl {
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
    fn ShiftStart(&self, ecwrite: TfEditCookie, pnewstart: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ecwrite: TfEditCookie, pnewend: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
    fn EndComposition(&self, ecwrite: TfEditCookie) -> windows_core::Result<()>;
}
impl ITfComposition_Vtbl {
    pub const fn new<Identity: ITfComposition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRange<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfComposition_Impl::GetRange(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShiftStart<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pnewstart: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfComposition_Impl::ShiftStart(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pnewstart)).into()
            }
        }
        unsafe extern "system" fn ShiftEnd<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pnewend: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfComposition_Impl::ShiftEnd(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pnewend)).into()
            }
        }
        unsafe extern "system" fn EndComposition<Identity: ITfComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfComposition_Impl::EndComposition(this, core::mem::transmute_copy(&ecwrite)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRange: GetRange::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            EndComposition: EndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfComposition {}
windows_core::imp::define_interface!(ITfCompositionSink, ITfCompositionSink_Vtbl, 0xa781718c_579a_4b15_a280_32b8577acc5e);
windows_core::imp::interface_hierarchy!(ITfCompositionSink, windows_core::IUnknown);
impl ITfCompositionSink {
    pub unsafe fn OnCompositionTerminated<P1>(&self, ecwrite: TfEditCookie, pcomposition: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfComposition>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCompositionTerminated)(windows_core::Interface::as_raw(self), ecwrite, pcomposition.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCompositionTerminated: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCompositionSink_Impl: windows_core::IUnknownImpl {
    fn OnCompositionTerminated(&self, ecwrite: TfEditCookie, pcomposition: windows_core::Ref<ITfComposition>) -> windows_core::Result<()>;
}
impl ITfCompositionSink_Vtbl {
    pub const fn new<Identity: ITfCompositionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCompositionTerminated<Identity: ITfCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfCompositionSink_Impl::OnCompositionTerminated(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pcomposition)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCompositionTerminated: OnCompositionTerminated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompositionSink {}
windows_core::imp::define_interface!(ITfCompositionView, ITfCompositionView_Vtbl, 0xd7540241_f9a1_4364_befc_dbcd2c4395b7);
windows_core::imp::interface_hierarchy!(ITfCompositionView, windows_core::IUnknown);
impl ITfCompositionView {
    pub unsafe fn GetOwnerClsid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwnerClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRange(&self) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwnerClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfCompositionView_Impl: windows_core::IUnknownImpl {
    fn GetOwnerClsid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetRange(&self) -> windows_core::Result<ITfRange>;
}
impl ITfCompositionView_Vtbl {
    pub const fn new<Identity: ITfCompositionView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOwnerClsid<Identity: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCompositionView_Impl::GetOwnerClsid(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITfCompositionView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCompositionView_Impl::GetRange(this) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerClsid: GetOwnerClsid::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCompositionView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfCompositionView {}
windows_core::imp::define_interface!(ITfConfigureSystemKeystrokeFeed, ITfConfigureSystemKeystrokeFeed_Vtbl, 0x0d2c969a_bc9c_437c_84ee_951c49b1a764);
windows_core::imp::interface_hierarchy!(ITfConfigureSystemKeystrokeFeed, windows_core::IUnknown);
impl ITfConfigureSystemKeystrokeFeed {
    pub unsafe fn DisableSystemKeystrokeFeed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableSystemKeystrokeFeed)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnableSystemKeystrokeFeed(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableSystemKeystrokeFeed)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DisableSystemKeystrokeFeed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableSystemKeystrokeFeed: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfConfigureSystemKeystrokeFeed_Impl: windows_core::IUnknownImpl {
    fn DisableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
    fn EnableSystemKeystrokeFeed(&self) -> windows_core::Result<()>;
}
impl ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub const fn new<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfConfigureSystemKeystrokeFeed_Impl::DisableSystemKeystrokeFeed(this).into()
            }
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Identity: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfConfigureSystemKeystrokeFeed_Impl::EnableSystemKeystrokeFeed(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Identity, OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfConfigureSystemKeystrokeFeed as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfConfigureSystemKeystrokeFeed {}
windows_core::imp::define_interface!(ITfContext, ITfContext_Vtbl, 0xaa80e7fd_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfContext, windows_core::IUnknown);
impl ITfContext {
    pub unsafe fn RequestEditSession<P1>(&self, tid: TfClientId, pes: P1, dwflags: u32) -> windows_core::Result<windows_core::HRESULT>
    where
        P1: windows_core::Param<ITfEditSession>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestEditSession)(windows_core::Interface::as_raw(self), tid, pes.param().abi(), dwflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InWriteSession(&self, tid: TfClientId) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InWriteSession)(windows_core::Interface::as_raw(self), tid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSelection(&self, ec: TfEditCookie, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), ec, ulindex, ulcount, pselection, pcfetched as _) }
    }
    pub unsafe fn SetSelection(&self, ec: TfEditCookie, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), ec, ulcount, pselection) }
    }
    pub unsafe fn GetStart(&self, ec: TfEditCookie) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEnd(&self, ec: TfEditCookie) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveView(&self) -> windows_core::Result<ITfContextView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumViews(&self) -> windows_core::Result<IEnumTfContextViews> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumViews)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "textstor")]
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TF_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), guidprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAppProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfReadOnlyProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAppProperty)(windows_core::Interface::as_raw(self), guidprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TrackProperties(&self, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32) -> windows_core::Result<ITfReadOnlyProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackProperties)(windows_core::Interface::as_raw(self), prgprop, cprop, prgappprop, cappprop, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumProperties(&self) -> windows_core::Result<IEnumTfProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRangeBackup<P1>(&self, ec: TfEditCookie, prange: P1) -> windows_core::Result<ITfRangeBackup>
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRangeBackup)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub InWriteSession: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, u32, *mut TF_SELECTION, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *const TF_SELECTION) -> windows_core::HRESULT,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "textstor")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "textstor"))]
    GetStatus: usize,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrackProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const windows_core::GUID, u32, *const *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRangeBackup: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "textstor")]
pub trait ITfContext_Impl: windows_core::IUnknownImpl {
    fn RequestEditSession(&self, tid: TfClientId, pes: windows_core::Ref<ITfEditSession>, dwflags: u32) -> windows_core::Result<windows_core::HRESULT>;
    fn InWriteSession(&self, tid: TfClientId) -> windows_core::Result<windows_core::BOOL>;
    fn GetSelection(&self, ec: TfEditCookie, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn SetSelection(&self, ec: TfEditCookie, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::Result<()>;
    fn GetStart(&self, ec: TfEditCookie) -> windows_core::Result<ITfRange>;
    fn GetEnd(&self, ec: TfEditCookie) -> windows_core::Result<ITfRange>;
    fn GetActiveView(&self) -> windows_core::Result<ITfContextView>;
    fn EnumViews(&self) -> windows_core::Result<IEnumTfContextViews>;
    fn GetStatus(&self) -> windows_core::Result<TF_STATUS>;
    fn GetProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfProperty>;
    fn GetAppProperty(&self, guidprop: *const windows_core::GUID) -> windows_core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(&self, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32) -> windows_core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(&self) -> windows_core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>) -> windows_core::Result<ITfRangeBackup>;
}
#[cfg(feature = "textstor")]
impl ITfContext_Vtbl {
    pub const fn new<Identity: ITfContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestEditSession<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, pes: *mut core::ffi::c_void, dwflags: u32, phrsession: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::RequestEditSession(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&pes), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        phrsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InWriteSession<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, pfwritesession: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::InWriteSession(this, core::mem::transmute_copy(&tid)) {
                    Ok(ok__) => {
                        pfwritesession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContext_Impl::GetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ulcount: u32, pselection: *const TF_SELECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContext_Impl::SetSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pselection)).into()
            }
        }
        unsafe extern "system" fn GetStart<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ppstart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetStart(this, core::mem::transmute_copy(&ec)) {
                    Ok(ok__) => {
                        ppstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnd<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ppend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetEnd(this, core::mem::transmute_copy(&ec)) {
                    Ok(ok__) => {
                        ppend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveView<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetActiveView(this) {
                    Ok(ok__) => {
                        ppview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumViews<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::EnumViews(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TF_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdcs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetProperty(this, core::mem::transmute_copy(&guidprop)) {
                    Ok(ok__) => {
                        ppprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAppProperty<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetAppProperty(this, core::mem::transmute_copy(&guidprop)) {
                    Ok(ok__) => {
                        ppprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrackProperties<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prgprop: *const *const windows_core::GUID, cprop: u32, prgappprop: *const *const windows_core::GUID, cappprop: u32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::TrackProperties(this, core::mem::transmute_copy(&prgprop), core::mem::transmute_copy(&cprop), core::mem::transmute_copy(&prgappprop), core::mem::transmute_copy(&cappprop)) {
                    Ok(ok__) => {
                        ppproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumProperties<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::EnumProperties(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::GetDocumentMgr(this) {
                    Ok(ok__) => {
                        ppdm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRangeBackup<Identity: ITfContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, ppbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContext_Impl::CreateRangeBackup(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        ppbackup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestEditSession: RequestEditSession::<Identity, OFFSET>,
            InWriteSession: InWriteSession::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            GetActiveView: GetActiveView::<Identity, OFFSET>,
            EnumViews: EnumViews::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetAppProperty: GetAppProperty::<Identity, OFFSET>,
            TrackProperties: TrackProperties::<Identity, OFFSET>,
            EnumProperties: EnumProperties::<Identity, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "textstor")]
impl windows_core::RuntimeName for ITfContext {}
windows_core::imp::define_interface!(ITfContextComposition, ITfContextComposition_Vtbl, 0xd40c8aae_ac92_4fc7_9a11_0ee0e23aa39b);
windows_core::imp::interface_hierarchy!(ITfContextComposition, windows_core::IUnknown);
impl ITfContextComposition {
    pub unsafe fn StartComposition<P1, P2>(&self, ecwrite: TfEditCookie, pcompositionrange: P1, psink: P2) -> windows_core::Result<ITfComposition>
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfCompositionSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartComposition)(windows_core::Interface::as_raw(self), ecwrite, pcompositionrange.param().abi(), psink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumCompositions(&self) -> windows_core::Result<IEnumITfCompositionView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCompositions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindComposition<P1>(&self, ecread: TfEditCookie, ptestrange: P1) -> windows_core::Result<IEnumITfCompositionView>
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindComposition)(windows_core::Interface::as_raw(self), ecread, ptestrange.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TakeOwnership<P1, P2>(&self, ecwrite: TfEditCookie, pcomposition: P1, psink: P2) -> windows_core::Result<ITfComposition>
    where
        P1: windows_core::Param<ITfCompositionView>,
        P2: windows_core::Param<ITfCompositionSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self), ecwrite, pcomposition.param().abi(), psink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextComposition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartComposition: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCompositions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindComposition: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextComposition_Impl: windows_core::IUnknownImpl {
    fn StartComposition(&self, ecwrite: TfEditCookie, pcompositionrange: windows_core::Ref<ITfRange>, psink: windows_core::Ref<ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
    fn EnumCompositions(&self) -> windows_core::Result<IEnumITfCompositionView>;
    fn FindComposition(&self, ecread: TfEditCookie, ptestrange: windows_core::Ref<ITfRange>) -> windows_core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(&self, ecwrite: TfEditCookie, pcomposition: windows_core::Ref<ITfCompositionView>, psink: windows_core::Ref<ITfCompositionSink>) -> windows_core::Result<ITfComposition>;
}
impl ITfContextComposition_Vtbl {
    pub const fn new<Identity: ITfContextComposition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartComposition<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pcompositionrange: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextComposition_Impl::StartComposition(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pcompositionrange), core::mem::transmute_copy(&psink)) {
                    Ok(ok__) => {
                        ppcomposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCompositions<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextComposition_Impl::EnumCompositions(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindComposition<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecread: TfEditCookie, ptestrange: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextComposition_Impl::FindComposition(this, core::mem::transmute_copy(&ecread), core::mem::transmute_copy(&ptestrange)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: ITfContextComposition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ecwrite: TfEditCookie, pcomposition: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, ppcomposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextComposition_Impl::TakeOwnership(this, core::mem::transmute_copy(&ecwrite), core::mem::transmute_copy(&pcomposition), core::mem::transmute_copy(&psink)) {
                    Ok(ok__) => {
                        ppcomposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartComposition: StartComposition::<Identity, OFFSET>,
            EnumCompositions: EnumCompositions::<Identity, OFFSET>,
            FindComposition: FindComposition::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextComposition {}
windows_core::imp::define_interface!(ITfContextKeyEventSink, ITfContextKeyEventSink_Vtbl, 0x0552ba5d_c835_4934_bf50_846aaa67432f);
windows_core::imp::interface_hierarchy!(ITfContextKeyEventSink, windows_core::IUnknown);
impl ITfContextKeyEventSink {
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnTestKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTestKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnTestKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTestKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextKeyEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub OnKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyUp: usize,
    #[cfg(feature = "minwindef")]
    pub OnTestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub OnTestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnTestKeyUp: usize,
}
#[cfg(feature = "minwindef")]
pub trait ITfContextKeyEventSink_Impl: windows_core::IUnknownImpl {
    fn OnKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnTestKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnTestKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "minwindef")]
impl ITfContextKeyEventSink_Vtbl {
    pub const fn new<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnKeyDown<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextKeyEventSink_Impl::OnKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextKeyEventSink_Impl::OnKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextKeyEventSink_Impl::OnTestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ITfContextKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextKeyEventSink_Impl::OnTestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextKeyEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for ITfContextKeyEventSink {}
windows_core::imp::define_interface!(ITfContextOwner, ITfContextOwner_Vtbl, 0xaa80e80c_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfContextOwner, windows_core::IUnknown);
impl ITfContextOwner {
    #[cfg(feature = "windef")]
    pub unsafe fn GetACPFromPoint(&self, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetACPFromPoint)(windows_core::Interface::as_raw(self), ptscreen, dwflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), acpstart, acpend, prc as _, pfclipped as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetScreenExt(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "textstor")]
    pub unsafe fn GetStatus(&self) -> windows_core::Result<TF_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetWnd(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), rguidattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetACPFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::windef::POINT, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "windef")]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut super::windef::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetTextExt: usize,
    #[cfg(feature = "windef")]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetScreenExt: usize,
    #[cfg(feature = "textstor")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "textstor"))]
    GetStatus: usize,
    #[cfg(feature = "windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWnd: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetAttribute: usize,
}
#[cfg(all(feature = "oaidl", feature = "textstor", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITfContextOwner_Impl: windows_core::IUnknownImpl {
    fn GetACPFromPoint(&self, ptscreen: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<i32>;
    fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetStatus(&self) -> windows_core::Result<TF_STATUS>;
    fn GetWnd(&self) -> windows_core::Result<super::windef::HWND>;
    fn GetAttribute(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "textstor", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl ITfContextOwner_Vtbl {
    pub const fn new<Identity: ITfContextOwner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetACPFromPoint<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptscreen: *const super::windef::POINT, dwflags: u32, pacp: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwner_Impl::GetACPFromPoint(this, core::mem::transmute_copy(&ptscreen), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pacp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwner_Impl::GetTextExt(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwner_Impl::GetScreenExt(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcs: *mut TF_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwner_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdcs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwner_Impl::GetWnd(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttribute<Identity: ITfContextOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwner_Impl::GetAttribute(this, core::mem::transmute_copy(&rguidattribute)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACPFromPoint: GetACPFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwner as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "textstor", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITfContextOwner {}
windows_core::imp::define_interface!(ITfContextOwnerCompositionServices, ITfContextOwnerCompositionServices_Vtbl, 0x86462810_593b_4916_9764_19c08e9ce110);
impl core::ops::Deref for ITfContextOwnerCompositionServices {
    type Target = ITfContextComposition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfContextOwnerCompositionServices, windows_core::IUnknown, ITfContextComposition);
impl ITfContextOwnerCompositionServices {
    pub unsafe fn TerminateComposition<P0>(&self, pcomposition: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        unsafe { (windows_core::Interface::vtable(self).TerminateComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionServices_Vtbl {
    pub base__: ITfContextComposition_Vtbl,
    pub TerminateComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextOwnerCompositionServices_Impl: ITfContextComposition_Impl {
    fn TerminateComposition(&self, pcomposition: windows_core::Ref<ITfCompositionView>) -> windows_core::Result<()>;
}
impl ITfContextOwnerCompositionServices_Vtbl {
    pub const fn new<Identity: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TerminateComposition<Identity: ITfContextOwnerCompositionServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerCompositionServices_Impl::TerminateComposition(this, core::mem::transmute_copy(&pcomposition)).into()
            }
        }
        Self { base__: ITfContextComposition_Vtbl::new::<Identity, OFFSET>(), TerminateComposition: TerminateComposition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionServices as windows_core::Interface>::IID || iid == &<ITfContextComposition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionServices {}
windows_core::imp::define_interface!(ITfContextOwnerCompositionSink, ITfContextOwnerCompositionSink_Vtbl, 0x5f20aa40_b57a_4f34_96ab_3576f377cc79);
windows_core::imp::interface_hierarchy!(ITfContextOwnerCompositionSink, windows_core::IUnknown);
impl ITfContextOwnerCompositionSink {
    pub unsafe fn OnStartComposition<P0>(&self, pcomposition: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnStartComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnUpdateComposition<P0, P1>(&self, pcomposition: P0, prangenew: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfCompositionView>,
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUpdateComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi(), prangenew.param().abi()) }
    }
    pub unsafe fn OnEndComposition<P0>(&self, pcomposition: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfCompositionView>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEndComposition)(windows_core::Interface::as_raw(self), pcomposition.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub OnUpdateComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfContextOwnerCompositionSink_Impl: windows_core::IUnknownImpl {
    fn OnStartComposition(&self, pcomposition: windows_core::Ref<ITfCompositionView>) -> windows_core::Result<windows_core::BOOL>;
    fn OnUpdateComposition(&self, pcomposition: windows_core::Ref<ITfCompositionView>, prangenew: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
    fn OnEndComposition(&self, pcomposition: windows_core::Ref<ITfCompositionView>) -> windows_core::Result<()>;
}
impl ITfContextOwnerCompositionSink_Vtbl {
    pub const fn new<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, pfok: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwnerCompositionSink_Impl::OnStartComposition(this, core::mem::transmute_copy(&pcomposition)) {
                    Ok(ok__) => {
                        pfok.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnUpdateComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerCompositionSink_Impl::OnUpdateComposition(this, core::mem::transmute_copy(&pcomposition), core::mem::transmute_copy(&prangenew)).into()
            }
        }
        unsafe extern "system" fn OnEndComposition<Identity: ITfContextOwnerCompositionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomposition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerCompositionSink_Impl::OnEndComposition(this, core::mem::transmute_copy(&pcomposition)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartComposition: OnStartComposition::<Identity, OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Identity, OFFSET>,
            OnEndComposition: OnEndComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerCompositionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfContextOwnerCompositionSink {}
windows_core::imp::define_interface!(ITfContextOwnerServices, ITfContextOwnerServices_Vtbl, 0xb23eb630_3e1c_11d3_a745_0050040ab407);
windows_core::imp::interface_hierarchy!(ITfContextOwnerServices, windows_core::IUnknown);
impl ITfContextOwnerServices {
    pub unsafe fn OnLayoutChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn OnAttributeChange(&self, rguidattribute: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnAttributeChange)(windows_core::Interface::as_raw(self), rguidattribute) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Serialize<P0, P1, P3>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), prange.param().abi(), phdr as _, pstream.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Unserialize<P0, P2, P3>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2, ploader: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
        P2: windows_core::Param<super::objidlbase::IStream>,
        P3: windows_core::Param<ITfPersistentPropertyLoaderACP>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unserialize)(windows_core::Interface::as_raw(self), pprop.param().abi(), phdr, pstream.param().abi(), ploader.param().abi()) }
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfProperty>,
    {
        unsafe { (windows_core::Interface::vtable(self).ForceLoadProperty)(windows_core::Interface::as_raw(self), pprop.param().abi()) }
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRange)(windows_core::Interface::as_raw(self), acpstart, acpend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnAttributeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Serialize: usize,
    #[cfg(feature = "objidlbase")]
    pub Unserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait ITfContextOwnerServices_Impl: windows_core::IUnknownImpl {
    fn OnLayoutChange(&self) -> windows_core::Result<()>;
    fn OnStatusChange(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OnAttributeChange(&self, rguidattribute: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Serialize(&self, pprop: windows_core::Ref<ITfProperty>, prange: windows_core::Ref<ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Unserialize(&self, pprop: windows_core::Ref<ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: windows_core::Ref<super::objidlbase::IStream>, ploader: windows_core::Ref<ITfPersistentPropertyLoaderACP>) -> windows_core::Result<()>;
    fn ForceLoadProperty(&self, pprop: windows_core::Ref<ITfProperty>) -> windows_core::Result<()>;
    fn CreateRange(&self, acpstart: i32, acpend: i32) -> windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "objidlbase")]
impl ITfContextOwnerServices_Vtbl {
    pub const fn new<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLayoutChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::OnLayoutChange(this).into()
            }
        }
        unsafe extern "system" fn OnStatusChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::OnStatusChange(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OnAttributeChange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidattribute: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::OnAttributeChange(this, core::mem::transmute_copy(&rguidattribute)).into()
            }
        }
        unsafe extern "system" fn Serialize<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, prange: *mut core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::Serialize(this, core::mem::transmute_copy(&pprop), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&phdr), core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn Unserialize<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::Unserialize(this, core::mem::transmute_copy(&pprop), core::mem::transmute_copy(&phdr), core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&ploader)).into()
            }
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprop: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextOwnerServices_Impl::ForceLoadProperty(this, core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn CreateRange<Identity: ITfContextOwnerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextOwnerServices_Impl::CreateRange(this, core::mem::transmute_copy(&acpstart), core::mem::transmute_copy(&acpend)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLayoutChange: OnLayoutChange::<Identity, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, OFFSET>,
            OnAttributeChange: OnAttributeChange::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            Unserialize: Unserialize::<Identity, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, OFFSET>,
            CreateRange: CreateRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextOwnerServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ITfContextOwnerServices {}
windows_core::imp::define_interface!(ITfContextView, ITfContextView_Vtbl, 0x2433bf8e_0f9b_435c_ba2c_180611978c30);
windows_core::imp::interface_hierarchy!(ITfContextView, windows_core::IUnknown);
impl ITfContextView {
    #[cfg(feature = "windef")]
    pub unsafe fn GetRangeFromPoint(&self, ec: TfEditCookie, ppt: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeFromPoint)(windows_core::Interface::as_raw(self), ec, ppt, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetTextExt<P1>(&self, ec: TfEditCookie, prange: P1, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTextExt)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), prc as _, pfclipped as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetScreenExt(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScreenExt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetWnd(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetRangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *const super::windef::POINT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetRangeFromPoint: usize,
    #[cfg(feature = "windef")]
    pub GetTextExt: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut super::windef::RECT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetTextExt: usize,
    #[cfg(feature = "windef")]
    pub GetScreenExt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetScreenExt: usize,
    #[cfg(feature = "windef")]
    pub GetWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWnd: usize,
}
#[cfg(feature = "windef")]
pub trait ITfContextView_Impl: windows_core::IUnknownImpl {
    fn GetRangeFromPoint(&self, ec: TfEditCookie, ppt: *const super::windef::POINT, dwflags: u32) -> windows_core::Result<ITfRange>;
    fn GetTextExt(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetScreenExt(&self) -> windows_core::Result<super::windef::RECT>;
    fn GetWnd(&self) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(feature = "windef")]
impl ITfContextView_Vtbl {
    pub const fn new<Identity: ITfContextView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRangeFromPoint<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ppt: *const super::windef::POINT, dwflags: u32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextView_Impl::GetRangeFromPoint(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextExt<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, prc: *mut super::windef::RECT, pfclipped: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfContextView_Impl::GetTextExt(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&prc), core::mem::transmute_copy(&pfclipped)).into()
            }
        }
        unsafe extern "system" fn GetScreenExt<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextView_Impl::GetScreenExt(this) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWnd<Identity: ITfContextView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfContextView_Impl::GetWnd(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRangeFromPoint: GetRangeFromPoint::<Identity, OFFSET>,
            GetTextExt: GetTextExt::<Identity, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, OFFSET>,
            GetWnd: GetWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfContextView as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ITfContextView {}
windows_core::imp::define_interface!(ITfCreatePropertyStore, ITfCreatePropertyStore_Vtbl, 0x2463fbf0_b0af_11d2_afc5_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfCreatePropertyStore, windows_core::IUnknown);
impl ITfCreatePropertyStore {
    pub unsafe fn IsStoreSerializable<P1, P2>(&self, guidprop: *const windows_core::GUID, prange: P1, ppropstore: P2) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfPropertyStore>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStoreSerializable)(windows_core::Interface::as_raw(self), guidprop, prange.param().abi(), ppropstore.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreatePropertyStore<P1, P3>(&self, guidprop: *const windows_core::GUID, prange: P1, cb: u32, pstream: P3) -> windows_core::Result<ITfPropertyStore>
    where
        P1: windows_core::Param<ITfRange>,
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyStore)(windows_core::Interface::as_raw(self), guidprop, prange.param().abi(), cb, pstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCreatePropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStoreSerializable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub CreatePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreatePropertyStore: usize,
}
#[cfg(feature = "objidlbase")]
pub trait ITfCreatePropertyStore_Impl: windows_core::IUnknownImpl {
    fn IsStoreSerializable(&self, guidprop: *const windows_core::GUID, prange: windows_core::Ref<ITfRange>, ppropstore: windows_core::Ref<ITfPropertyStore>) -> windows_core::Result<windows_core::BOOL>;
    fn CreatePropertyStore(&self, guidprop: *const windows_core::GUID, prange: windows_core::Ref<ITfRange>, cb: u32, pstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<ITfPropertyStore>;
}
#[cfg(feature = "objidlbase")]
impl ITfCreatePropertyStore_Vtbl {
    pub const fn new<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStoreSerializable<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void, pfserializable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCreatePropertyStore_Impl::IsStoreSerializable(this, core::mem::transmute_copy(&guidprop), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&ppropstore)) {
                    Ok(ok__) => {
                        pfserializable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyStore<Identity: ITfCreatePropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprop: *const windows_core::GUID, prange: *mut core::ffi::c_void, cb: u32, pstream: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfCreatePropertyStore_Impl::CreatePropertyStore(this, core::mem::transmute_copy(&guidprop), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pstream)) {
                    Ok(ok__) => {
                        ppstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStoreSerializable: IsStoreSerializable::<Identity, OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfCreatePropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ITfCreatePropertyStore {}
windows_core::imp::define_interface!(ITfDisplayAttributeInfo, ITfDisplayAttributeInfo_Vtbl, 0x70528852_2f26_4aea_8c96_215150578932);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeInfo, windows_core::IUnknown);
impl ITfDisplayAttributeInfo {
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeInfo)(windows_core::Interface::as_raw(self), pda as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttributeInfo)(windows_core::Interface::as_raw(self), pda) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetAttributeInfo: usize,
    #[cfg(feature = "windef")]
    pub SetAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetAttributeInfo: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ITfDisplayAttributeInfo_Impl: windows_core::IUnknownImpl {
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl ITfDisplayAttributeInfo_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGUID<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDisplayAttributeInfo_Impl::GetGUID(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDisplayAttributeInfo_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeInfo_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
            }
        }
        unsafe extern "system" fn SetAttributeInfo<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeInfo_Impl::SetAttributeInfo(this, core::mem::transmute_copy(&pda)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ITfDisplayAttributeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeInfo_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ITfDisplayAttributeInfo {}
windows_core::imp::define_interface!(ITfDisplayAttributeMgr, ITfDisplayAttributeMgr_Vtbl, 0x8ded7393_5db1_475c_9e71_a39111b0ff67);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeMgr, windows_core::IUnknown);
impl ITfDisplayAttributeMgr {
    pub unsafe fn OnUpdateInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUpdateInfo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDisplayAttributeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID, ppinfo: *mut Option<ITfDisplayAttributeInfo>, pclsidowner: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayAttributeInfo)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(ppinfo), pclsidowner as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeMgr_Impl: windows_core::IUnknownImpl {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID, ppinfo: windows_core::OutRef<ITfDisplayAttributeInfo>, pclsidowner: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfDisplayAttributeMgr_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdateInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeMgr_Impl::OnUpdateInfo(this).into()
            }
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDisplayAttributeMgr_Impl::EnumDisplayAttributeInfo(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ITfDisplayAttributeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void, pclsidowner: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeMgr_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&ppinfo), core::mem::transmute_copy(&pclsidowner)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeMgr {}
windows_core::imp::define_interface!(ITfDisplayAttributeNotifySink, ITfDisplayAttributeNotifySink_Vtbl, 0xad56f402_e162_4f25_908f_7d577cf9bda9);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeNotifySink, windows_core::IUnknown);
impl ITfDisplayAttributeNotifySink {
    pub unsafe fn OnUpdateInfo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUpdateInfo)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnUpdateInfo(&self) -> windows_core::Result<()>;
}
impl ITfDisplayAttributeNotifySink_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdateInfo<Identity: ITfDisplayAttributeNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDisplayAttributeNotifySink_Impl::OnUpdateInfo(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdateInfo: OnUpdateInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeNotifySink {}
windows_core::imp::define_interface!(ITfDisplayAttributeProvider, ITfDisplayAttributeProvider_Vtbl, 0xfee47777_163c_4769_996a_6e9c50ad8f54);
windows_core::imp::interface_hierarchy!(ITfDisplayAttributeProvider, windows_core::IUnknown);
impl ITfDisplayAttributeProvider {
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDisplayAttributeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITfDisplayAttributeInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayAttributeInfo)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDisplayAttributeProvider_Impl: windows_core::IUnknownImpl {
    fn EnumDisplayAttributeInfo(&self) -> windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITfDisplayAttributeInfo>;
}
impl ITfDisplayAttributeProvider_Vtbl {
    pub const fn new<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDisplayAttributeProvider_Impl::EnumDisplayAttributeInfo(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ITfDisplayAttributeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDisplayAttributeProvider_Impl::GetDisplayAttributeInfo(this, core::mem::transmute_copy(&guid)) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDisplayAttributeProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDisplayAttributeProvider {}
windows_core::imp::define_interface!(ITfDocumentMgr, ITfDocumentMgr_Vtbl, 0xaa80e7f4_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfDocumentMgr, windows_core::IUnknown);
impl ITfDocumentMgr {
    pub unsafe fn CreateContext<P2>(&self, tidowner: TfClientId, dwflags: u32, punk: P2, ppic: *mut Option<ITfContext>, pectextstore: *mut TfEditCookie) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), tidowner, dwflags, punk.param().abi(), core::mem::transmute(ppic), pectextstore as _) }
    }
    pub unsafe fn Push<P0>(&self, pic: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), pic.param().abi()) }
    }
    pub unsafe fn Pop(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pop)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn GetTop(&self) -> windows_core::Result<ITfContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTop)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBase(&self) -> windows_core::Result<ITfContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBase)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumContexts(&self) -> windows_core::Result<IEnumTfContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumContexts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDocumentMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateContext: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut TfEditCookie) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pop: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfDocumentMgr_Impl: windows_core::IUnknownImpl {
    fn CreateContext(&self, tidowner: TfClientId, dwflags: u32, punk: windows_core::Ref<windows_core::IUnknown>, ppic: windows_core::OutRef<ITfContext>, pectextstore: *mut TfEditCookie) -> windows_core::Result<()>;
    fn Push(&self, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
    fn Pop(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetTop(&self) -> windows_core::Result<ITfContext>;
    fn GetBase(&self) -> windows_core::Result<ITfContext>;
    fn EnumContexts(&self) -> windows_core::Result<IEnumTfContexts>;
}
impl ITfDocumentMgr_Vtbl {
    pub const fn new<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateContext<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tidowner: TfClientId, dwflags: u32, punk: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void, pectextstore: *mut TfEditCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDocumentMgr_Impl::CreateContext(this, core::mem::transmute_copy(&tidowner), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ppic), core::mem::transmute_copy(&pectextstore)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDocumentMgr_Impl::Push(this, core::mem::transmute_copy(&pic)).into()
            }
        }
        unsafe extern "system" fn Pop<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfDocumentMgr_Impl::Pop(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetTop<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDocumentMgr_Impl::GetTop(this) {
                    Ok(ok__) => {
                        ppic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBase<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDocumentMgr_Impl::GetBase(this) {
                    Ok(ok__) => {
                        ppic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumContexts<Identity: ITfDocumentMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfDocumentMgr_Impl::EnumContexts(this) {
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
            CreateContext: CreateContext::<Identity, OFFSET>,
            Push: Push::<Identity, OFFSET>,
            Pop: Pop::<Identity, OFFSET>,
            GetTop: GetTop::<Identity, OFFSET>,
            GetBase: GetBase::<Identity, OFFSET>,
            EnumContexts: EnumContexts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfDocumentMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfDocumentMgr {}
windows_core::imp::define_interface!(ITfEditRecord, ITfEditRecord_Vtbl, 0x42d4d099_7c1a_4a89_b836_6c6f22160df0);
windows_core::imp::interface_hierarchy!(ITfEditRecord, windows_core::IUnknown);
impl ITfEditRecord {
    pub unsafe fn GetSelectionStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectionStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTextAndPropertyUpdates(&self, dwflags: u32, prgproperties: *const *const windows_core::GUID, cproperties: u32) -> windows_core::Result<IEnumTfRanges> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextAndPropertyUpdates)(windows_core::Interface::as_raw(self), dwflags, prgproperties, cproperties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditRecord_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTextAndPropertyUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfEditRecord_Impl: windows_core::IUnknownImpl {
    fn GetSelectionStatus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetTextAndPropertyUpdates(&self, dwflags: u32, prgproperties: *const *const windows_core::GUID, cproperties: u32) -> windows_core::Result<IEnumTfRanges>;
}
impl ITfEditRecord_Vtbl {
    pub const fn new<Identity: ITfEditRecord_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelectionStatus<Identity: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfchanged: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfEditRecord_Impl::GetSelectionStatus(this) {
                    Ok(ok__) => {
                        pfchanged.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Identity: ITfEditRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, prgproperties: *const *const windows_core::GUID, cproperties: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfEditRecord_Impl::GetTextAndPropertyUpdates(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&prgproperties), core::mem::transmute_copy(&cproperties)) {
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
            GetSelectionStatus: GetSelectionStatus::<Identity, OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditRecord as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditRecord {}
windows_core::imp::define_interface!(ITfEditSession, ITfEditSession_Vtbl, 0xaa80e803_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfEditSession, windows_core::IUnknown);
impl ITfEditSession {
    pub unsafe fn DoEditSession(&self, ec: TfEditCookie) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoEditSession)(windows_core::Interface::as_raw(self), ec) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoEditSession: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie) -> windows_core::HRESULT,
}
pub trait ITfEditSession_Impl: windows_core::IUnknownImpl {
    fn DoEditSession(&self, ec: TfEditCookie) -> windows_core::Result<()>;
}
impl ITfEditSession_Vtbl {
    pub const fn new<Identity: ITfEditSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoEditSession<Identity: ITfEditSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfEditSession_Impl::DoEditSession(this, core::mem::transmute_copy(&ec)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoEditSession: DoEditSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditSession {}
windows_core::imp::define_interface!(ITfEditTransactionSink, ITfEditTransactionSink_Vtbl, 0x708fbf70_b520_416b_b06c_2c41ab44f8ba);
windows_core::imp::interface_hierarchy!(ITfEditTransactionSink, windows_core::IUnknown);
impl ITfEditTransactionSink {
    pub unsafe fn OnStartEditTransaction<P0>(&self, pic: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStartEditTransaction)(windows_core::Interface::as_raw(self), pic.param().abi()) }
    }
    pub unsafe fn OnEndEditTransaction<P0>(&self, pic: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEndEditTransaction)(windows_core::Interface::as_raw(self), pic.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditTransactionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfEditTransactionSink_Impl: windows_core::IUnknownImpl {
    fn OnStartEditTransaction(&self, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
    fn OnEndEditTransaction(&self, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
}
impl ITfEditTransactionSink_Vtbl {
    pub const fn new<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStartEditTransaction<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfEditTransactionSink_Impl::OnStartEditTransaction(this, core::mem::transmute_copy(&pic)).into()
            }
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ITfEditTransactionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfEditTransactionSink_Impl::OnEndEditTransaction(this, core::mem::transmute_copy(&pic)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStartEditTransaction: OnStartEditTransaction::<Identity, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfEditTransactionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfEditTransactionSink {}
windows_core::imp::define_interface!(ITfFunction, ITfFunction_Vtbl, 0xdb593490_098f_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfFunction, windows_core::IUnknown);
impl ITfFunction {
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFunction_Impl: windows_core::IUnknownImpl {
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfFunction_Vtbl {
    pub const fn new<Identity: ITfFunction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayName<Identity: ITfFunction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfFunction_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDisplayName: GetDisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFunction {}
windows_core::imp::define_interface!(ITfFunctionProvider, ITfFunctionProvider_Vtbl, 0x101d6610_0990_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfFunctionProvider, windows_core::IUnknown);
impl ITfFunctionProvider {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFunction<T>(&self, rguid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), rguid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunctionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfFunctionProvider_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFunction(&self, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ITfFunctionProvider_Vtbl {
    pub const fn new<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfFunctionProvider_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfFunctionProvider_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunction<Identity: ITfFunctionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfFunctionProvider_Impl::GetFunction(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfFunctionProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfFunctionProvider {}
windows_core::imp::define_interface!(ITfInputProcessorProfileActivationSink, ITfInputProcessorProfileActivationSink_Vtbl, 0x71c6e74e_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileActivationSink, windows_core::IUnknown);
impl ITfInputProcessorProfileActivationSink {
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn OnActivated(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnActivated)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, catid, guidprofile, hkl, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileActivationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub OnActivated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, super::minwindef::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    OnActivated: usize,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub trait ITfInputProcessorProfileActivationSink_Impl: windows_core::IUnknownImpl {
    fn OnActivated(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl ITfInputProcessorProfileActivationSink_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivated<Identity: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, catid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileActivationSink_Impl::OnActivated(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivated: OnActivated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileActivationSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfInputProcessorProfileActivationSink {}
windows_core::imp::define_interface!(ITfInputProcessorProfileMgr, ITfInputProcessorProfileMgr_Vtbl, 0x71c6e74c_0f28_11d8_a82a_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileMgr, windows_core::IUnknown);
impl ITfInputProcessorProfileMgr {
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn ActivateProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn DeactivateProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeactivateProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn GetProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl, pprofile as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumProfiles(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumTfInputProcessorProfiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumProfiles)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseInputProcessor(&self, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseInputProcessor)(windows_core::Interface::as_raw(self), rclsid, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn RegisterProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32, hklsubstitute: super::minwindef::HKL, dwpreferredlayout: u32, benabledbydefault: bool, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, pchdesc, cchdesc, pchiconfile, cchfile, uiconindex, hklsubstitute, dwpreferredlayout, benabledbydefault.into(), dwflags) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn UnregisterProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, dwflags) }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn GetActiveProfile(&self, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActiveProfile)(windows_core::Interface::as_raw(self), catid, pprofile as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub ActivateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *const windows_core::GUID, *const windows_core::GUID, super::minwindef::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    ActivateProfile: usize,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub DeactivateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *const windows_core::GUID, *const windows_core::GUID, super::minwindef::HKL, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    DeactivateProfile: usize,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::winnt::LANGID, *const windows_core::GUID, *const windows_core::GUID, super::minwindef::HKL, *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    GetProfile: usize,
    #[cfg(feature = "winnt")]
    pub EnumProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumProfiles: usize,
    pub ReleaseInputProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub RegisterProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *const u16, u32, *const u16, u32, u32, super::minwindef::HKL, u32, windows_core::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    RegisterProfile: usize,
    #[cfg(feature = "winnt")]
    pub UnregisterProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    UnregisterProfile: usize,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub GetActiveProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    GetActiveProfile: usize,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub trait ITfInputProcessorProfileMgr_Impl: windows_core::IUnknownImpl {
    fn ActivateProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn DeactivateProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::Result<()>;
    fn GetProfile(&self, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
    fn EnumProfiles(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(&self, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn RegisterProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32, hklsubstitute: super::minwindef::HKL, dwpreferredlayout: u32, benabledbydefault: windows_core::BOOL, dwflags: u32) -> windows_core::Result<()>;
    fn UnregisterProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveProfile(&self, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl ITfInputProcessorProfileMgr_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::ActivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DeactivateProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::DeactivateProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofiletype: u32, langid: super::winnt::LANGID, clsid: *const windows_core::GUID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::GetProfile(this, core::mem::transmute_copy(&dwprofiletype), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn EnumProfiles<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfileMgr_Impl::EnumProfiles(this, core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseInputProcessor<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::ReleaseInputProcessor(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RegisterProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32, hklsubstitute: super::minwindef::HKL, dwpreferredlayout: u32, benabledbydefault: windows_core::BOOL, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::RegisterProfile(
                    this,
                    core::mem::transmute_copy(&rclsid),
                    core::mem::transmute_copy(&langid),
                    core::mem::transmute_copy(&guidprofile),
                    core::mem::transmute_copy(&pchdesc),
                    core::mem::transmute_copy(&cchdesc),
                    core::mem::transmute_copy(&pchiconfile),
                    core::mem::transmute_copy(&cchfile),
                    core::mem::transmute_copy(&uiconindex),
                    core::mem::transmute_copy(&hklsubstitute),
                    core::mem::transmute_copy(&dwpreferredlayout),
                    core::mem::transmute_copy(&benabledbydefault),
                    core::mem::transmute_copy(&dwflags),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnregisterProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::UnregisterProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetActiveProfile<Identity: ITfInputProcessorProfileMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, catid: *const windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfileMgr_Impl::GetActiveProfile(this, core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pprofile)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateProfile: ActivateProfile::<Identity, OFFSET>,
            DeactivateProfile: DeactivateProfile::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
            EnumProfiles: EnumProfiles::<Identity, OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Identity, OFFSET>,
            RegisterProfile: RegisterProfile::<Identity, OFFSET>,
            UnregisterProfile: UnregisterProfile::<Identity, OFFSET>,
            GetActiveProfile: GetActiveProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileMgr as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfInputProcessorProfileMgr {}
windows_core::imp::define_interface!(ITfInputProcessorProfileSubstituteLayout, ITfInputProcessorProfileSubstituteLayout_Vtbl, 0x4fd67194_1002_4513_bff2_c0ddf6258552);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfileSubstituteLayout, windows_core::IUnknown);
impl ITfInputProcessorProfileSubstituteLayout {
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn GetSubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::minwindef::HKL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubstituteKeyboardLayout)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub GetSubstituteKeyboardLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *mut super::minwindef::HKL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    GetSubstituteKeyboardLayout: usize,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: windows_core::IUnknownImpl {
    fn GetSubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<super::minwindef::HKL>;
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Identity: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, phkl: *mut super::minwindef::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfileSubstituteLayout_Impl::GetSubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                    Ok(ok__) => {
                        phkl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfileSubstituteLayout as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfInputProcessorProfileSubstituteLayout {}
windows_core::imp::define_interface!(ITfInputProcessorProfiles, ITfInputProcessorProfiles_Vtbl, 0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfiles, windows_core::IUnknown);
impl ITfInputProcessorProfiles {
    pub unsafe fn Register(&self, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), rclsid) }
    }
    pub unsafe fn Unregister(&self, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), rclsid) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn AddLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, pchdesc, cchdesc, pchiconfile, cchfile, uiconindex) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile) }
    }
    #[cfg(feature = "comcat")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> windows_core::Result<super::comcat::IEnumGUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumInputProcessorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: super::winnt::LANGID, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultLanguageProfile)(windows_core::Interface::as_raw(self), langid, catid, pclsid as _, pguidprofile as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: super::winnt::LANGID, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultLanguageProfile)(windows_core::Interface::as_raw(self), langid, rclsid, guidprofiles) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofiles) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const windows_core::GUID, plangid: *mut super::winnt::LANGID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActiveLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, plangid as _, pguidprofile as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageProfileDescription)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetCurrentLanguage(&self) -> windows_core::Result<super::winnt::LANGID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn ChangeCurrentLanguage(&self, langid: super::winnt::LANGID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeCurrentLanguage)(windows_core::Interface::as_raw(self), langid) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut super::winnt::LANGID, pulcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLanguageList)(windows_core::Interface::as_raw(self), pplangid as _, pulcount as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnumLanguageProfiles(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumTfLanguageProfiles> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumLanguageProfiles)(windows_core::Interface::as_raw(self), langid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnableLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabledLanguageProfile)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn EnableLanguageProfileByDefault(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableLanguageProfileByDefault)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into()) }
    }
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub unsafe fn SubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SubstituteKeyboardLayout)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, hkl) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub AddLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *const u16, u32, *const u16, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    AddLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub RemoveLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RemoveLanguageProfile: usize,
    #[cfg(feature = "comcat")]
    pub EnumInputProcessorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "comcat"))]
    EnumInputProcessorInfo: usize,
    #[cfg(feature = "winnt")]
    pub GetDefaultLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *const windows_core::GUID, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetDefaultLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub SetDefaultLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetDefaultLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub ActivateLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    ActivateLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub GetActiveLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::winnt::LANGID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetActiveLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub GetLanguageProfileDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLanguageProfileDescription: usize,
    #[cfg(feature = "winnt")]
    pub GetCurrentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::LANGID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetCurrentLanguage: usize,
    #[cfg(feature = "winnt")]
    pub ChangeCurrentLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    ChangeCurrentLanguage: usize,
    #[cfg(feature = "winnt")]
    pub GetLanguageList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::winnt::LANGID, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetLanguageList: usize,
    #[cfg(feature = "winnt")]
    pub EnumLanguageProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnumLanguageProfiles: usize,
    #[cfg(feature = "winnt")]
    pub EnableLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnableLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub IsEnabledLanguageProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    IsEnabledLanguageProfile: usize,
    #[cfg(feature = "winnt")]
    pub EnableLanguageProfileByDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    EnableLanguageProfileByDefault: usize,
    #[cfg(all(feature = "minwindef", feature = "winnt"))]
    pub SubstituteKeyboardLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, super::minwindef::HKL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "winnt")))]
    SubstituteKeyboardLayout: usize,
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
pub trait ITfInputProcessorProfiles_Impl: windows_core::IUnknownImpl {
    fn Register(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Unregister(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn AddLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32) -> windows_core::Result<()>;
    fn RemoveLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumInputProcessorInfo(&self) -> windows_core::Result<super::comcat::IEnumGUID>;
    fn GetDefaultLanguageProfile(&self, langid: super::winnt::LANGID, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetDefaultLanguageProfile(&self, langid: super::winnt::LANGID, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ActivateLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofiles: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetActiveLanguageProfile(&self, rclsid: *const windows_core::GUID, plangid: *mut super::winnt::LANGID, pguidprofile: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetLanguageProfileDescription(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentLanguage(&self) -> windows_core::Result<super::winnt::LANGID>;
    fn ChangeCurrentLanguage(&self, langid: super::winnt::LANGID) -> windows_core::Result<()>;
    fn GetLanguageList(&self, pplangid: *mut *mut super::winnt::LANGID, pulcount: *mut u32) -> windows_core::Result<()>;
    fn EnumLanguageProfiles(&self, langid: super::winnt::LANGID) -> windows_core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsEnabledLanguageProfile(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
    fn EnableLanguageProfileByDefault(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SubstituteKeyboardLayout(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
impl ITfInputProcessorProfiles_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::Register(this, core::mem::transmute_copy(&rclsid)).into()
            }
        }
        unsafe extern "system" fn Unregister<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::Unregister(this, core::mem::transmute_copy(&rclsid)).into()
            }
        }
        unsafe extern "system" fn AddLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32, pchiconfile: *const u16, cchfile: u32, uiconindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::AddLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&pchdesc), core::mem::transmute_copy(&cchdesc), core::mem::transmute_copy(&pchiconfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uiconindex)).into()
            }
        }
        unsafe extern "system" fn RemoveLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::RemoveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)).into()
            }
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfiles_Impl::EnumInputProcessorInfo(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, catid: *const windows_core::GUID, pclsid: *mut windows_core::GUID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::GetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&catid), core::mem::transmute_copy(&pclsid), core::mem::transmute_copy(&pguidprofile)).into()
            }
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, rclsid: *const windows_core::GUID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::SetDefaultLanguageProfile(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&guidprofiles)).into()
            }
        }
        unsafe extern "system" fn ActivateLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofiles: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::ActivateLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofiles)).into()
            }
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, plangid: *mut super::winnt::LANGID, pguidprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::GetActiveLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&plangid), core::mem::transmute_copy(&pguidprofile)).into()
            }
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pbstrprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfiles_Impl::GetLanguageProfileDescription(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                    Ok(ok__) => {
                        pbstrprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentLanguage<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plangid: *mut super::winnt::LANGID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfiles_Impl::GetCurrentLanguage(this) {
                    Ok(ok__) => {
                        plangid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::ChangeCurrentLanguage(this, core::mem::transmute_copy(&langid)).into()
            }
        }
        unsafe extern "system" fn GetLanguageList<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplangid: *mut *mut super::winnt::LANGID, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::GetLanguageList(this, core::mem::transmute_copy(&pplangid), core::mem::transmute_copy(&pulcount)).into()
            }
        }
        unsafe extern "system" fn EnumLanguageProfiles<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfiles_Impl::EnumLanguageProfiles(this, core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::EnableLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pfenable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInputProcessorProfiles_Impl::IsEnabledLanguageProfile(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile)) {
                    Ok(ok__) => {
                        pfenable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::EnableLanguageProfileByDefault(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Identity: ITfInputProcessorProfiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, hkl: super::minwindef::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfiles_Impl::SubstituteKeyboardLayout(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&hkl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Identity, OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Identity, OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Identity, OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Identity, OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Identity, OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Identity, OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Identity, OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Identity, OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Identity, OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Identity, OFFSET>,
            GetLanguageList: GetLanguageList::<Identity, OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Identity, OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Identity, OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Identity, OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Identity, OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfInputProcessorProfiles {}
windows_core::imp::define_interface!(ITfInputProcessorProfilesEx, ITfInputProcessorProfilesEx_Vtbl, 0x892f230f_fe00_4a41_a98e_fcd6de0d35ef);
impl core::ops::Deref for ITfInputProcessorProfilesEx {
    type Target = ITfInputProcessorProfiles;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfInputProcessorProfilesEx, windows_core::IUnknown, ITfInputProcessorProfiles);
impl ITfInputProcessorProfilesEx {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetLanguageProfileDisplayName(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchfile: *const u16, cchfile: u32, uresid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLanguageProfileDisplayName)(windows_core::Interface::as_raw(self), rclsid, langid, guidprofile, pchfile, cchfile, uresid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfilesEx_Vtbl {
    pub base__: ITfInputProcessorProfiles_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetLanguageProfileDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::winnt::LANGID, *const windows_core::GUID, *const u16, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetLanguageProfileDisplayName: usize,
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
pub trait ITfInputProcessorProfilesEx_Impl: ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(&self, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchfile: *const u16, cchfile: u32, uresid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
impl ITfInputProcessorProfilesEx_Vtbl {
    pub const fn new<Identity: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Identity: ITfInputProcessorProfilesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, pchfile: *const u16, cchfile: u32, uresid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfInputProcessorProfilesEx_Impl::SetLanguageProfileDisplayName(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&pchfile), core::mem::transmute_copy(&cchfile), core::mem::transmute_copy(&uresid)).into()
            }
        }
        Self {
            base__: ITfInputProcessorProfiles_Vtbl::new::<Identity, OFFSET>(),
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInputProcessorProfilesEx as windows_core::Interface>::IID || iid == &<ITfInputProcessorProfiles as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "comcat", feature = "minwindef", feature = "winnt"))]
impl windows_core::RuntimeName for ITfInputProcessorProfilesEx {}
windows_core::imp::define_interface!(ITfInsertAtSelection, ITfInsertAtSelection_Vtbl, 0x55ce16ba_3014_41c1_9ceb_fade1446ac6c);
windows_core::imp::interface_hierarchy!(ITfInsertAtSelection, windows_core::IUnknown);
impl ITfInsertAtSelection {
    pub unsafe fn InsertTextAtSelection(&self, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32) -> windows_core::Result<ITfRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertTextAtSelection)(windows_core::Interface::as_raw(self), ec, dwflags, pchtext, cch, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn InsertEmbeddedAtSelection<P2>(&self, ec: TfEditCookie, dwflags: u32, pdataobject: P2) -> windows_core::Result<ITfRange>
    where
        P2: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertEmbeddedAtSelection)(windows_core::Interface::as_raw(self), ec, dwflags, pdataobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInsertAtSelection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InsertTextAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *const u16, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    InsertEmbeddedAtSelection: usize,
}
#[cfg(feature = "objidl")]
pub trait ITfInsertAtSelection_Impl: windows_core::IUnknownImpl {
    fn InsertTextAtSelection(&self, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32) -> windows_core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(&self, ec: TfEditCookie, dwflags: u32, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<ITfRange>;
}
#[cfg(feature = "objidl")]
impl ITfInsertAtSelection_Vtbl {
    pub const fn new<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertTextAtSelection<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInsertAtSelection_Impl::InsertTextAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ITfInsertAtSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dwflags: u32, pdataobject: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfInsertAtSelection_Impl::InsertEmbeddedAtSelection(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject)) {
                    Ok(ok__) => {
                        pprange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InsertTextAtSelection: InsertTextAtSelection::<Identity, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfInsertAtSelection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for ITfInsertAtSelection {}
windows_core::imp::define_interface!(ITfKeyEventSink, ITfKeyEventSink_Vtbl, 0xaa80e7f5_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfKeyEventSink, windows_core::IUnknown);
impl ITfKeyEventSink {
    pub unsafe fn OnSetFocus(&self, fforeground: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), fforeground.into()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnTestKeyDown<P0>(&self, pic: P0, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTestKeyDown)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnTestKeyUp<P0>(&self, pic: P0, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTestKeyUp)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyDown<P0>(&self, pic: P0, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnKeyDown)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyUp<P0>(&self, pic: P0, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnKeyUp)(windows_core::Interface::as_raw(self), pic.param().abi(), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnPreservedKey<P0>(&self, pic: P0, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnPreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), rguid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub OnTestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub OnTestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnTestKeyUp: usize,
    #[cfg(feature = "minwindef")]
    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub OnKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyUp: usize,
    pub OnPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait ITfKeyEventSink_Impl: windows_core::IUnknownImpl {
    fn OnSetFocus(&self, fforeground: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTestKeyDown(&self, pic: windows_core::Ref<ITfContext>, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnTestKeyUp(&self, pic: windows_core::Ref<ITfContext>, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnKeyDown(&self, pic: windows_core::Ref<ITfContext>, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnKeyUp(&self, pic: windows_core::Ref<ITfContext>, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn OnPreservedKey(&self, pic: windows_core::Ref<ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "minwindef")]
impl ITfKeyEventSink_Vtbl {
    pub const fn new<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetFocus<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforeground: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeyEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&fforeground)).into()
            }
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeyEventSink_Impl::OnTestKeyDown(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeyEventSink_Impl::OnTestKeyUp(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnKeyDown<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeyEventSink_Impl::OnKeyDown(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnKeyUp<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeyEventSink_Impl::OnKeyUp(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnPreservedKey<Identity: ITfKeyEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeyEventSink_Impl::OnPreservedKey(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, OFFSET>,
            OnPreservedKey: OnPreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for ITfKeyEventSink {}
windows_core::imp::define_interface!(ITfKeyTraceEventSink, ITfKeyTraceEventSink_Vtbl, 0x1cd4c13b_1c36_4191_a70a_7f3e611f367d);
windows_core::imp::interface_hierarchy!(ITfKeyTraceEventSink, windows_core::IUnknown);
impl ITfKeyTraceEventSink {
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyTraceDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnKeyTraceDown)(windows_core::Interface::as_raw(self), wparam, lparam) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn OnKeyTraceUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnKeyTraceUp)(windows_core::Interface::as_raw(self), wparam, lparam) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyTraceEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub OnKeyTraceDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyTraceDown: usize,
    #[cfg(feature = "minwindef")]
    pub OnKeyTraceUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    OnKeyTraceUp: usize,
}
#[cfg(feature = "minwindef")]
pub trait ITfKeyTraceEventSink_Impl: windows_core::IUnknownImpl {
    fn OnKeyTraceDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<()>;
    fn OnKeyTraceUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl ITfKeyTraceEventSink_Vtbl {
    pub const fn new<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnKeyTraceDown<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeyTraceEventSink_Impl::OnKeyTraceDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn OnKeyTraceUp<Identity: ITfKeyTraceEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeyTraceEventSink_Impl::OnKeyTraceUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnKeyTraceDown: OnKeyTraceDown::<Identity, OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeyTraceEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for ITfKeyTraceEventSink {}
windows_core::imp::define_interface!(ITfKeystrokeMgr, ITfKeystrokeMgr_Vtbl, 0xaa80e7f0_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfKeystrokeMgr, windows_core::IUnknown);
impl ITfKeystrokeMgr {
    pub unsafe fn AdviseKeyEventSink<P1>(&self, tid: TfClientId, psink: P1, fforeground: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfKeyEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseKeyEventSink)(windows_core::Interface::as_raw(self), tid, psink.param().abi(), fforeground.into()) }
    }
    pub unsafe fn UnadviseKeyEventSink(&self, tid: TfClientId) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseKeyEventSink)(windows_core::Interface::as_raw(self), tid) }
    }
    pub unsafe fn GetForeground(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForeground)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn TestKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TestKeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn TestKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TestKeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn KeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyDown)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn KeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyUp)(windows_core::Interface::as_raw(self), wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPreservedKey<P0>(&self, pic: P0, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), pprekey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsPreservedKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreservedKey)(windows_core::Interface::as_raw(self), rguid, pprekey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PreserveKey(&self, tid: TfClientId, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: *const u16, cchdesc: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreserveKey)(windows_core::Interface::as_raw(self), tid, rguid, prekey, pchdesc, cchdesc) }
    }
    pub unsafe fn UnpreserveKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnpreserveKey)(windows_core::Interface::as_raw(self), rguid, pprekey) }
    }
    pub unsafe fn SetPreservedKeyDescription(&self, rguid: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreservedKeyDescription)(windows_core::Interface::as_raw(self), rguid, pchdesc, cchdesc) }
    }
    pub unsafe fn GetPreservedKeyDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreservedKeyDescription)(windows_core::Interface::as_raw(self), rguid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SimulatePreservedKey<P0>(&self, pic: P0, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SimulatePreservedKey)(windows_core::Interface::as_raw(self), pic.param().abi(), rguid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeystrokeMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseKeyEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub UnadviseKeyEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId) -> windows_core::HRESULT,
    pub GetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub TestKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    TestKeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub TestKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    TestKeyUp: usize,
    #[cfg(feature = "minwindef")]
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    KeyDown: usize,
    #[cfg(feature = "minwindef")]
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    KeyUp: usize,
    pub GetPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const TF_PRESERVEDKEY, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsPreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const TF_PRESERVEDKEY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub PreserveKey: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *const windows_core::GUID, *const TF_PRESERVEDKEY, *const u16, u32) -> windows_core::HRESULT,
    pub UnpreserveKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const TF_PRESERVEDKEY) -> windows_core::HRESULT,
    pub SetPreservedKeyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const u16, u32) -> windows_core::HRESULT,
    pub GetPreservedKeyDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SimulatePreservedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait ITfKeystrokeMgr_Impl: windows_core::IUnknownImpl {
    fn AdviseKeyEventSink(&self, tid: TfClientId, psink: windows_core::Ref<ITfKeyEventSink>, fforeground: windows_core::BOOL) -> windows_core::Result<()>;
    fn UnadviseKeyEventSink(&self, tid: TfClientId) -> windows_core::Result<()>;
    fn GetForeground(&self) -> windows_core::Result<windows_core::GUID>;
    fn TestKeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn TestKeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn KeyDown(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn KeyUp(&self, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<windows_core::BOOL>;
    fn GetPreservedKey(&self, pic: windows_core::Ref<ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::GUID>;
    fn IsPreservedKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<windows_core::BOOL>;
    fn PreserveKey(&self, tid: TfClientId, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: *const u16, cchdesc: u32) -> windows_core::Result<()>;
    fn UnpreserveKey(&self, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
    fn SetPreservedKeyDescription(&self, rguid: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32) -> windows_core::Result<()>;
    fn GetPreservedKeyDescription(&self, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BSTR>;
    fn SimulatePreservedKey(&self, pic: windows_core::Ref<ITfContext>, rguid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "minwindef")]
impl ITfKeystrokeMgr_Vtbl {
    pub const fn new<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseKeyEventSink<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, psink: *mut core::ffi::c_void, fforeground: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeystrokeMgr_Impl::AdviseKeyEventSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&psink), core::mem::transmute_copy(&fforeground)).into()
            }
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeystrokeMgr_Impl::UnadviseKeyEventSink(this, core::mem::transmute_copy(&tid)).into()
            }
        }
        unsafe extern "system" fn GetForeground<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::GetForeground(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TestKeyDown<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::TestKeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TestKeyUp<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::TestKeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyDown<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::KeyDown(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyUp<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::KeyUp(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::GetPreservedKey(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&pprekey)) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::IsPreservedKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)) {
                    Ok(ok__) => {
                        pfregistered.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreserveKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, rguid: *const windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: *const u16, cchdesc: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeystrokeMgr_Impl::PreserveKey(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&prekey), core::mem::transmute_copy(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
            }
        }
        unsafe extern "system" fn UnpreserveKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeystrokeMgr_Impl::UnpreserveKey(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pprekey)).into()
            }
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pchdesc: *const u16, cchdesc: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfKeystrokeMgr_Impl::SetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&pchdesc), core::mem::transmute_copy(&cchdesc)).into()
            }
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pbstrdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::GetPreservedKeyDescription(this, core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pbstrdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SimulatePreservedKey<Identity: ITfKeystrokeMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, rguid: *const windows_core::GUID, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfKeystrokeMgr_Impl::SimulatePreservedKey(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&rguid)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseKeyEventSink: AdviseKeyEventSink::<Identity, OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Identity, OFFSET>,
            GetForeground: GetForeground::<Identity, OFFSET>,
            TestKeyDown: TestKeyDown::<Identity, OFFSET>,
            TestKeyUp: TestKeyUp::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            GetPreservedKey: GetPreservedKey::<Identity, OFFSET>,
            IsPreservedKey: IsPreservedKey::<Identity, OFFSET>,
            PreserveKey: PreserveKey::<Identity, OFFSET>,
            UnpreserveKey: UnpreserveKey::<Identity, OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Identity, OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Identity, OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfKeystrokeMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for ITfKeystrokeMgr {}
windows_core::imp::define_interface!(ITfLanguageProfileNotifySink, ITfLanguageProfileNotifySink_Vtbl, 0x43c9fe15_f494_4c17_9de2_b8a4ac350aa8);
windows_core::imp::interface_hierarchy!(ITfLanguageProfileNotifySink, windows_core::IUnknown);
impl ITfLanguageProfileNotifySink {
    #[cfg(feature = "winnt")]
    pub unsafe fn OnLanguageChange(&self, langid: super::winnt::LANGID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnLanguageChange)(windows_core::Interface::as_raw(self), langid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnLanguageChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLanguageChanged)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLanguageProfileNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub OnLanguageChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OnLanguageChange: usize,
    pub OnLanguageChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait ITfLanguageProfileNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnLanguageChange(&self, langid: super::winnt::LANGID) -> windows_core::Result<windows_core::BOOL>;
    fn OnLanguageChanged(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ITfLanguageProfileNotifySink_Vtbl {
    pub const fn new<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLanguageChange<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, pfaccept: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfLanguageProfileNotifySink_Impl::OnLanguageChange(this, core::mem::transmute_copy(&langid)) {
                    Ok(ok__) => {
                        pfaccept.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnLanguageChanged<Identity: ITfLanguageProfileNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfLanguageProfileNotifySink_Impl::OnLanguageChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLanguageChange: OnLanguageChange::<Identity, OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfLanguageProfileNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ITfLanguageProfileNotifySink {}
windows_core::imp::define_interface!(ITfMessagePump, ITfMessagePump_Vtbl, 0x8f1b8ad8_0b6b_4874_90c5_bd76011e8f7c);
windows_core::imp::interface_hierarchy!(ITfMessagePump, windows_core::IUnknown);
impl ITfMessagePump {
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn PeekMessage(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PeekMessage)(windows_core::Interface::as_raw(self), pmsg as _, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg, pfresult as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn GetMessage(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), pmsg as _, hwnd, wmsgfiltermin, wmsgfiltermax, pfresult as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn PeekMessageW(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PeekMessageW)(windows_core::Interface::as_raw(self), pmsg as _, hwnd, wmsgfiltermin, wmsgfiltermax, wremovemsg, pfresult as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn GetMessageW(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMessageW)(windows_core::Interface::as_raw(self), pmsg as _, hwnd, wmsgfiltermin, wmsgfiltermax, pfresult as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMessagePump_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub PeekMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winuser::MSG, super::windef::HWND, u32, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    PeekMessage: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winuser::MSG, super::windef::HWND, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    GetMessage: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub PeekMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winuser::MSG, super::windef::HWND, u32, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    PeekMessageW: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub GetMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winuser::MSG, super::windef::HWND, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    GetMessageW: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait ITfMessagePump_Impl: windows_core::IUnknownImpl {
    fn PeekMessage(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMessage(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn PeekMessageW(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMessageW(&self, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl ITfMessagePump_Vtbl {
    pub const fn new<Identity: ITfMessagePump_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PeekMessage<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMessagePump_Impl::PeekMessage(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMessagePump_Impl::GetMessage(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
            }
        }
        unsafe extern "system" fn PeekMessageW<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMessagePump_Impl::PeekMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&wremovemsg), core::mem::transmute_copy(&pfresult)).into()
            }
        }
        unsafe extern "system" fn GetMessageW<Identity: ITfMessagePump_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut super::winuser::MSG, hwnd: super::windef::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMessagePump_Impl::GetMessageW(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&wmsgfiltermin), core::mem::transmute_copy(&wmsgfiltermax), core::mem::transmute_copy(&pfresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PeekMessage: PeekMessage::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            PeekMessageW: PeekMessageW::<Identity, OFFSET>,
            GetMessageW: GetMessageW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMessagePump as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for ITfMessagePump {}
windows_core::imp::define_interface!(ITfMouseSink, ITfMouseSink_Vtbl, 0xa1adaaa2_3a24_449d_ac96_5183e7f5c217);
windows_core::imp::interface_hierarchy!(ITfMouseSink, windows_core::IUnknown);
impl ITfMouseSink {
    pub unsafe fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnMouseEvent)(windows_core::Interface::as_raw(self), uedge, uquadrant, dwbtnstatus, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMouseEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITfMouseSink_Impl: windows_core::IUnknownImpl {
    fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> windows_core::Result<windows_core::BOOL>;
}
impl ITfMouseSink_Vtbl {
    pub const fn new<Identity: ITfMouseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMouseEvent<Identity: ITfMouseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfMouseSink_Impl::OnMouseEvent(this, core::mem::transmute_copy(&uedge), core::mem::transmute_copy(&uquadrant), core::mem::transmute_copy(&dwbtnstatus)) {
                    Ok(ok__) => {
                        pfeaten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMouseEvent: OnMouseEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseSink {}
windows_core::imp::define_interface!(ITfMouseTracker, ITfMouseTracker_Vtbl, 0x09d146cd_a544_4132_925b_7afa8ef322d0);
windows_core::imp::interface_hierarchy!(ITfMouseTracker, windows_core::IUnknown);
impl ITfMouseTracker {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ITfRange>,
        P1: windows_core::Param<ITfMouseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdviseMouseSink)(windows_core::Interface::as_raw(self), range.param().abi(), psink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseMouseSink)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTracker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfMouseTracker_Impl: windows_core::IUnknownImpl {
    fn AdviseMouseSink(&self, range: windows_core::Ref<ITfRange>, psink: windows_core::Ref<ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfMouseTracker_Vtbl {
    pub const fn new<Identity: ITfMouseTracker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseMouseSink<Identity: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfMouseTracker_Impl::AdviseMouseSink(this, core::mem::transmute_copy(&range), core::mem::transmute_copy(&psink)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ITfMouseTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMouseTracker_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTracker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseTracker {}
windows_core::imp::define_interface!(ITfMouseTrackerACP, ITfMouseTrackerACP_Vtbl, 0x3bdd78e2_c16e_47fd_b883_ce6facc1a208);
windows_core::imp::interface_hierarchy!(ITfMouseTrackerACP, windows_core::IUnknown);
impl ITfMouseTrackerACP {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ITfRangeACP>,
        P1: windows_core::Param<ITfMouseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdviseMouseSink)(windows_core::Interface::as_raw(self), range.param().abi(), psink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseMouseSink)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTrackerACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfMouseTrackerACP_Impl: windows_core::IUnknownImpl {
    fn AdviseMouseSink(&self, range: windows_core::Ref<ITfRangeACP>, psink: windows_core::Ref<ITfMouseSink>) -> windows_core::Result<u32>;
    fn UnadviseMouseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfMouseTrackerACP_Vtbl {
    pub const fn new<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseMouseSink<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfMouseTrackerACP_Impl::AdviseMouseSink(this, core::mem::transmute_copy(&range), core::mem::transmute_copy(&psink)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ITfMouseTrackerACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfMouseTrackerACP_Impl::UnadviseMouseSink(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseMouseSink: AdviseMouseSink::<Identity, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfMouseTrackerACP as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfMouseTrackerACP {}
windows_core::imp::define_interface!(ITfPersistentPropertyLoaderACP, ITfPersistentPropertyLoaderACP_Vtbl, 0x4ef89150_0807_11d3_8df0_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfPersistentPropertyLoaderACP, windows_core::IUnknown);
impl ITfPersistentPropertyLoaderACP {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadProperty)(windows_core::Interface::as_raw(self), phdr, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPersistentPropertyLoaderACP_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub LoadProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_PERSISTENT_PROPERTY_HEADER_ACP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    LoadProperty: usize,
}
#[cfg(feature = "objidlbase")]
pub trait ITfPersistentPropertyLoaderACP_Impl: windows_core::IUnknownImpl {
    fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> windows_core::Result<super::objidlbase::IStream>;
}
#[cfg(feature = "objidlbase")]
impl ITfPersistentPropertyLoaderACP_Vtbl {
    pub const fn new<Identity: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadProperty<Identity: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPersistentPropertyLoaderACP_Impl::LoadProperty(this, core::mem::transmute_copy(&phdr)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadProperty: LoadProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPersistentPropertyLoaderACP as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ITfPersistentPropertyLoaderACP {}
windows_core::imp::define_interface!(ITfPreservedKeyNotifySink, ITfPreservedKeyNotifySink_Vtbl, 0x6f77c993_d2b1_446e_853e_5912efc8a286);
windows_core::imp::interface_hierarchy!(ITfPreservedKeyNotifySink, windows_core::IUnknown);
impl ITfPreservedKeyNotifySink {
    pub unsafe fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUpdated)(windows_core::Interface::as_raw(self), pprekey) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPreservedKeyNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *const TF_PRESERVEDKEY) -> windows_core::HRESULT,
}
pub trait ITfPreservedKeyNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> windows_core::Result<()>;
}
impl ITfPreservedKeyNotifySink_Vtbl {
    pub const fn new<Identity: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdated<Identity: ITfPreservedKeyNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfPreservedKeyNotifySink_Impl::OnUpdated(this, core::mem::transmute_copy(&pprekey)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUpdated: OnUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPreservedKeyNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfPreservedKeyNotifySink {}
windows_core::imp::define_interface!(ITfProperty, ITfProperty_Vtbl, 0xe2449660_9542_11d2_bf46_00105a2799b5);
impl core::ops::Deref for ITfProperty {
    type Target = ITfReadOnlyProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfProperty, windows_core::IUnknown, ITfReadOnlyProperty);
impl ITfProperty {
    pub unsafe fn FindRange<P1>(&self, ec: TfEditCookie, prange: P1, pprange: *mut Option<ITfRange>, apos: TfAnchor) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), core::mem::transmute(pprange), apos) }
    }
    pub unsafe fn SetValueStore<P1, P2>(&self, ec: TfEditCookie, prange: P1, ppropstore: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
        P2: windows_core::Param<ITfPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValueStore)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), ppropstore.param().abi()) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue<P1>(&self, ec: TfEditCookie, prange: P1, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), pvarvalue) }
    }
    pub unsafe fn Clear<P1>(&self, ec: TfEditCookie, prange: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), ec, prange.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfProperty_Vtbl {
    pub base__: ITfReadOnlyProperty_Vtbl,
    pub FindRange: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub SetValueStore: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITfProperty_Impl: ITfReadOnlyProperty_Impl {
    fn FindRange(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, pprange: windows_core::OutRef<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn SetValueStore(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, ppropstore: windows_core::Ref<ITfPropertyStore>) -> windows_core::Result<()>;
    fn SetValue(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITfProperty_Vtbl {
    pub const fn new<Identity: ITfProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindRange<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, pprange: *mut *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfProperty_Impl::FindRange(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&pprange), core::mem::transmute_copy(&apos)).into()
            }
        }
        unsafe extern "system" fn SetValueStore<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, ppropstore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfProperty_Impl::SetValueStore(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&ppropstore)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfProperty_Impl::SetValue(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITfProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfProperty_Impl::Clear(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange)).into()
            }
        }
        Self {
            base__: ITfReadOnlyProperty_Vtbl::new::<Identity, OFFSET>(),
            FindRange: FindRange::<Identity, OFFSET>,
            SetValueStore: SetValueStore::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfProperty as windows_core::Interface>::IID || iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITfProperty {}
windows_core::imp::define_interface!(ITfPropertyStore, ITfPropertyStore_Vtbl, 0x6834b120_88cb_11d2_bf45_00105a2799b5);
windows_core::imp::interface_hierarchy!(ITfPropertyStore, windows_core::IUnknown);
impl ITfPropertyStore {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetData(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OnTextUpdated<P1>(&self, dwflags: u32, prangenew: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTextUpdated)(windows_core::Interface::as_raw(self), dwflags, prangenew.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Shrink<P0>(&self, prangenew: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shrink)(windows_core::Interface::as_raw(self), prangenew.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Divide<P0, P1>(&self, prangethis: P0, prangenew: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<ITfRange>,
        P1: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Divide)(windows_core::Interface::as_raw(self), prangethis.param().abi(), prangenew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyRangeCreator(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyRangeCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Serialize<P0>(&self, pstream: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pstream.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetData: usize,
    pub OnTextUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Shrink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Divide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyRangeCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Serialize: usize,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITfPropertyStore_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDataType(&self) -> windows_core::Result<u32>;
    fn GetData(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn OnTextUpdated(&self, dwflags: u32, prangenew: windows_core::Ref<ITfRange>) -> windows_core::Result<windows_core::BOOL>;
    fn Shrink(&self, prangenew: windows_core::Ref<ITfRange>) -> windows_core::Result<windows_core::BOOL>;
    fn Divide(&self, prangethis: windows_core::Ref<ITfRange>, prangenew: windows_core::Ref<ITfRange>) -> windows_core::Result<ITfPropertyStore>;
    fn Clone(&self) -> windows_core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(&self) -> windows_core::Result<windows_core::GUID>;
    fn Serialize(&self, pstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl ITfPropertyStore_Vtbl {
    pub const fn new<Identity: ITfPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataType<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::GetDataType(this) {
                    Ok(ok__) => {
                        pdwreserved.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetData<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::GetData(this) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnTextUpdated<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, prangenew: *mut core::ffi::c_void, pfaccept: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::OnTextUpdated(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&prangenew)) {
                    Ok(ok__) => {
                        pfaccept.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shrink<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pffree: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::Shrink(this, core::mem::transmute_copy(&prangenew)) {
                    Ok(ok__) => {
                        pffree.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Divide<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangethis: *mut core::ffi::c_void, prangenew: *mut core::ffi::c_void, pppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::Divide(this, core::mem::transmute_copy(&prangethis), core::mem::transmute_copy(&prangenew)) {
                    Ok(ok__) => {
                        pppropstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clone<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppropstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::GetPropertyRangeCreator(this) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Serialize<Identity: ITfPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfPropertyStore_Impl::Serialize(this, core::mem::transmute_copy(&pstream)) {
                    Ok(ok__) => {
                        pcb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDataType: GetDataType::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            OnTextUpdated: OnTextUpdated::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
            Divide: Divide::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITfPropertyStore {}
windows_core::imp::define_interface!(ITfQueryEmbedded, ITfQueryEmbedded_Vtbl, 0x0fab9bdb_d250_4169_84e5_6be118fdd7a8);
windows_core::imp::interface_hierarchy!(ITfQueryEmbedded, windows_core::IUnknown);
impl ITfQueryEmbedded {
    #[cfg(all(feature = "objidl", feature = "wtypes"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryInsertEmbedded)(windows_core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfQueryEmbedded_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "wtypes"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::objidl::FORMATETC, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "wtypes")))]
    QueryInsertEmbedded: usize,
}
#[cfg(all(feature = "objidl", feature = "wtypes"))]
pub trait ITfQueryEmbedded_Impl: windows_core::IUnknownImpl {
    fn QueryInsertEmbedded(&self, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "objidl", feature = "wtypes"))]
impl ITfQueryEmbedded_Vtbl {
    pub const fn new<Identity: ITfQueryEmbedded_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ITfQueryEmbedded_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidservice: *const windows_core::GUID, pformatetc: *const super::objidl::FORMATETC, pfinsertable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfQueryEmbedded_Impl::QueryInsertEmbedded(this, core::mem::transmute_copy(&pguidservice), core::mem::transmute_copy(&pformatetc)) {
                    Ok(ok__) => {
                        pfinsertable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInsertEmbedded: QueryInsertEmbedded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfQueryEmbedded as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "wtypes"))]
impl windows_core::RuntimeName for ITfQueryEmbedded {}
windows_core::imp::define_interface!(ITfRange, ITfRange_Vtbl, 0xaa80e7ff_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfRange, windows_core::IUnknown);
impl ITfRange {
    pub unsafe fn GetText(&self, ec: TfEditCookie, dwflags: u32, pchtext: *mut u16, cchmax: u32, pcch: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), ec, dwflags, pchtext as _, cchmax, pcch as _) }
    }
    pub unsafe fn SetText(&self, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), ec, dwflags, pchtext, cch) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetFormattedText(&self, ec: TfEditCookie) -> windows_core::Result<super::objidl::IDataObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormattedText)(windows_core::Interface::as_raw(self), ec, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEmbedded<T>(&self, ec: TfEditCookie, rguidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetEmbedded)(windows_core::Interface::as_raw(self), ec, rguidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn InsertEmbedded<P2>(&self, ec: TfEditCookie, dwflags: u32, pdataobject: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::objidl::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEmbedded)(windows_core::Interface::as_raw(self), ec, dwflags, pdataobject.param().abi()) }
    }
    pub unsafe fn ShiftStart(&self, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShiftStart)(windows_core::Interface::as_raw(self), ec, cchreq, pcch as _, phalt) }
    }
    pub unsafe fn ShiftEnd(&self, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShiftEnd)(windows_core::Interface::as_raw(self), ec, cchreq, pcch as _, phalt) }
    }
    pub unsafe fn ShiftStartToRange<P1>(&self, ec: TfEditCookie, prange: P1, apos: TfAnchor) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShiftStartToRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), apos) }
    }
    pub unsafe fn ShiftEndToRange<P1>(&self, ec: TfEditCookie, prange: P1, apos: TfAnchor) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShiftEndToRange)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), apos) }
    }
    pub unsafe fn ShiftStartRegion(&self, ec: TfEditCookie, dir: TfShiftDir) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShiftStartRegion)(windows_core::Interface::as_raw(self), ec, dir, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ShiftEndRegion(&self, ec: TfEditCookie, dir: TfShiftDir) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShiftEndRegion)(windows_core::Interface::as_raw(self), ec, dir, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEmpty(&self, ec: TfEditCookie) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEmpty)(windows_core::Interface::as_raw(self), ec, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Collapse(&self, ec: TfEditCookie, apos: TfAnchor) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self), ec, apos) }
    }
    pub unsafe fn IsEqualStart<P1>(&self, ec: TfEditCookie, pwith: P1, apos: TfAnchor) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqualStart)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEqualEnd<P1>(&self, ec: TfEditCookie, pwith: P1, apos: TfAnchor) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqualEnd)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CompareStart<P1>(&self, ec: TfEditCookie, pwith: P1, apos: TfAnchor) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareStart)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CompareEnd<P1>(&self, ec: TfEditCookie, pwith: P1, apos: TfAnchor) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareEnd)(windows_core::Interface::as_raw(self), ec, pwith.param().abi(), apos, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AdjustForInsert(&self, ec: TfEditCookie, cchinsert: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdjustForInsert)(windows_core::Interface::as_raw(self), ec, cchinsert, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGravity)(windows_core::Interface::as_raw(self), pgstart as _, pgend as _) }
    }
    pub unsafe fn SetGravity(&self, ec: TfEditCookie, gstart: TfGravity, gend: TfGravity) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGravity)(windows_core::Interface::as_raw(self), ec, gstart, gend) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *mut u16, u32, *mut u32) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *const u16, i32) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub GetFormattedText: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub InsertEmbedded: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    InsertEmbedded: usize,
    pub ShiftStart: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, i32, *mut i32, *const TF_HALTCOND) -> windows_core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, i32, *mut i32, *const TF_HALTCOND) -> windows_core::HRESULT,
    pub ShiftStartToRange: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub ShiftEndToRange: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor) -> windows_core::HRESULT,
    pub ShiftStartRegion: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, TfShiftDir, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ShiftEndRegion: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, TfShiftDir, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, TfAnchor) -> windows_core::HRESULT,
    pub IsEqualStart: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsEqualEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CompareStart: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor, *mut i32) -> windows_core::HRESULT,
    pub CompareEnd: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, TfAnchor, *mut i32) -> windows_core::HRESULT,
    pub AdjustForInsert: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfGravity, *mut TfGravity) -> windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, TfGravity, TfGravity) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait ITfRange_Impl: windows_core::IUnknownImpl {
    fn GetText(&self, ec: TfEditCookie, dwflags: u32, pchtext: *mut u16, cchmax: u32, pcch: *mut u32) -> windows_core::Result<()>;
    fn SetText(&self, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32) -> windows_core::Result<()>;
    fn GetFormattedText(&self, ec: TfEditCookie) -> windows_core::Result<super::objidl::IDataObject>;
    fn GetEmbedded(&self, ec: TfEditCookie, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertEmbedded(&self, ec: TfEditCookie, dwflags: u32, pdataobject: windows_core::Ref<super::objidl::IDataObject>) -> windows_core::Result<()>;
    fn ShiftStart(&self, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftEnd(&self, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::Result<()>;
    fn ShiftStartToRange(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftEndToRange(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<()>;
    fn ShiftStartRegion(&self, ec: TfEditCookie, dir: TfShiftDir) -> windows_core::Result<windows_core::BOOL>;
    fn ShiftEndRegion(&self, ec: TfEditCookie, dir: TfShiftDir) -> windows_core::Result<windows_core::BOOL>;
    fn IsEmpty(&self, ec: TfEditCookie) -> windows_core::Result<windows_core::BOOL>;
    fn Collapse(&self, ec: TfEditCookie, apos: TfAnchor) -> windows_core::Result<()>;
    fn IsEqualStart(&self, ec: TfEditCookie, pwith: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<windows_core::BOOL>;
    fn IsEqualEnd(&self, ec: TfEditCookie, pwith: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<windows_core::BOOL>;
    fn CompareStart(&self, ec: TfEditCookie, pwith: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn CompareEnd(&self, ec: TfEditCookie, pwith: windows_core::Ref<ITfRange>, apos: TfAnchor) -> windows_core::Result<i32>;
    fn AdjustForInsert(&self, ec: TfEditCookie, cchinsert: u32) -> windows_core::Result<windows_core::BOOL>;
    fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::Result<()>;
    fn SetGravity(&self, ec: TfEditCookie, gstart: TfGravity, gend: TfGravity) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ITfRange>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
#[cfg(feature = "objidl")]
impl ITfRange_Vtbl {
    pub const fn new<Identity: ITfRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dwflags: u32, pchtext: *mut u16, cchmax: u32, pcch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::GetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&pcch)).into()
            }
        }
        unsafe extern "system" fn SetText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dwflags: u32, pchtext: *const u16, cch: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::SetText(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchtext), core::mem::transmute_copy(&cch)).into()
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::GetFormattedText(this, core::mem::transmute_copy(&ec)) {
                    Ok(ok__) => {
                        ppdataobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEmbedded<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, rguidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::GetEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&rguidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dwflags: u32, pdataobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::InsertEmbedded(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdataobject)).into()
            }
        }
        unsafe extern "system" fn ShiftStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::ShiftStart(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
            }
        }
        unsafe extern "system" fn ShiftEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::ShiftEnd(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchreq), core::mem::transmute_copy(&pcch), core::mem::transmute_copy(&phalt)).into()
            }
        }
        unsafe extern "system" fn ShiftStartToRange<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::ShiftStartToRange(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&apos)).into()
            }
        }
        unsafe extern "system" fn ShiftEndToRange<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, apos: TfAnchor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::ShiftEndToRange(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange), core::mem::transmute_copy(&apos)).into()
            }
        }
        unsafe extern "system" fn ShiftStartRegion<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dir: TfShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::ShiftStartRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                    Ok(ok__) => {
                        pfnoregion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShiftEndRegion<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, dir: TfShiftDir, pfnoregion: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::ShiftEndRegion(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&dir)) {
                    Ok(ok__) => {
                        pfnoregion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEmpty<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, pfempty: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::IsEmpty(this, core::mem::transmute_copy(&ec)) {
                    Ok(ok__) => {
                        pfempty.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Collapse<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, apos: TfAnchor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::Collapse(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&apos)).into()
            }
        }
        unsafe extern "system" fn IsEqualStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::IsEqualStart(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&pwith), core::mem::transmute_copy(&apos)) {
                    Ok(ok__) => {
                        pfequal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqualEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, pwith: *mut core::ffi::c_void, apos: TfAnchor, pfequal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::IsEqualEnd(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&pwith), core::mem::transmute_copy(&apos)) {
                    Ok(ok__) => {
                        pfequal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareStart<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::CompareStart(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&pwith), core::mem::transmute_copy(&apos)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareEnd<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, pwith: *mut core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::CompareEnd(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&pwith), core::mem::transmute_copy(&apos)) {
                    Ok(ok__) => {
                        plresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdjustForInsert<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, cchinsert: u32, pfinsertok: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::AdjustForInsert(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&cchinsert)) {
                    Ok(ok__) => {
                        pfinsertok.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGravity<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::GetGravity(this, core::mem::transmute_copy(&pgstart), core::mem::transmute_copy(&pgend)).into()
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, gstart: TfGravity, gend: TfGravity) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRange_Impl::SetGravity(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&gstart), core::mem::transmute_copy(&gend)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppclone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfRange_Impl::GetContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, OFFSET>,
            ShiftStart: ShiftStart::<Identity, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Identity, OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Identity, OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Identity, OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Identity, OFFSET>,
            IsEmpty: IsEmpty::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            IsEqualStart: IsEqualStart::<Identity, OFFSET>,
            IsEqualEnd: IsEqualEnd::<Identity, OFFSET>,
            CompareStart: CompareStart::<Identity, OFFSET>,
            CompareEnd: CompareEnd::<Identity, OFFSET>,
            AdjustForInsert: AdjustForInsert::<Identity, OFFSET>,
            GetGravity: GetGravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for ITfRange {}
windows_core::imp::define_interface!(ITfRangeACP, ITfRangeACP_Vtbl, 0x057a6296_029b_4154_b79a_0d461d4ea94c);
impl core::ops::Deref for ITfRangeACP {
    type Target = ITfRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfRangeACP, windows_core::IUnknown, ITfRange);
impl ITfRangeACP {
    pub unsafe fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), pacpanchor as _, pcch as _) }
    }
    pub unsafe fn SetExtent(&self, acpanchor: i32, cch: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExtent)(windows_core::Interface::as_raw(self), acpanchor, cch) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeACP_Vtbl {
    pub base__: ITfRange_Vtbl,
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait ITfRangeACP_Impl: ITfRange_Impl {
    fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::Result<()>;
    fn SetExtent(&self, acpanchor: i32, cch: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl ITfRangeACP_Vtbl {
    pub const fn new<Identity: ITfRangeACP_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtent<Identity: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRangeACP_Impl::GetExtent(this, core::mem::transmute_copy(&pacpanchor), core::mem::transmute_copy(&pcch)).into()
            }
        }
        unsafe extern "system" fn SetExtent<Identity: ITfRangeACP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, acpanchor: i32, cch: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRangeACP_Impl::SetExtent(this, core::mem::transmute_copy(&acpanchor), core::mem::transmute_copy(&cch)).into()
            }
        }
        Self { base__: ITfRange_Vtbl::new::<Identity, OFFSET>(), GetExtent: GetExtent::<Identity, OFFSET>, SetExtent: SetExtent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeACP as windows_core::Interface>::IID || iid == &<ITfRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for ITfRangeACP {}
windows_core::imp::define_interface!(ITfRangeBackup, ITfRangeBackup_Vtbl, 0x463a506d_6992_49d2_9b88_93d55e70bb16);
windows_core::imp::interface_hierarchy!(ITfRangeBackup, windows_core::IUnknown);
impl ITfRangeBackup {
    pub unsafe fn Restore<P1>(&self, ec: TfEditCookie, prange: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self), ec, prange.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeBackup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfRangeBackup_Impl: windows_core::IUnknownImpl {
    fn Restore(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
}
impl ITfRangeBackup_Vtbl {
    pub const fn new<Identity: ITfRangeBackup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Restore<Identity: ITfRangeBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfRangeBackup_Impl::Restore(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Restore: Restore::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfRangeBackup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfRangeBackup {}
windows_core::imp::define_interface!(ITfReadOnlyProperty, ITfReadOnlyProperty_Vtbl, 0x17d49a3d_f8b8_4b2f_b254_52319dd64c53);
windows_core::imp::interface_hierarchy!(ITfReadOnlyProperty, windows_core::IUnknown);
impl ITfReadOnlyProperty {
    pub unsafe fn GetType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumRanges<P2>(&self, ec: TfEditCookie, ppenum: *mut Option<IEnumTfRanges>, ptargetrange: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<ITfRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumRanges)(windows_core::Interface::as_raw(self), ec, core::mem::transmute(ppenum), ptargetrange.param().abi()) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue<P1>(&self, ec: TfEditCookie, prange: P1) -> windows_core::Result<super::oaidl::VARIANT>
    where
        P1: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), ec, prange.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadOnlyProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EnumRanges: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITfReadOnlyProperty_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<windows_core::GUID>;
    fn EnumRanges(&self, ec: TfEditCookie, ppenum: windows_core::OutRef<IEnumTfRanges>, ptargetrange: windows_core::Ref<ITfRange>) -> windows_core::Result<()>;
    fn GetValue(&self, ec: TfEditCookie, prange: windows_core::Ref<ITfRange>) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITfReadOnlyProperty_Vtbl {
    pub const fn new<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadOnlyProperty_Impl::GetType(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRanges<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, ppenum: *mut *mut core::ffi::c_void, ptargetrange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfReadOnlyProperty_Impl::EnumRanges(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&ppenum), core::mem::transmute_copy(&ptargetrange)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ec: TfEditCookie, prange: *mut core::ffi::c_void, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadOnlyProperty_Impl::GetValue(this, core::mem::transmute_copy(&ec), core::mem::transmute_copy(&prange)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfReadOnlyProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadOnlyProperty_Impl::GetContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            EnumRanges: EnumRanges::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadOnlyProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITfReadOnlyProperty {}
windows_core::imp::define_interface!(ITfReadingInformationUIElement, ITfReadingInformationUIElement_Vtbl, 0xea1ea139_19df_11d7_a6d2_00065b84435c);
impl core::ops::Deref for ITfReadingInformationUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfReadingInformationUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfReadingInformationUIElement {
    pub unsafe fn GetUpdatedFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdatedFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<ITfContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetMaxReadingStringLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxReadingStringLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsVerticalOrderPreferred(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVerticalOrderPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadingInformationUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxReadingStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetErrorIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsVerticalOrderPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITfReadingInformationUIElement_Impl: ITfUIElement_Impl {
    fn GetUpdatedFlags(&self) -> windows_core::Result<u32>;
    fn GetContext(&self) -> windows_core::Result<ITfContext>;
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMaxReadingStringLength(&self) -> windows_core::Result<u32>;
    fn GetErrorIndex(&self) -> windows_core::Result<u32>;
    fn IsVerticalOrderPreferred(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ITfReadingInformationUIElement_Vtbl {
    pub const fn new<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::GetUpdatedFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::GetContext(this) {
                    Ok(ok__) => {
                        ppic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::GetString(this) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchmax: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::GetMaxReadingStringLength(this) {
                    Ok(ok__) => {
                        pcchmax.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorIndex<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::GetErrorIndex(this) {
                    Ok(ok__) => {
                        perrorindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Identity: ITfReadingInformationUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvertical: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReadingInformationUIElement_Impl::IsVerticalOrderPreferred(this) {
                    Ok(ok__) => {
                        pfvertical.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(),
            GetUpdatedFlags: GetUpdatedFlags::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Identity, OFFSET>,
            GetErrorIndex: GetErrorIndex::<Identity, OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReadingInformationUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReadingInformationUIElement {}
windows_core::imp::define_interface!(ITfReverseConversion, ITfReverseConversion_Vtbl, 0xa415e162_157d_417d_8a8c_0ab26c7d2781);
windows_core::imp::interface_hierarchy!(ITfReverseConversion, windows_core::IUnknown);
impl ITfReverseConversion {
    pub unsafe fn DoReverseConversion<P0>(&self, lpstr: P0) -> windows_core::Result<ITfReverseConversionList>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoReverseConversion)(windows_core::Interface::as_raw(self), lpstr.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoReverseConversion: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfReverseConversion_Impl: windows_core::IUnknownImpl {
    fn DoReverseConversion(&self, lpstr: &windows_core::PCWSTR) -> windows_core::Result<ITfReverseConversionList>;
}
impl ITfReverseConversion_Vtbl {
    pub const fn new<Identity: ITfReverseConversion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoReverseConversion<Identity: ITfReverseConversion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpstr: windows_core::PCWSTR, pplist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReverseConversion_Impl::DoReverseConversion(this, core::mem::transmute(&lpstr)) {
                    Ok(ok__) => {
                        pplist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoReverseConversion: DoReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversion as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReverseConversion {}
windows_core::imp::define_interface!(ITfReverseConversionList, ITfReverseConversionList_Vtbl, 0x151d69f0_86f4_4674_b721_56911e797f47);
windows_core::imp::interface_hierarchy!(ITfReverseConversionList, windows_core::IUnknown);
impl ITfReverseConversionList {
    pub unsafe fn GetLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfReverseConversionList_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> windows_core::Result<u32>;
    fn GetString(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfReverseConversionList_Vtbl {
    pub const fn new<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReverseConversionList_Impl::GetLength(this) {
                    Ok(ok__) => {
                        puindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: ITfReverseConversionList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReverseConversionList_Impl::GetString(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfReverseConversionList {}
windows_core::imp::define_interface!(ITfReverseConversionMgr, ITfReverseConversionMgr_Vtbl, 0xb643c236_c493_41b6_abb3_692412775cc4);
windows_core::imp::interface_hierarchy!(ITfReverseConversionMgr, windows_core::IUnknown);
impl ITfReverseConversionMgr {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetReverseConversion(&self, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflag: u32) -> windows_core::Result<ITfReverseConversion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReverseConversion)(windows_core::Interface::as_raw(self), langid, guidprofile, dwflag, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetReverseConversion: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LANGID, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetReverseConversion: usize,
}
#[cfg(feature = "winnt")]
pub trait ITfReverseConversionMgr_Impl: windows_core::IUnknownImpl {
    fn GetReverseConversion(&self, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflag: u32) -> windows_core::Result<ITfReverseConversion>;
}
#[cfg(feature = "winnt")]
impl ITfReverseConversionMgr_Vtbl {
    pub const fn new<Identity: ITfReverseConversionMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetReverseConversion<Identity: ITfReverseConversionMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: super::winnt::LANGID, guidprofile: *const windows_core::GUID, dwflag: u32, ppreverseconversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfReverseConversionMgr_Impl::GetReverseConversion(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&guidprofile), core::mem::transmute_copy(&dwflag)) {
                    Ok(ok__) => {
                        ppreverseconversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetReverseConversion: GetReverseConversion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfReverseConversionMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ITfReverseConversionMgr {}
windows_core::imp::define_interface!(ITfSource, ITfSource_Vtbl, 0x4ea48a35_60ae_446f_8fd6_e6a8d82459f7);
windows_core::imp::interface_hierarchy!(ITfSource, windows_core::IUnknown);
impl ITfSource {
    pub unsafe fn AdviseSink<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdviseSink)(windows_core::Interface::as_raw(self), riid, punk.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnadviseSink(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseSink)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfSource_Impl: windows_core::IUnknownImpl {
    fn AdviseSink(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn UnadviseSink(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl ITfSource_Vtbl {
    pub const fn new<Identity: ITfSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSink<Identity: ITfSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfSource_Impl::AdviseSink(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnadviseSink<Identity: ITfSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSource_Impl::UnadviseSink(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSink: AdviseSink::<Identity, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSource {}
windows_core::imp::define_interface!(ITfSourceSingle, ITfSourceSingle_Vtbl, 0x73131f9c_56a9_49dd_b0ee_d046633f7528);
windows_core::imp::interface_hierarchy!(ITfSourceSingle, windows_core::IUnknown);
impl ITfSourceSingle {
    pub unsafe fn AdviseSingleSink<P2>(&self, tid: TfClientId, riid: *const windows_core::GUID, punk: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseSingleSink)(windows_core::Interface::as_raw(self), tid, riid, punk.param().abi()) }
    }
    pub unsafe fn UnadviseSingleSink(&self, tid: TfClientId, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnadviseSingleSink)(windows_core::Interface::as_raw(self), tid, riid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSourceSingle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseSingleSink: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnadviseSingleSink: unsafe extern "system" fn(*mut core::ffi::c_void, TfClientId, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITfSourceSingle_Impl: windows_core::IUnknownImpl {
    fn AdviseSingleSink(&self, tid: TfClientId, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnadviseSingleSink(&self, tid: TfClientId, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITfSourceSingle_Vtbl {
    pub const fn new<Identity: ITfSourceSingle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseSingleSink<Identity: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSourceSingle_Impl::AdviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn UnadviseSingleSink<Identity: ITfSourceSingle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: TfClientId, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfSourceSingle_Impl::UnadviseSingleSink(this, core::mem::transmute_copy(&tid), core::mem::transmute_copy(&riid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseSingleSink: AdviseSingleSink::<Identity, OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfSourceSingle as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfSourceSingle {}
windows_core::imp::define_interface!(ITfStatusSink, ITfStatusSink_Vtbl, 0x6b7d8d73_b267_4f69_b32e_1ca321ce4f45);
windows_core::imp::interface_hierarchy!(ITfStatusSink, windows_core::IUnknown);
impl ITfStatusSink {
    pub unsafe fn OnStatusChange<P0>(&self, pic: P0, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), pic.param().abi(), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfStatusSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfStatusSink_Impl: windows_core::IUnknownImpl {
    fn OnStatusChange(&self, pic: windows_core::Ref<ITfContext>, dwflags: u32) -> windows_core::Result<()>;
}
impl ITfStatusSink_Vtbl {
    pub const fn new<Identity: ITfStatusSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatusChange<Identity: ITfStatusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfStatusSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatusChange: OnStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfStatusSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfStatusSink {}
windows_core::imp::define_interface!(ITfTextEditSink, ITfTextEditSink_Vtbl, 0x8127d409_ccd3_4683_967a_b43d5b482bf7);
windows_core::imp::interface_hierarchy!(ITfTextEditSink, windows_core::IUnknown);
impl ITfTextEditSink {
    pub unsafe fn OnEndEdit<P0, P2>(&self, pic: P0, ecreadonly: TfEditCookie, peditrecord: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfEditRecord>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEndEdit)(windows_core::Interface::as_raw(self), pic.param().abi(), ecreadonly, peditrecord.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextEditSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEndEdit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextEditSink_Impl: windows_core::IUnknownImpl {
    fn OnEndEdit(&self, pic: windows_core::Ref<ITfContext>, ecreadonly: TfEditCookie, peditrecord: windows_core::Ref<ITfEditRecord>) -> windows_core::Result<()>;
}
impl ITfTextEditSink_Vtbl {
    pub const fn new<Identity: ITfTextEditSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEndEdit<Identity: ITfTextEditSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: TfEditCookie, peditrecord: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfTextEditSink_Impl::OnEndEdit(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&ecreadonly), core::mem::transmute_copy(&peditrecord)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEndEdit: OnEndEdit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextEditSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextEditSink {}
windows_core::imp::define_interface!(ITfTextInputProcessor, ITfTextInputProcessor_Vtbl, 0xaa80e7f7_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfTextInputProcessor, windows_core::IUnknown);
impl ITfTextInputProcessor {
    pub unsafe fn Activate<P0>(&self, ptim: P0, tid: TfClientId) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfThreadMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), ptim.param().abi(), tid) }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfClientId) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextInputProcessor_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, ptim: windows_core::Ref<ITfThreadMgr>, tid: TfClientId) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
}
impl ITfTextInputProcessor_Vtbl {
    pub const fn new<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: TfClientId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfTextInputProcessor_Impl::Activate(this, core::mem::transmute_copy(&ptim), core::mem::transmute_copy(&tid)).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ITfTextInputProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfTextInputProcessor_Impl::Deactivate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextInputProcessor {}
windows_core::imp::define_interface!(ITfTextInputProcessorEx, ITfTextInputProcessorEx_Vtbl, 0x6e4e2102_f9cd_433d_b496_303ce03a6507);
impl core::ops::Deref for ITfTextInputProcessorEx {
    type Target = ITfTextInputProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfTextInputProcessorEx, windows_core::IUnknown, ITfTextInputProcessor);
impl ITfTextInputProcessorEx {
    pub unsafe fn ActivateEx<P0>(&self, ptim: P0, tid: TfClientId, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfThreadMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), ptim.param().abi(), tid, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessorEx_Vtbl {
    pub base__: ITfTextInputProcessor_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfClientId, u32) -> windows_core::HRESULT,
}
pub trait ITfTextInputProcessorEx_Impl: ITfTextInputProcessor_Impl {
    fn ActivateEx(&self, ptim: windows_core::Ref<ITfThreadMgr>, tid: TfClientId, dwflags: u32) -> windows_core::Result<()>;
}
impl ITfTextInputProcessorEx_Vtbl {
    pub const fn new<Identity: ITfTextInputProcessorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateEx<Identity: ITfTextInputProcessorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptim: *mut core::ffi::c_void, tid: TfClientId, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfTextInputProcessorEx_Impl::ActivateEx(this, core::mem::transmute_copy(&ptim), core::mem::transmute_copy(&tid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: ITfTextInputProcessor_Vtbl::new::<Identity, OFFSET>(), ActivateEx: ActivateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextInputProcessorEx as windows_core::Interface>::IID || iid == &<ITfTextInputProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextInputProcessorEx {}
windows_core::imp::define_interface!(ITfTextLayoutSink, ITfTextLayoutSink_Vtbl, 0x2af2d06a_dd5b_4927_a0b4_54f19c91fade);
windows_core::imp::interface_hierarchy!(ITfTextLayoutSink, windows_core::IUnknown);
impl ITfTextLayoutSink {
    pub unsafe fn OnLayoutChange<P0, P2>(&self, pic: P0, lcode: TfLayoutCode, pview: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfContextView>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLayoutChange)(windows_core::Interface::as_raw(self), pic.param().abi(), lcode, pview.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextLayoutSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfLayoutCode, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTextLayoutSink_Impl: windows_core::IUnknownImpl {
    fn OnLayoutChange(&self, pic: windows_core::Ref<ITfContext>, lcode: TfLayoutCode, pview: windows_core::Ref<ITfContextView>) -> windows_core::Result<()>;
}
impl ITfTextLayoutSink_Vtbl {
    pub const fn new<Identity: ITfTextLayoutSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLayoutChange<Identity: ITfTextLayoutSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, lcode: TfLayoutCode, pview: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfTextLayoutSink_Impl::OnLayoutChange(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&lcode), core::mem::transmute_copy(&pview)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLayoutChange: OnLayoutChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTextLayoutSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTextLayoutSink {}
windows_core::imp::define_interface!(ITfThreadFocusSink, ITfThreadFocusSink_Vtbl, 0xc0f1db0c_3a20_405c_a303_96b6010a885f);
windows_core::imp::interface_hierarchy!(ITfThreadFocusSink, windows_core::IUnknown);
impl ITfThreadFocusSink {
    pub unsafe fn OnSetThreadFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSetThreadFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnKillThreadFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnKillThreadFocus)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadFocusSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnKillThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadFocusSink_Impl: windows_core::IUnknownImpl {
    fn OnSetThreadFocus(&self) -> windows_core::Result<()>;
    fn OnKillThreadFocus(&self) -> windows_core::Result<()>;
}
impl ITfThreadFocusSink_Vtbl {
    pub const fn new<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetThreadFocus<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadFocusSink_Impl::OnSetThreadFocus(this).into()
            }
        }
        unsafe extern "system" fn OnKillThreadFocus<Identity: ITfThreadFocusSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadFocusSink_Impl::OnKillThreadFocus(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetThreadFocus: OnSetThreadFocus::<Identity, OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadFocusSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadFocusSink {}
windows_core::imp::define_interface!(ITfThreadMgr, ITfThreadMgr_Vtbl, 0xaa80e801_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfThreadMgr, windows_core::IUnknown);
impl ITfThreadMgr {
    pub unsafe fn Activate(&self) -> windows_core::Result<TfClientId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDocumentMgrs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AssociateFocus<P1>(&self, hwnd: super::windef::HWND, pdimnew: P1) -> windows_core::Result<ITfDocumentMgr>
    where
        P1: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociateFocus)(windows_core::Interface::as_raw(self), hwnd, pdimnew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsThreadFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThreadFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionProvider)(windows_core::Interface::as_raw(self), clsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFunctionProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalCompartment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfClientId) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AssociateFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AssociateFocus: usize,
    pub IsThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFunctionProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ITfThreadMgr_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<TfClientId>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<()>;
    fn AssociateFocus(&self, hwnd: super::windef::HWND, pdimnew: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
}
#[cfg(feature = "windef")]
impl ITfThreadMgr_Vtbl {
    pub const fn new<Identity: ITfThreadMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut TfClientId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::Activate(this) {
                    Ok(ok__) => {
                        ptid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr_Impl::Deactivate(this).into()
            }
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::CreateDocumentMgr(this) {
                    Ok(ok__) => {
                        ppdim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::EnumDocumentMgrs(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::GetFocus(this) {
                    Ok(ok__) => {
                        ppdimfocus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr_Impl::SetFocus(this, core::mem::transmute_copy(&pdimfocus)).into()
            }
        }
        unsafe extern "system" fn AssociateFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pdimnew: *mut core::ffi::c_void, ppdimprev: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::AssociateFocus(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdimnew)) {
                    Ok(ok__) => {
                        ppdimprev.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::IsThreadFocus(this) {
                    Ok(ok__) => {
                        pfthreadfocus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                    Ok(ok__) => {
                        ppfuncprov.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::EnumFunctionProviders(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ITfThreadMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr_Impl::GetGlobalCompartment(this) {
                    Ok(ok__) => {
                        ppcompmgr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            AssociateFocus: AssociateFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ITfThreadMgr {}
windows_core::imp::define_interface!(ITfThreadMgr2, ITfThreadMgr2_Vtbl, 0x0ab198ef_6477_4ee8_8812_6780edb82d5e);
windows_core::imp::interface_hierarchy!(ITfThreadMgr2, windows_core::IUnknown);
impl ITfThreadMgr2 {
    pub unsafe fn Activate(&self) -> windows_core::Result<TfClientId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDocumentMgrs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi()) }
    }
    pub unsafe fn IsThreadFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThreadFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFunctionProvider)(windows_core::Interface::as_raw(self), clsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFunctionProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalCompartment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut TfClientId, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), ptid as _, dwflags) }
    }
    pub unsafe fn GetActiveFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SuspendKeystrokeHandling(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SuspendKeystrokeHandling)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ResumeKeystrokeHandling(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResumeKeystrokeHandling)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfClientId) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsThreadFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFunctionProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfClientId, u32) -> windows_core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SuspendKeystrokeHandling: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeKeystrokeHandling: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadMgr2_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<TfClientId>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CreateDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(&self) -> windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(&self) -> windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(&self, pdimfocus: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<()>;
    fn IsThreadFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetFunctionProvider(&self, clsid: *const windows_core::GUID) -> windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(&self) -> windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(&self) -> windows_core::Result<ITfCompartmentMgr>;
    fn ActivateEx(&self, ptid: *mut TfClientId, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
    fn SuspendKeystrokeHandling(&self) -> windows_core::Result<()>;
    fn ResumeKeystrokeHandling(&self) -> windows_core::Result<()>;
}
impl ITfThreadMgr2_Vtbl {
    pub const fn new<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut TfClientId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::Activate(this) {
                    Ok(ok__) => {
                        ptid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr2_Impl::Deactivate(this).into()
            }
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::CreateDocumentMgr(this) {
                    Ok(ok__) => {
                        ppdim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::EnumDocumentMgrs(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdimfocus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::GetFocus(this) {
                    Ok(ok__) => {
                        ppdimfocus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr2_Impl::SetFocus(this, core::mem::transmute_copy(&pdimfocus)).into()
            }
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthreadfocus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::IsThreadFocus(this) {
                    Ok(ok__) => {
                        pfthreadfocus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, ppfuncprov: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::GetFunctionProvider(this, core::mem::transmute_copy(&clsid)) {
                    Ok(ok__) => {
                        ppfuncprov.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::EnumFunctionProviders(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcompmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::GetGlobalCompartment(this) {
                    Ok(ok__) => {
                        ppcompmgr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateEx<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut TfClientId, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr2_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgr2_Impl::GetActiveFlags(this) {
                    Ok(ok__) => {
                        lpdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr2_Impl::SuspendKeystrokeHandling(this).into()
            }
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Identity: ITfThreadMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgr2_Impl::ResumeKeystrokeHandling(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, OFFSET>,
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Identity, OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgr2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgr2 {}
windows_core::imp::define_interface!(ITfThreadMgrEventSink, ITfThreadMgrEventSink_Vtbl, 0xaa80e80e_2021_11d2_93e0_0060b067b86e);
windows_core::imp::interface_hierarchy!(ITfThreadMgrEventSink, windows_core::IUnknown);
impl ITfThreadMgrEventSink {
    pub unsafe fn OnInitDocumentMgr<P0>(&self, pdim: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInitDocumentMgr)(windows_core::Interface::as_raw(self), pdim.param().abi()) }
    }
    pub unsafe fn OnUninitDocumentMgr<P0>(&self, pdim: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUninitDocumentMgr)(windows_core::Interface::as_raw(self), pdim.param().abi()) }
    }
    pub unsafe fn OnSetFocus<P0, P1>(&self, pdimfocus: P0, pdimprevfocus: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfDocumentMgr>,
        P1: windows_core::Param<ITfDocumentMgr>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetFocus)(windows_core::Interface::as_raw(self), pdimfocus.param().abi(), pdimprevfocus.param().abi()) }
    }
    pub unsafe fn OnPushContext<P0>(&self, pic: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPushContext)(windows_core::Interface::as_raw(self), pic.param().abi()) }
    }
    pub unsafe fn OnPopContext<P0>(&self, pic: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPopContext)(windows_core::Interface::as_raw(self), pic.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInitDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUninitDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPushContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPopContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfThreadMgrEventSink_Impl: windows_core::IUnknownImpl {
    fn OnInitDocumentMgr(&self, pdim: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnUninitDocumentMgr(&self, pdim: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnSetFocus(&self, pdimfocus: windows_core::Ref<ITfDocumentMgr>, pdimprevfocus: windows_core::Ref<ITfDocumentMgr>) -> windows_core::Result<()>;
    fn OnPushContext(&self, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
    fn OnPopContext(&self, pic: windows_core::Ref<ITfContext>) -> windows_core::Result<()>;
}
impl ITfThreadMgrEventSink_Vtbl {
    pub const fn new<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInitDocumentMgr<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEventSink_Impl::OnInitDocumentMgr(this, core::mem::transmute_copy(&pdim)).into()
            }
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdim: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEventSink_Impl::OnUninitDocumentMgr(this, core::mem::transmute_copy(&pdim)).into()
            }
        }
        unsafe extern "system" fn OnSetFocus<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdimfocus: *mut core::ffi::c_void, pdimprevfocus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEventSink_Impl::OnSetFocus(this, core::mem::transmute_copy(&pdimfocus), core::mem::transmute_copy(&pdimprevfocus)).into()
            }
        }
        unsafe extern "system" fn OnPushContext<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEventSink_Impl::OnPushContext(this, core::mem::transmute_copy(&pic)).into()
            }
        }
        unsafe extern "system" fn OnPopContext<Identity: ITfThreadMgrEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEventSink_Impl::OnPopContext(this, core::mem::transmute_copy(&pic)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInitDocumentMgr: OnInitDocumentMgr::<Identity, OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Identity, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, OFFSET>,
            OnPushContext: OnPushContext::<Identity, OFFSET>,
            OnPopContext: OnPopContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfThreadMgrEventSink {}
windows_core::imp::define_interface!(ITfThreadMgrEx, ITfThreadMgrEx_Vtbl, 0x3e90ade3_7594_4cb0_bb58_69628f5f458c);
impl core::ops::Deref for ITfThreadMgrEx {
    type Target = ITfThreadMgr;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfThreadMgrEx, windows_core::IUnknown, ITfThreadMgr);
impl ITfThreadMgrEx {
    pub unsafe fn ActivateEx(&self, ptid: *mut TfClientId, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateEx)(windows_core::Interface::as_raw(self), ptid as _, dwflags) }
    }
    pub unsafe fn GetActiveFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEx_Vtbl {
    pub base__: ITfThreadMgr_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TfClientId, u32) -> windows_core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait ITfThreadMgrEx_Impl: ITfThreadMgr_Impl {
    fn ActivateEx(&self, ptid: *mut TfClientId, dwflags: u32) -> windows_core::Result<()>;
    fn GetActiveFlags(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "windef")]
impl ITfThreadMgrEx_Vtbl {
    pub const fn new<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateEx<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptid: *mut TfClientId, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfThreadMgrEx_Impl::ActivateEx(this, core::mem::transmute_copy(&ptid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ITfThreadMgrEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfThreadMgrEx_Impl::GetActiveFlags(this) {
                    Ok(ok__) => {
                        lpdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITfThreadMgr_Vtbl::new::<Identity, OFFSET>(),
            ActivateEx: ActivateEx::<Identity, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfThreadMgrEx as windows_core::Interface>::IID || iid == &<ITfThreadMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ITfThreadMgrEx {}
windows_core::imp::define_interface!(ITfToolTipUIElement, ITfToolTipUIElement_Vtbl, 0x52b18b5c_555d_46b2_b00a_fa680144fbdb);
impl core::ops::Deref for ITfToolTipUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfToolTipUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfToolTipUIElement {
    pub unsafe fn GetString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfToolTipUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfToolTipUIElement_Impl: ITfUIElement_Impl {
    fn GetString(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITfToolTipUIElement_Vtbl {
    pub const fn new<Identity: ITfToolTipUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetString<Identity: ITfToolTipUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfToolTipUIElement_Impl::GetString(this) {
                    Ok(ok__) => {
                        pstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetString: GetString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfToolTipUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfToolTipUIElement {}
windows_core::imp::define_interface!(ITfTransitoryExtensionSink, ITfTransitoryExtensionSink_Vtbl, 0xa615096f_1c57_4813_8a15_55ee6e5a839c);
windows_core::imp::interface_hierarchy!(ITfTransitoryExtensionSink, windows_core::IUnknown);
impl ITfTransitoryExtensionSink {
    pub unsafe fn OnTransitoryExtensionUpdated<P0, P2, P3>(&self, pic: P0, ecreadonly: TfEditCookie, presultrange: P2, pcompositionrange: P3) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<ITfContext>,
        P2: windows_core::Param<ITfRange>,
        P3: windows_core::Param<ITfRange>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnTransitoryExtensionUpdated)(windows_core::Interface::as_raw(self), pic.param().abi(), ecreadonly, presultrange.param().abi(), pcompositionrange.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransitoryExtensionUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TfEditCookie, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITfTransitoryExtensionSink_Impl: windows_core::IUnknownImpl {
    fn OnTransitoryExtensionUpdated(&self, pic: windows_core::Ref<ITfContext>, ecreadonly: TfEditCookie, presultrange: windows_core::Ref<ITfRange>, pcompositionrange: windows_core::Ref<ITfRange>) -> windows_core::Result<windows_core::BOOL>;
}
impl ITfTransitoryExtensionSink_Vtbl {
    pub const fn new<Identity: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Identity: ITfTransitoryExtensionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pic: *mut core::ffi::c_void, ecreadonly: TfEditCookie, presultrange: *mut core::ffi::c_void, pcompositionrange: *mut core::ffi::c_void, pfdeleteresultrange: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfTransitoryExtensionSink_Impl::OnTransitoryExtensionUpdated(this, core::mem::transmute_copy(&pic), core::mem::transmute_copy(&ecreadonly), core::mem::transmute_copy(&presultrange), core::mem::transmute_copy(&pcompositionrange)) {
                    Ok(ok__) => {
                        pfdeleteresultrange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTransitoryExtensionSink {}
windows_core::imp::define_interface!(ITfTransitoryExtensionUIElement, ITfTransitoryExtensionUIElement_Vtbl, 0x858f956a_972f_42a2_a2f2_0321e1abe209);
impl core::ops::Deref for ITfTransitoryExtensionUIElement {
    type Target = ITfUIElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITfTransitoryExtensionUIElement, windows_core::IUnknown, ITfUIElement);
impl ITfTransitoryExtensionUIElement {
    pub unsafe fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentMgr)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetDocumentMgr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfTransitoryExtensionUIElement_Impl: ITfUIElement_Impl {
    fn GetDocumentMgr(&self) -> windows_core::Result<ITfDocumentMgr>;
}
impl ITfTransitoryExtensionUIElement_Vtbl {
    pub const fn new<Identity: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentMgr<Identity: ITfTransitoryExtensionUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdim: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfTransitoryExtensionUIElement_Impl::GetDocumentMgr(this) {
                    Ok(ok__) => {
                        ppdim.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ITfUIElement_Vtbl::new::<Identity, OFFSET>(), GetDocumentMgr: GetDocumentMgr::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfTransitoryExtensionUIElement as windows_core::Interface>::IID || iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfTransitoryExtensionUIElement {}
windows_core::imp::define_interface!(ITfUIElement, ITfUIElement_Vtbl, 0xea1ea137_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElement, windows_core::IUnknown);
impl ITfUIElement {
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Show(&self, bshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), bshow.into()) }
    }
    pub unsafe fn IsShown(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsShown)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsShown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITfUIElement_Impl: windows_core::IUnknownImpl {
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn Show(&self, bshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsShown(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ITfUIElement_Vtbl {
    pub const fn new<Identity: ITfUIElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDescription<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfUIElement_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGUID<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfUIElement_Impl::GetGUID(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Show<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElement_Impl::Show(this, core::mem::transmute_copy(&bshow)).into()
            }
        }
        unsafe extern "system" fn IsShown<Identity: ITfUIElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfUIElement_Impl::IsShown(this) {
                    Ok(ok__) => {
                        pbshow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            Show: Show::<Identity, OFFSET>,
            IsShown: IsShown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElement as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElement {}
windows_core::imp::define_interface!(ITfUIElementMgr, ITfUIElementMgr_Vtbl, 0xea1ea135_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElementMgr, windows_core::IUnknown);
impl ITfUIElementMgr {
    pub unsafe fn BeginUIElement<P0>(&self, pelement: P0, pbshow: *mut windows_core::BOOL, pdwuielementid: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITfUIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginUIElement)(windows_core::Interface::as_raw(self), pelement.param().abi(), pbshow as _, pdwuielementid as _) }
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateUIElement)(windows_core::Interface::as_raw(self), dwuielementid) }
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndUIElement)(windows_core::Interface::as_raw(self), dwuielementid) }
    }
    pub unsafe fn GetUIElement(&self, dwuielementid: u32) -> windows_core::Result<ITfUIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUIElement)(windows_core::Interface::as_raw(self), dwuielementid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumUIElements(&self) -> windows_core::Result<IEnumTfUIElements> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumUIElements)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub UpdateUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUIElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITfUIElementMgr_Impl: windows_core::IUnknownImpl {
    fn BeginUIElement(&self, pelement: windows_core::Ref<ITfUIElement>, pbshow: *mut windows_core::BOOL, pdwuielementid: *mut u32) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn GetUIElement(&self, dwuielementid: u32) -> windows_core::Result<ITfUIElement>;
    fn EnumUIElements(&self) -> windows_core::Result<IEnumTfUIElements>;
}
impl ITfUIElementMgr_Vtbl {
    pub const fn new<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, pbshow: *mut windows_core::BOOL, pdwuielementid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementMgr_Impl::BeginUIElement(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&pbshow), core::mem::transmute_copy(&pdwuielementid)).into()
            }
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementMgr_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
            }
        }
        unsafe extern "system" fn EndUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementMgr_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
            }
        }
        unsafe extern "system" fn GetUIElement<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfUIElementMgr_Impl::GetUIElement(this, core::mem::transmute_copy(&dwuielementid)) {
                    Ok(ok__) => {
                        ppelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumUIElements<Identity: ITfUIElementMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITfUIElementMgr_Impl::EnumUIElements(this) {
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
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
            GetUIElement: GetUIElement::<Identity, OFFSET>,
            EnumUIElements: EnumUIElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElementMgr {}
windows_core::imp::define_interface!(ITfUIElementSink, ITfUIElementSink_Vtbl, 0xea1ea136_19df_11d7_a6d2_00065b84435c);
windows_core::imp::interface_hierarchy!(ITfUIElementSink, windows_core::IUnknown);
impl ITfUIElementSink {
    pub unsafe fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginUIElement)(windows_core::Interface::as_raw(self), dwuielementid, pbshow as _) }
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateUIElement)(windows_core::Interface::as_raw(self), dwuielementid) }
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndUIElement)(windows_core::Interface::as_raw(self), dwuielementid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub UpdateUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITfUIElementSink_Impl: windows_core::IUnknownImpl {
    fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn UpdateUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
    fn EndUIElement(&self, dwuielementid: u32) -> windows_core::Result<()>;
}
impl ITfUIElementSink_Vtbl {
    pub const fn new<Identity: ITfUIElementSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32, pbshow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementSink_Impl::BeginUIElement(this, core::mem::transmute_copy(&dwuielementid), core::mem::transmute_copy(&pbshow)).into()
            }
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementSink_Impl::UpdateUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
            }
        }
        unsafe extern "system" fn EndUIElement<Identity: ITfUIElementSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuielementid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITfUIElementSink_Impl::EndUIElement(this, core::mem::transmute_copy(&dwuielementid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginUIElement: BeginUIElement::<Identity, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, OFFSET>,
            EndUIElement: EndUIElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITfUIElementSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITfUIElementSink {}
pub const TF_AE_END: TfActiveSelEnd = 2;
pub const TF_AE_NONE: TfActiveSelEnd = 0;
pub const TF_AE_START: TfActiveSelEnd = 1;
pub const TF_ANCHOR_END: TfAnchor = 1;
pub const TF_ANCHOR_START: TfAnchor = 0;
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = 2;
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = 5;
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = 0;
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = 4;
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = -1;
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = 1;
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = 3;
pub const TF_CHAR_EMBEDDED: u32 = 65532;
pub const TF_CLIENTID_NULL: TfClientId = TfClientId(0);
pub const TF_CLUIE_COUNT: u32 = 2;
pub const TF_CLUIE_CURRENTPAGE: u32 = 32;
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1;
pub const TF_CLUIE_PAGEINDEX: u32 = 16;
pub const TF_CLUIE_SELECTION: u32 = 4;
pub const TF_CLUIE_STRING: u32 = 8;
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0;
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32;
pub const TF_CONVERSIONMODE_EUDC: u32 = 512;
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048;
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8;
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2;
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1;
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256;
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16;
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128;
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024;
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = 2;
pub const TF_CT_NONE: TF_DA_COLORTYPE = 0;
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = 1;
pub type TF_DA_ATTR_INFO = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
#[cfg(feature = "windef")]
impl Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: super::windef::COLORREF,
}
#[cfg(feature = "windef")]
impl Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TF_DA_COLORTYPE = i32;
pub type TF_DA_LINESTYLE = i32;
pub const TF_DEFAULT_SELECTION: i32 = -1;
pub const TF_DISABLE_COMMANDING: u32 = 4;
pub const TF_DISABLE_DICTATION: u32 = 2;
pub const TF_DISABLE_SPEECH: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: windows_core::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "windef")]
impl Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_ES_ASYNC: u32 = 8;
pub const TF_ES_ASYNCDONTCARE: u32 = 0;
pub const TF_ES_READ: u32 = 2;
pub const TF_ES_READWRITE: u32 = 6;
pub const TF_ES_SYNC: u32 = 1;
pub const TF_E_ALREADY_EXISTS: i32 = -2147220218;
pub const TF_E_COMPOSITION_REJECTED: i32 = -2147220216;
pub const TF_E_DISCONNECTED: i32 = -2147220220;
pub const TF_E_EMPTYCONTEXT: i32 = -2147220215;
pub const TF_E_FORMAT: i32 = -2147220982;
pub const TF_E_INVALIDPOINT: i32 = -2147220985;
pub const TF_E_INVALIDPOS: i32 = -2147220992;
pub const TF_E_INVALIDVIEW: i32 = -2147220219;
pub const TF_E_LOCKED: i32 = -2147220224;
pub const TF_E_NOINTERFACE: i32 = -2147220988;
pub const TF_E_NOLAYOUT: i32 = -2147220986;
pub const TF_E_NOLOCK: i32 = -2147220991;
pub const TF_E_NOOBJECT: i32 = -2147220990;
pub const TF_E_NOPROVIDER: i32 = -2147220221;
pub const TF_E_NOSELECTION: i32 = -2147220987;
pub const TF_E_NOSERVICE: i32 = -2147220989;
pub const TF_E_NOTOWNEDRANGE: i32 = -2147220222;
pub const TF_E_RANGE_NOT_COVERED: i32 = -2147220217;
pub const TF_E_READONLY: i32 = -2147220983;
pub const TF_E_STACKFULL: i32 = -2147220223;
pub const TF_E_SYNCHRONOUS: i32 = -2147220984;
pub const TF_GRAVITY_BACKWARD: TfGravity = 0;
pub const TF_GRAVITY_FORWARD: TfGravity = 1;
pub const TF_GTP_INCL_TEXT: u32 = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TF_HALTCOND {
    pub pHaltRange: core::mem::ManuallyDrop<Option<ITfRange>>,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
pub const TF_HF_OBJECT: u32 = 1;
pub const TF_IAS_NOQUERY: u32 = 1;
pub const TF_IAS_NO_DEFAULT_COMPOSITION: u32 = 2147483648;
pub const TF_IAS_QUERYONLY: u32 = 2;
pub const TF_IE_CORRECTION: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: super::winnt::LANGID,
    pub clsid: windows_core::GUID,
    pub guidProfile: windows_core::GUID,
    pub catid: windows_core::GUID,
    pub hklSubstitute: super::minwindef::HKL,
    pub dwCaps: u32,
    pub hkl: super::minwindef::HKL,
    pub dwFlags: u32,
}
pub const TF_INVALID_COOKIE: u32 = 4294967295;
pub const TF_INVALID_EDIT_COOKIE: u32 = 0;
pub const TF_INVALID_GUIDATOM: TfGuidAtom = TfGuidAtom(0);
pub const TF_INVALID_UIELEMENTID: i32 = -1;
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2;
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4;
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1;
pub const TF_IPPMF_FORPROCESS: u32 = 268435456;
pub const TF_IPPMF_FORSESSION: u32 = 536870912;
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824;
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8;
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1;
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536;
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2;
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072;
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4;
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16;
pub const TF_IPP_FLAG_ACTIVE: u32 = 1;
pub const TF_IPP_FLAG_ENABLED: u32 = 2;
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4;
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: windows_core::GUID,
    pub langid: super::winnt::LANGID,
    pub catid: windows_core::GUID,
    pub fActive: windows_core::BOOL,
    pub guidProfile: windows_core::GUID,
}
pub const TF_LC_CHANGE: TfLayoutCode = 1;
pub const TF_LC_CREATE: TfLayoutCode = 0;
pub const TF_LC_DESTROY: TfLayoutCode = 2;
pub const TF_LS_DASH: TF_DA_LINESTYLE = 3;
pub const TF_LS_DOT: TF_DA_LINESTYLE = 2;
pub const TF_LS_NONE: TF_DA_LINESTYLE = 0;
pub const TF_LS_SOLID: TF_DA_LINESTYLE = 1;
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = 4;
pub const TF_MOD_ALT: u32 = 1;
pub const TF_MOD_CONTROL: u32 = 2;
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024;
pub const TF_MOD_LALT: u32 = 64;
pub const TF_MOD_LCONTROL: u32 = 128;
pub const TF_MOD_LSHIFT: u32 = 256;
pub const TF_MOD_ON_KEYUP: u32 = 512;
pub const TF_MOD_RALT: u32 = 8;
pub const TF_MOD_RCONTROL: u32 = 16;
pub const TF_MOD_RSHIFT: u32 = 32;
pub const TF_MOD_SHIFT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: windows_core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: windows_core::GUID,
}
pub const TF_POPF_ALL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1;
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2;
pub const TF_PROFILE_ARRAY: windows_core::GUID = windows_core::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
pub const TF_PROFILE_CANTONESE: windows_core::GUID = windows_core::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
pub const TF_PROFILE_CHANGJIE: windows_core::GUID = windows_core::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
pub const TF_PROFILE_DAYI: windows_core::GUID = windows_core::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
pub const TF_PROFILE_NEWCHANGJIE: windows_core::GUID = windows_core::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_NEWPHONETIC: windows_core::GUID = windows_core::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
pub const TF_PROFILE_NEWQUICK: windows_core::GUID = windows_core::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
pub const TF_PROFILE_PHONETIC: windows_core::GUID = windows_core::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
pub const TF_PROFILE_PINYIN: windows_core::GUID = windows_core::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_QUICK: windows_core::GUID = windows_core::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
pub const TF_PROFILE_SIMPLEFAST: windows_core::GUID = windows_core::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
pub const TF_PROFILE_TIGRINYA: windows_core::GUID = windows_core::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
pub const TF_PROFILE_WUBI: windows_core::GUID = windows_core::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
pub const TF_PROFILE_YI: windows_core::GUID = windows_core::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub struct TF_PROPERTYVAL {
    pub guidId: windows_core::GUID,
    pub varValue: super::oaidl::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TF_RCM_COMLESS: u32 = 1;
pub const TF_RCM_HINT_COLLISION: u32 = 8;
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4;
pub const TF_RCM_VKEY: u32 = 2;
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1;
pub const TF_RIUIE_CONTEXT: u32 = 1;
pub const TF_RIUIE_ERRORINDEX: u32 = 8;
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4;
pub const TF_RIUIE_STRING: u32 = 2;
pub const TF_RIUIE_VERTICALORDER: u32 = 16;
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2;
pub const TF_RP_LOCALPROCESS: u32 = 4;
pub const TF_RP_LOCALTHREAD: u32 = 8;
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16;
pub const TF_SD_BACKWARD: TfShiftDir = 0;
pub const TF_SD_FORWARD: TfShiftDir = 1;
pub const TF_SD_LOADING: u32 = 2;
pub const TF_SD_READONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TF_SELECTION {
    pub range: core::mem::ManuallyDrop<Option<ITfRange>>,
    pub style: TF_SELECTIONSTYLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: windows_core::BOOL,
}
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4;
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16;
pub const TF_SENTENCEMODE_NONE: u32 = 0;
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8;
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1;
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2;
pub const TF_SS_DISJOINTSEL: u32 = 1;
pub const TF_SS_REGIONS: u32 = 2;
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16;
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32;
pub const TF_SS_TRANSITORY: u32 = 4;
#[cfg(feature = "textstor")]
pub type TF_STATUS = super::textstor::TS_STATUS;
pub const TF_ST_CORRECTION: u32 = 1;
pub const TF_S_ASYNC: u32 = 262912;
pub const TF_TF_IGNOREEND: u32 = 2;
pub const TF_TF_MOVESTART: u32 = 1;
pub const TF_TMAE_COMLESS: u32 = 8;
pub const TF_TMAE_CONSOLE: u32 = 64;
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32;
pub const TF_TMAE_NOACTIVATETIP: u32 = 1;
pub const TF_TMAE_SECUREMODE: u32 = 2;
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4;
pub const TF_TMAE_WOW16: u32 = 16;
pub const TF_TMF_ACTIVATED: u32 = 2147483648;
pub const TF_TMF_COMLESS: u32 = 8;
pub const TF_TMF_CONSOLE: u32 = 64;
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824;
pub const TF_TMF_NOACTIVATETIP: u32 = 1;
pub const TF_TMF_SECUREMODE: u32 = 2;
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4;
pub const TF_TMF_WOW16: u32 = 16;
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2;
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1;
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0;
pub const TF_TU_CORRECTION: u32 = 1;
pub const TF_URP_ALLPROFILES: u32 = 2;
pub const TF_URP_LOCALPROCESS: u32 = 4;
pub const TF_URP_LOCALTHREAD: u32 = 8;
pub const TF_US_HIDETIPUI: u32 = 1;
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4;
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2;
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3;
pub const TKB_ALTERNATES_STANDARD: u32 = 1;
pub type TfActiveSelEnd = i32;
pub type TfAnchor = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TfClientId(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TfEditCookie(pub u32);
pub type TfGravity = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TfGuidAtom(pub u32);
pub type TfLayoutCode = i32;
pub type TfShiftDir = i32;
