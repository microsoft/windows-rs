windows_core::imp::define_interface!(IEnumWIA_DEV_CAPS, IEnumWIA_DEV_CAPS_Vtbl, 0x1fcc4287_aca6_11d2_a093_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_CAPS, windows_core::IUnknown);
impl IEnumWIA_DEV_CAPS {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::HRESULT {
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
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WIA_DEV_CAP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_DEV_CAPS_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_DEV_CAPS_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_CAPS_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_CAPS_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(ok__);
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_CAPS as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_DEV_CAPS {}
windows_core::imp::define_interface!(IEnumWIA_DEV_INFO, IEnumWIA_DEV_INFO_Vtbl, 0x5e38b83c_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_INFO, windows_core::IUnknown);
impl IEnumWIA_DEV_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> windows_core::HRESULT {
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
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_DEV_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IWiaPropertyStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_DEV_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_INFO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_INFO_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(ok__);
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_INFO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_DEV_INFO {}
windows_core::imp::define_interface!(IEnumWIA_FORMAT_INFO, IEnumWIA_FORMAT_INFO_Vtbl, 0x81befc5b_656d_44f1_b24c_d41d51b4dc81);
windows_core::imp::interface_hierarchy!(IEnumWIA_FORMAT_INFO, windows_core::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
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
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WIA_FORMAT_INFO, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_FORMAT_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_FORMAT_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_FORMAT_INFO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_FORMAT_INFO_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(ok__);
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_FORMAT_INFO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_FORMAT_INFO {}
windows_core::imp::define_interface!(IEnumWiaItem, IEnumWiaItem_Vtbl, 0x5e8383fc_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IEnumWiaItem, windows_core::IUnknown);
impl IEnumWiaItem {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem: *mut Option<IWiaItem>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem), pceltfetched as _) }
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
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWiaItem_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppiwiaitem: windows_core::OutRef<IWiaItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWiaItem_Vtbl {
    pub const fn new<Identity: IEnumWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(ok__);
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWiaItem {}
windows_core::imp::define_interface!(IEnumWiaItem2, IEnumWiaItem2_Vtbl, 0x59970af4_cd0d_44d9_ab24_52295630e582);
windows_core::imp::interface_hierarchy!(IEnumWiaItem2, windows_core::IUnknown);
impl IEnumWiaItem2 {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem2: *mut Option<IWiaItem2>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem2), pceltfetched as _) }
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
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWiaItem2_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppiwiaitem2: windows_core::OutRef<IWiaItem2>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem2>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWiaItem2_Vtbl {
    pub const fn new<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem2: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem2), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem2_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(ok__);
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWiaItem2 {}
windows_core::imp::define_interface!(IWiaAppErrorHandler, IWiaAppErrorHandler_Vtbl, 0x6c16186c_d0a6_400c_80f4_d26986a0e734);
windows_core::imp::interface_hierarchy!(IWiaAppErrorHandler, windows_core::IUnknown);
impl IWiaAppErrorHandler {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportStatus<P1>(&self, lflags: i32, pwiaitem2: P1, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportStatus)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), hrstatus, lpercentcomplete) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaAppErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetWindow: usize,
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaAppErrorHandler_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn ReportStatus(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaAppErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaAppErrorHandler_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportStatus<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaAppErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            ReportStatus: ReportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaAppErrorHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaAppErrorHandler {}
windows_core::imp::define_interface!(IWiaDataCallback, IWiaDataCallback_Vtbl, 0xa558a866_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataCallback, windows_core::IUnknown);
impl IWiaDataCallback {
    pub unsafe fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BandedDataCallback)(windows_core::Interface::as_raw(self), lmessage, lstatus, lpercentcomplete, loffset, llength, lreserved, lreslength, pbbuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BandedDataCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, *const u8) -> windows_core::HRESULT,
}
pub trait IWiaDataCallback_Impl: windows_core::IUnknownImpl {
    fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *const u8) -> windows_core::Result<()>;
}
impl IWiaDataCallback_Vtbl {
    pub const fn new<Identity: IWiaDataCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BandedDataCallback<Identity: IWiaDataCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataCallback_Impl::BandedDataCallback(this, core::mem::transmute_copy(&lmessage), core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lpercentcomplete), core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&llength), core::mem::transmute_copy(&lreserved), core::mem::transmute_copy(&lreslength), core::mem::transmute_copy(&pbbuffer)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BandedDataCallback: BandedDataCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaDataCallback {}
windows_core::imp::define_interface!(IWiaDataTransfer, IWiaDataTransfer_Vtbl, 0xa6cef998_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataTransfer, windows_core::IUnknown);
impl IWiaDataTransfer {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
    pub unsafe fn idtGetData<P1>(&self, pmedium: *mut super::objidl::STGMEDIUM, piwiadatacallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pmedium), piwiadatacallback.param().abi()) }
    }
    pub unsafe fn idtGetBandedData<P1>(&self, pwiadatatransinfo: *const WIA_DATA_TRANSFER_INFO, piwiadatacallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetBandedData)(windows_core::Interface::as_raw(self), pwiadatatransinfo, piwiadatacallback.param().abi()) }
    }
    pub unsafe fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).idtQueryGetData)(windows_core::Interface::as_raw(self), pfe) }
    }
    pub unsafe fn idtEnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).idtEnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).idtGetExtendedTransferInfo)(windows_core::Interface::as_raw(self), pextendedtransferinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
    pub idtGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::objidl::STGMEDIUM, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes")))]
    idtGetData: usize,
    pub idtGetBandedData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WIA_DATA_TRANSFER_INFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub idtQueryGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WIA_FORMAT_INFO) -> windows_core::HRESULT,
    pub idtEnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub idtGetExtendedTransferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
pub trait IWiaDataTransfer_Impl: windows_core::IUnknownImpl {
    fn idtGetData(&self, pmedium: *mut super::objidl::STGMEDIUM, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtGetBandedData(&self, pwiadatatransinfo: *const WIA_DATA_TRANSFER_INFO, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> windows_core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl IWiaDataTransfer_Vtbl {
    pub const fn new<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn idtGetData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedium: *mut super::objidl::STGMEDIUM, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetData(this, core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&piwiadatacallback)).into()
            }
        }
        unsafe extern "system" fn idtGetBandedData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiadatatransinfo: *const WIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetBandedData(this, core::mem::transmute_copy(&pwiadatatransinfo), core::mem::transmute_copy(&piwiadatacallback)).into()
            }
        }
        unsafe extern "system" fn idtQueryGetData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtQueryGetData(this, core::mem::transmute_copy(&pfe)).into()
            }
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDataTransfer_Impl::idtEnumWIA_FORMAT_INFO(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetExtendedTransferInfo(this, core::mem::transmute_copy(&pextendedtransferinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            idtGetData: idtGetData::<Identity, OFFSET>,
            idtGetBandedData: idtGetBandedData::<Identity, OFFSET>,
            idtQueryGetData: idtQueryGetData::<Identity, OFFSET>,
            idtEnumWIA_FORMAT_INFO: idtEnumWIA_FORMAT_INFO::<Identity, OFFSET>,
            idtGetExtendedTransferInfo: idtGetExtendedTransferInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataTransfer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl windows_core::RuntimeName for IWiaDataTransfer {}
windows_core::imp::define_interface!(IWiaDevMgr, IWiaDevMgr_Vtbl, 0x5eb2502a_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IWiaDevMgr, windows_core::IUnknown);
impl IWiaDevMgr {
    pub unsafe fn EnumDeviceInfo(&self, lflag: i32) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflag, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetImageDlg<P4>(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: P4, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IWiaItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, lintent, pitemroot.param().abi(), core::mem::transmute_copy(bstrfilename), pguidformat as _) }
    }
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrcommandline), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3) -> windows_core::Result<windows_core::IUnknown>
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn AddDeviceDlg(&self, hwndparent: super::windef::HWND, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SelectDeviceDlg: usize,
    #[cfg(feature = "Win32_windef")]
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SelectDeviceDlgID: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetImageDlg: usize,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub AddDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    AddDeviceDlg: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaDevMgr_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflag: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn SelectDeviceDlg(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn SelectDeviceDlgID(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetImageDlg(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: windows_core::Ref<IWiaItem>, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddDeviceDlg(&self, hwndparent: super::windef::HWND, lflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaDevMgr_Vtbl {
    pub const fn new<Identity: IWiaDevMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflag: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflag)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, ppwiaitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::CreateDevice(this, core::mem::transmute(&bstrdeviceid)) {
                    Ok(ok__) => {
                        ppwiaitemroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)) {
                    Ok(ok__) => {
                        ppitemroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pbstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::GetImageDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&pitemroot), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pguidformat)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrcommandline: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrcommandline), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback)) {
                    Ok(ok__) => {
                        peventobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::AddDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            AddDeviceDlg: AddDeviceDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaDevMgr {}
windows_core::imp::define_interface!(IWiaDevMgr2, IWiaDevMgr2_Vtbl, 0x79c07cf1_cbdd_41ee_8ec3_f00080cada7a);
windows_core::imp::interface_hierarchy!(IWiaDevMgr2, windows_core::IUnknown);
impl IWiaDevMgr2 {
    pub unsafe fn EnumDeviceInfo(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDevice(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3) -> windows_core::Result<windows_core::IUnknown>
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrfullappname), core::mem::transmute_copy(bstrcommandlinearg), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, hwndparent: super::windef::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut Option<IWiaItem2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, core::mem::transmute(ppitem)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SelectDeviceDlg: usize,
    #[cfg(feature = "Win32_windef")]
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SelectDeviceDlgID: usize,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetImageDlg: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaDevMgr2_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlg(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlgID(&self, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, hwndparent: super::windef::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaDevMgr2_Vtbl {
    pub const fn new<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, ppwiaitem2root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::CreateDevice(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid)) {
                    Ok(ok__) => {
                        ppwiaitem2root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)) {
                    Ok(ok__) => {
                        ppitemroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pbstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback)) {
                    Ok(ok__) => {
                        peventobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrfullappname: *mut core::ffi::c_void, bstrcommandlinearg: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrfullappname), core::mem::transmute(&bstrcommandlinearg), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, hwndparent: super::windef::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::GetImageDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaDevMgr2 {}
windows_core::imp::define_interface!(IWiaErrorHandler, IWiaErrorHandler_Vtbl, 0x0e4a51b1_bc1f_443d_a835_72e890759ef3);
windows_core::imp::interface_hierarchy!(IWiaErrorHandler, windows_core::IUnknown);
impl IWiaErrorHandler {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ReportStatus<P2>(&self, lflags: i32, hwndparent: super::windef::HWND, pwiaitem2: P2, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportStatus)(windows_core::Interface::as_raw(self), lflags, hwndparent, pwiaitem2.param().abi(), hrstatus, lpercentcomplete) }
    }
    pub unsafe fn GetStatusDescription<P1>(&self, lflags: i32, pwiaitem2: P1, hrstatus: windows_core::HRESULT) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<IWiaItem2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatusDescription)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), hrstatus, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::HWND, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    ReportStatus: usize,
    pub GetStatusDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaErrorHandler_Impl: windows_core::IUnknownImpl {
    fn ReportStatus(&self, lflags: i32, hwndparent: super::windef::HWND, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
    fn GetStatusDescription(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportStatus<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::windef::HWND, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
            }
        }
        unsafe extern "system" fn GetStatusDescription<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaErrorHandler_Impl::GetStatusDescription(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus)) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportStatus: ReportStatus::<Identity, OFFSET>,
            GetStatusDescription: GetStatusDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaErrorHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaErrorHandler {}
windows_core::imp::define_interface!(IWiaEventCallback, IWiaEventCallback_Vtbl, 0xae6287b0_0084_11d2_973b_00a0c9068f2e);
windows_core::imp::interface_hierarchy!(IWiaEventCallback, windows_core::IUnknown);
impl IWiaEventCallback {
    pub unsafe fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImageEventCallback)(windows_core::Interface::as_raw(self), peventguid, core::mem::transmute_copy(bstreventdescription), core::mem::transmute_copy(bstrdeviceid), core::mem::transmute_copy(bstrdevicedescription), dwdevicetype, core::mem::transmute_copy(bstrfullitemname), puleventtype as _, ulreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImageEventCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IWiaEventCallback_Impl: windows_core::IUnknownImpl {
    fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> windows_core::Result<()>;
}
impl IWiaEventCallback_Vtbl {
    pub const fn new<Identity: IWiaEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageEventCallback<Identity: IWiaEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstreventdescription: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, bstrdevicedescription: *mut core::ffi::c_void, dwdevicetype: u32, bstrfullitemname: *mut core::ffi::c_void, puleventtype: *mut u32, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaEventCallback_Impl::ImageEventCallback(this, core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstreventdescription), core::mem::transmute(&bstrdeviceid), core::mem::transmute(&bstrdevicedescription), core::mem::transmute_copy(&dwdevicetype), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&puleventtype), core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ImageEventCallback: ImageEventCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaEventCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaEventCallback {}
windows_core::imp::define_interface!(IWiaImageFilter, IWiaImageFilter_Vtbl, 0xa8a79ffa_450b_41f1_8f87_849ccd94ebf6);
windows_core::imp::interface_hierarchy!(IWiaImageFilter, windows_core::IUnknown);
impl IWiaImageFilter {
    pub unsafe fn InitializeFilter<P0, P1>(&self, pwiaitem2: P0, pwiatransfercallback: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWiaItem2>,
        P1: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFilter)(windows_core::Interface::as_raw(self), pwiaitem2.param().abi(), pwiatransfercallback.param().abi()) }
    }
    pub unsafe fn SetNewCallback<P0>(&self, pwiatransfercallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNewCallback)(windows_core::Interface::as_raw(self), pwiatransfercallback.param().abi()) }
    }
    #[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef"))]
    pub unsafe fn FilterPreviewImage<P1, P3>(&self, lflags: i32, pwiachilditem2: P1, inputimageextents: super::windef::RECT, pinputstream: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P3: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).FilterPreviewImage)(windows_core::Interface::as_raw(self), lflags, pwiachilditem2.param().abi(), core::mem::transmute(inputimageextents), pinputstream.param().abi()) }
    }
    pub unsafe fn ApplyProperties<P0>(&self, pwiapropertystorage: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWiaPropertyStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).ApplyProperties)(windows_core::Interface::as_raw(self), pwiapropertystorage.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaImageFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNewCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef"))]
    pub FilterPreviewImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::windef::RECT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_objidlbase", feature = "Win32_windef")))]
    FilterPreviewImage: usize,
    pub ApplyProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef"))]
pub trait IWiaImageFilter_Impl: windows_core::IUnknownImpl {
    fn InitializeFilter(&self, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn SetNewCallback(&self, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn FilterPreviewImage(&self, lflags: i32, pwiachilditem2: windows_core::Ref<IWiaItem2>, inputimageextents: &super::windef::RECT, pinputstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn ApplyProperties(&self, pwiapropertystorage: windows_core::Ref<IWiaPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef"))]
impl IWiaImageFilter_Vtbl {
    pub const fn new<Identity: IWiaImageFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFilter<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::InitializeFilter(this, core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn SetNewCallback<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::SetNewCallback(this, core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn FilterPreviewImage<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiachilditem2: *mut core::ffi::c_void, inputimageextents: super::windef::RECT, pinputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::FilterPreviewImage(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiachilditem2), core::mem::transmute(&inputimageextents), core::mem::transmute_copy(&pinputstream)).into()
            }
        }
        unsafe extern "system" fn ApplyProperties<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiapropertystorage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::ApplyProperties(this, core::mem::transmute_copy(&pwiapropertystorage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, OFFSET>,
            SetNewCallback: SetNewCallback::<Identity, OFFSET>,
            FilterPreviewImage: FilterPreviewImage::<Identity, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaImageFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IWiaImageFilter {}
windows_core::imp::define_interface!(IWiaItem, IWiaItem_Vtbl, 0x4db1ad10_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IWiaItem, windows_core::IUnknown);
impl IWiaItem {
    pub unsafe fn GetItemType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AnalyzeItem(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn EnumChildItems(&self) -> windows_core::Result<IEnumWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn CreateChildItem(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DeviceDlg(&self, hwndparent: super::windef::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags, lintent, plitemcount as _, ppiwiaitem as _) }
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(piwiaitem)) }
    }
    pub unsafe fn GetRootItem(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DumpDrvItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpDrvItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DumpTreeItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpTreeItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), ulsize, pbuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AnalyzeItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32, *mut i32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    DeviceDlg: usize,
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpDrvItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpTreeItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaItem_Impl: windows_core::IUnknownImpl {
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn AnalyzeItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self) -> windows_core::Result<IEnumWiaItem>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn CreateChildItem(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn DeviceDlg(&self, hwndparent: super::windef::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpDrvItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpTreeItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaItem_Vtbl {
    pub const fn new<Identity: IWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemType<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::GetItemType(this) {
                    Ok(ok__) => {
                        pitemtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnalyzeItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::AnalyzeItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumChildItems(this) {
                    Ok(ok__) => {
                        ppienumwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::CreateChildItem(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::windef::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&plitemcount), core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&piwiaitem)).into()
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::GetRootItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpDrvItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpTreeItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, OFFSET>,
            AnalyzeItem: AnalyzeItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            DumpItemData: DumpItemData::<Identity, OFFSET>,
            DumpDrvItemData: DumpDrvItemData::<Identity, OFFSET>,
            DumpTreeItemData: DumpTreeItemData::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaItem {}
windows_core::imp::define_interface!(IWiaItem2, IWiaItem2_Vtbl, 0x6cba0075_1287_407d_9b77_cf0e030435cc);
windows_core::imp::interface_hierarchy!(IWiaItem2, windows_core::IUnknown);
impl IWiaItem2 {
    pub unsafe fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), litemflags, lcreationflags, core::mem::transmute_copy(bstritemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn EnumChildItems(&self, pcategoryguid: *const windows_core::GUID) -> windows_core::Result<IEnumWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), pcategoryguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetItemCategory(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DeviceDlg(&self, lflags: i32, hwndparent: super::windef::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: Option<*mut Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), lflags, hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, ppitem.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(ppiwiaitem2)) }
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CheckExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), riidextensioninterface, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetExtension<T>(&self, lflags: i32, bstrname: &windows_core::BSTR) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetParentItem(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootItem(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviewComponent(&self, lflags: i32) -> windows_core::Result<IWiaPreview> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviewComponent)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), ulsize, pbuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    DeviceDlg: usize,
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviewComponent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IWiaItem2_Impl: windows_core::IUnknownImpl {
    fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self, pcategoryguid: *const windows_core::GUID) -> windows_core::Result<IEnumWiaItem2>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn GetItemCategory(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn DeviceDlg(&self, lflags: i32, hwndparent: super::windef::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn CheckExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL>;
    fn GetExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParentItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetPreviewComponent(&self, lflags: i32) -> windows_core::Result<IWiaPreview>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IWiaItem2_Vtbl {
    pub const fn new<Identity: IWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::CreateChildItem(this, core::mem::transmute_copy(&litemflags), core::mem::transmute_copy(&lcreationflags), core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryguid: *const windows_core::GUID, ppienumwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumChildItems(this, core::mem::transmute_copy(&pcategoryguid)) {
                    Ok(ok__) => {
                        ppienumwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemCategory<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetItemCategory(this) {
                    Ok(ok__) => {
                        pitemcategoryguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetItemType(this) {
                    Ok(ok__) => {
                        pitemtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::windef::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::CheckExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface)) {
                    Ok(ok__) => {
                        pbextensionexists.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&ppout)).into()
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetParentItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetRootItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppwiapreview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetPreviewComponent(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppwiapreview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            GetItemCategory: GetItemCategory::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            CheckExtension: CheckExtension::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            GetPreviewComponent: GetPreviewComponent::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IWiaItem2 {}
windows_core::imp::define_interface!(IWiaItemExtras, IWiaItemExtras_Vtbl, 0x6291ef2c_36ef_4532_876a_8e132593778d);
windows_core::imp::interface_hierarchy!(IWiaItemExtras, windows_core::IUnknown);
impl IWiaItemExtras {
    pub unsafe fn GetExtendedErrorInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Escape(&self, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), dwescapecode, lpindata, cbindatasize, poutdata as _, dwoutdatasize, pdwactualdatasize as _) }
    }
    pub unsafe fn CancelPendingIO(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelPendingIO)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItemExtras_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetExtendedErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub CancelPendingIO: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaItemExtras_Impl: windows_core::IUnknownImpl {
    fn GetExtendedErrorInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Escape(&self, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::Result<()>;
    fn CancelPendingIO(&self) -> windows_core::Result<()>;
}
impl IWiaItemExtras_Vtbl {
    pub const fn new<Identity: IWiaItemExtras_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrerrortext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItemExtras_Impl::GetExtendedErrorInfo(this) {
                    Ok(ok__) => {
                        bstrerrortext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Escape<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItemExtras_Impl::Escape(this, core::mem::transmute_copy(&dwescapecode), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdatasize)).into()
            }
        }
        unsafe extern "system" fn CancelPendingIO<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItemExtras_Impl::CancelPendingIO(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetExtendedErrorInfo: GetExtendedErrorInfo::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            CancelPendingIO: CancelPendingIO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItemExtras as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaItemExtras {}
windows_core::imp::define_interface!(IWiaLog, IWiaLog_Vtbl, 0xa00c10b6_82a1_452f_8b6c_86062aad6890);
windows_core::imp::interface_hierarchy!(IWiaLog, windows_core::IUnknown);
impl IWiaLog {
    pub unsafe fn InitializeLog(&self, hinstance: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeLog)(windows_core::Interface::as_raw(self), hinstance) }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult) }
    }
    pub unsafe fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeLog: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaLog_Impl: windows_core::IUnknownImpl {
    fn InitializeLog(&self, hinstance: i32) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWiaLog_Vtbl {
    pub const fn new<Identity: IWiaLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLog<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::InitializeLog(this, core::mem::transmute_copy(&hinstance)).into()
            }
        }
        unsafe extern "system" fn hResult<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn Log<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLog: InitializeLog::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaLog {}
windows_core::imp::define_interface!(IWiaLogEx, IWiaLogEx_Vtbl, 0xaf1f22ac_7a40_4787_b421_aeb47a1fbd0b);
windows_core::imp::interface_hierarchy!(IWiaLogEx, windows_core::IUnknown);
impl IWiaLogEx {
    pub unsafe fn InitializeLogEx(&self, hinstance: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeLogEx)(windows_core::Interface::as_raw(self), hinstance) }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult) }
    }
    pub unsafe fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
    pub unsafe fn hResultEx(&self, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResultEx)(windows_core::Interface::as_raw(self), lmethodid, hresult) }
    }
    pub unsafe fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LogEx)(windows_core::Interface::as_raw(self), lmethodid, lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeLogEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub hResultEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub LogEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaLogEx_Impl: windows_core::IUnknownImpl {
    fn InitializeLogEx(&self, hinstance: *const u8) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn hResultEx(&self, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWiaLogEx_Vtbl {
    pub const fn new<Identity: IWiaLogEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::InitializeLogEx(this, core::mem::transmute_copy(&hinstance)).into()
            }
        }
        unsafe extern "system" fn hResult<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn Log<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        unsafe extern "system" fn hResultEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::hResultEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn LogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::LogEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLogEx: InitializeLogEx::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
            hResultEx: hResultEx::<Identity, OFFSET>,
            LogEx: LogEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLogEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaLogEx {}
windows_core::imp::define_interface!(IWiaNotifyDevMgr, IWiaNotifyDevMgr_Vtbl, 0x70681ea0_e7bf_4291_9fb1_4e8813a3f78e);
windows_core::imp::interface_hierarchy!(IWiaNotifyDevMgr, windows_core::IUnknown);
impl IWiaNotifyDevMgr {
    pub unsafe fn NewDeviceArrival(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NewDeviceArrival)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaNotifyDevMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewDeviceArrival: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaNotifyDevMgr_Impl: windows_core::IUnknownImpl {
    fn NewDeviceArrival(&self) -> windows_core::Result<()>;
}
impl IWiaNotifyDevMgr_Vtbl {
    pub const fn new<Identity: IWiaNotifyDevMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewDeviceArrival<Identity: IWiaNotifyDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaNotifyDevMgr_Impl::NewDeviceArrival(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewDeviceArrival: NewDeviceArrival::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaNotifyDevMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaNotifyDevMgr {}
windows_core::imp::define_interface!(IWiaPreview, IWiaPreview_Vtbl, 0x95c2b4fd_33f2_4d86_ad40_9431f0df08f7);
windows_core::imp::interface_hierarchy!(IWiaPreview, windows_core::IUnknown);
impl IWiaPreview {
    pub unsafe fn GetNewPreview<P1, P2>(&self, lflags: i32, pwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNewPreview)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), pwiatransfercallback.param().abi()) }
    }
    pub unsafe fn UpdatePreview<P1, P2>(&self, lflags: i32, pchildwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdatePreview)(windows_core::Interface::as_raw(self), lflags, pchildwiaitem2.param().abi(), pwiatransfercallback.param().abi()) }
    }
    pub unsafe fn DetectRegions(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetectRegions)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPreview_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNewPreview: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdatePreview: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaPreview_Impl: windows_core::IUnknownImpl {
    fn GetNewPreview(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn UpdatePreview(&self, lflags: i32, pchildwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn DetectRegions(&self, lflags: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IWiaPreview_Vtbl {
    pub const fn new<Identity: IWiaPreview_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewPreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::GetNewPreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn UpdatePreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pchildwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::UpdatePreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pchildwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn DetectRegions<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewPreview: GetNewPreview::<Identity, OFFSET>,
            UpdatePreview: UpdatePreview::<Identity, OFFSET>,
            DetectRegions: DetectRegions::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPreview as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaPreview {}
windows_core::imp::define_interface!(IWiaPropertyStorage, IWiaPropertyStorage_Vtbl, 0x98b5e8a0_29cc_491a_aac0_e6db4fdcceb6);
windows_core::imp::interface_hierarchy!(IWiaPropertyStorage, windows_core::IUnknown);
impl IWiaPropertyStorage {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgpropvar: *const super::propidlbase::PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar), propidnamefirst) }
    }
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes"))]
    pub unsafe fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid) }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_propidlbase")]
    pub unsafe fn Enum(&self) -> windows_core::Result<super::propidlbase::IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn SetTimes(&self, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime) }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_propidlbase"))]
    pub unsafe fn Stat(&self, pstatpsstg: *mut super::propidlbase::STATPROPSETSTG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyAttributes)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgflags as _, core::mem::transmute(rgpropvar)) }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut Option<super::objidlbase::IStream>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid as _, core::mem::transmute(ppistream)) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn SetPropertyStream<P1>(&self, pcompatibilityid: *const windows_core::GUID, pistream: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid, pistream.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::propidlbase::PROPSPEC, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::propidlbase::PROPSPEC, *const super::propidlbase::PROPVARIANT, super::wtypes::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    WriteMultiple: usize,
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes"))]
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::propidlbase::PROPSPEC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_propidlbase", feature = "Win32_wtypes")))]
    DeleteMultiple: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    ReadPropertyNames: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    WritePropertyNames: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::wtypes::PROPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    DeletePropertyNames: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_propidlbase")]
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_propidlbase"))]
    Enum: usize,
    #[cfg(feature = "Win32_minwindef")]
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_propidlbase"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::propidlbase::STATPROPSETSTG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_propidlbase")))]
    Stat: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::propidlbase::PROPSPEC, *mut u32, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetPropertyAttributes: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub GetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    GetPropertyStream: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub SetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    SetPropertyStream: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWiaPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgpropvar: *const super::propidlbase::PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<windows_core::PWSTR>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<super::propidlbase::IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut super::propidlbase::STATPROPSETSTG) -> windows_core::Result<()>;
    fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: windows_core::OutRef<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn SetPropertyStream(&self, pcompatibilityid: *const windows_core::GUID, pistream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWiaPropertyStorage_Vtbl {
    pub const fn new<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgpropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)) {
                    Ok(ok__) => {
                        rgpropvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgpropvar: *const super::propidlbase::PROPVARIANT, propidnamefirst: super::wtypes::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)) {
                    Ok(ok__) => {
                        rglpwstrname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const super::wtypes::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::Enum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut super::propidlbase::STATPROPSETSTG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::propidlbase::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetPropertyAttributes(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgflags), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumprops: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pulnumprops.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyStream<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), core::mem::transmute_copy(&ppistream)).into()
            }
        }
        unsafe extern "system" fn SetPropertyStream<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *const windows_core::GUID, pistream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), core::mem::transmute_copy(&pistream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            SetTimes: SetTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetPropertyStream: GetPropertyStream::<Identity, OFFSET>,
            SetPropertyStream: SetPropertyStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPropertyStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWiaPropertyStorage {}
windows_core::imp::define_interface!(IWiaSegmentationFilter, IWiaSegmentationFilter_Vtbl, 0xec46a697_ac04_4447_8f65_ff63d5154b21);
windows_core::imp::interface_hierarchy!(IWiaSegmentationFilter, windows_core::IUnknown);
impl IWiaSegmentationFilter {
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn DetectRegions<P1, P2>(&self, lflags: i32, pinputstream: P1, pwiaitem2: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectRegions)(windows_core::Interface::as_raw(self), lflags, pinputstream.param().abi(), pwiaitem2.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_objidlbase")]
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    DetectRegions: usize,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IWiaSegmentationFilter_Impl: windows_core::IUnknownImpl {
    fn DetectRegions(&self, lflags: i32, pinputstream: windows_core::Ref<super::objidlbase::IStream>, pwiaitem2: windows_core::Ref<IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IWiaSegmentationFilter_Vtbl {
    pub const fn new<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectRegions<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pinputstream: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaSegmentationFilter_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pinputstream), core::mem::transmute_copy(&pwiaitem2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DetectRegions: DetectRegions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaSegmentationFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IWiaSegmentationFilter {}
windows_core::imp::define_interface!(IWiaTransfer, IWiaTransfer_Vtbl, 0xc39d6942_2f4e_4d04_92fe_4ef4d3a1de5a);
windows_core::imp::interface_hierarchy!(IWiaTransfer, windows_core::IUnknown);
impl IWiaTransfer {
    pub unsafe fn Download<P1>(&self, lflags: i32, piwiatransfercallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), lflags, piwiatransfercallback.param().abi()) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Upload<P1, P2>(&self, lflags: i32, psource: P1, piwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Upload)(windows_core::Interface::as_raw(self), lflags, psource.param().abi(), piwiatransfercallback.param().abi()) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Upload: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Upload: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IWiaTransfer_Impl: windows_core::IUnknownImpl {
    fn Download(&self, lflags: i32, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Upload(&self, lflags: i32, psource: windows_core::Ref<super::objidlbase::IStream>, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IWiaTransfer_Vtbl {
    pub const fn new<Identity: IWiaTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Download<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Download(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&piwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn Upload<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, psource: *mut core::ffi::c_void, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Upload(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&psource), core::mem::transmute_copy(&piwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaTransfer_Impl::EnumWIA_FORMAT_INFO(this) {
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
            Download: Download::<Identity, OFFSET>,
            Upload: Upload::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EnumWIA_FORMAT_INFO: EnumWIA_FORMAT_INFO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransfer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IWiaTransfer {}
windows_core::imp::define_interface!(IWiaTransferCallback, IWiaTransferCallback_Vtbl, 0x27d4eaaf_28a6_4ca5_9aab_e678168b9527);
windows_core::imp::interface_hierarchy!(IWiaTransferCallback, windows_core::IUnknown);
impl IWiaTransferCallback {
    pub unsafe fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransferCallback)(windows_core::Interface::as_raw(self), lflags, pwiatransferparams) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextStream)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const WiaTransferParams) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub GetNextStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    GetNextStream: usize,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IWiaTransferCallback_Impl: windows_core::IUnknownImpl {
    fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
    fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::objidlbase::IStream>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IWiaTransferCallback_Vtbl {
    pub const fn new<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferCallback<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransferCallback_Impl::TransferCallback(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
            }
        }
        unsafe extern "system" fn GetNextStream<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransferCallback: TransferCallback::<Identity, OFFSET>,
            GetNextStream: GetNextStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransferCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IWiaTransferCallback {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_DATA_CALLBACK_HEADER(pub *mut WIA_DATA_CALLBACK_HEADER);
impl PWIA_DATA_CALLBACK_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_DATA_CALLBACK_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_DATA_TRANSFER_INFO(pub *mut WIA_DATA_TRANSFER_INFO);
impl PWIA_DATA_TRANSFER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_DATA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_DEV_CAP(pub *mut WIA_DEV_CAP);
impl PWIA_DEV_CAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_DEV_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_DITHER_PATTERN_DATA(pub *mut WIA_DITHER_PATTERN_DATA);
impl PWIA_DITHER_PATTERN_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_EVENT_HANDLER(pub *mut WIA_DEV_CAP);
impl PWIA_EVENT_HANDLER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_EVENT_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_EXTENDED_TRANSFER_INFO(pub *mut WIA_EXTENDED_TRANSFER_INFO);
impl PWIA_EXTENDED_TRANSFER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_EXTENDED_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_FORMAT_INFO(pub *mut WIA_FORMAT_INFO);
impl PWIA_FORMAT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIA_PROPID_TO_NAME(pub *mut WIA_PROPID_TO_NAME);
#[cfg(feature = "Win32_wtypes")]
impl PWIA_PROPID_TO_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wtypes")]
impl Default for PWIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: windows_core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: windows_core::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WIA_DEV_CAP {
    pub guid: windows_core::GUID,
    pub ulFlags: u32,
    pub bstrName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrIcon: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrCommandline: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
impl Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIA_EVENT_HANDLER = WIA_DEV_CAP;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: windows_core::GUID,
    pub lTymed: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: super::wtypes::PROPID,
    pub pszName: windows_core::PWSTR,
}
pub const WiaDevMgr: windows_core::GUID = windows_core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: windows_core::GUID = windows_core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaLog: windows_core::GUID = windows_core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: windows_core::HRESULT,
}
