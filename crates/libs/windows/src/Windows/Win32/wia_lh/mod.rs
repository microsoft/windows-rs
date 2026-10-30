windows_core::imp::define_interface!(IEnumWIA_DEV_CAPS, IEnumWIA_DEV_CAPS_Vtbl, 0x1fcc4287_aca6_11d2_a093_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_CAPS, windows_core::IUnknown);
impl IEnumWIA_DEV_CAPS {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt, pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelt: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelt as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut WIA_DEV_CAP, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumWIA_DEV_CAPS_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienum: windows_core::OutRef<IEnumWIA_DEV_CAPS>) -> windows_core::Result<()>;
    fn GetCount(&self, pcelt: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumWIA_DEV_CAPS_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
                IEnumWIA_DEV_CAPS_Impl::Clone(this, core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::GetCount(this, core::mem::transmute_copy(&pcelt)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumWIA_DEV_CAPS {}
windows_core::imp::define_interface!(IEnumWIA_DEV_INFO, IEnumWIA_DEV_INFO_Vtbl, 0x5e38b83c_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_INFO, windows_core::IUnknown);
impl IEnumWIA_DEV_INFO {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IWiaPropertyStorage>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), celt as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumWIA_DEV_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: windows_core::OutRef<IWiaPropertyStorage>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienum: windows_core::OutRef<IEnumWIA_DEV_INFO>) -> windows_core::Result<()>;
    fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumWIA_DEV_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
                IEnumWIA_DEV_INFO_Impl::Clone(this, core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::GetCount(this, core::mem::transmute_copy(&celt)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumWIA_DEV_INFO {}
windows_core::imp::define_interface!(IEnumWIA_FORMAT_INFO, IEnumWIA_FORMAT_INFO_Vtbl, 0x81befc5b_656d_44f1_b24c_d41d51b4dc81);
windows_core::imp::interface_hierarchy!(IEnumWIA_FORMAT_INFO, windows_core::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcelt: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcelt as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut WIA_FORMAT_INFO, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumWIA_FORMAT_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienum: windows_core::OutRef<IEnumWIA_FORMAT_INFO>) -> windows_core::Result<()>;
    fn GetCount(&self, pcelt: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumWIA_FORMAT_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
                IEnumWIA_FORMAT_INFO_Impl::Clone(this, core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::GetCount(this, core::mem::transmute_copy(&pcelt)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumWIA_FORMAT_INFO {}
windows_core::imp::define_interface!(IEnumWiaItem, IEnumWiaItem_Vtbl, 0x5e8383fc_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IEnumWiaItem, windows_core::IUnknown);
impl IEnumWiaItem {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, ppiwiaitem: *mut Option<IWiaItem>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), celt as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumWiaItem_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, ppiwiaitem: windows_core::OutRef<IWiaItem>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienum: windows_core::OutRef<IEnumWiaItem>) -> windows_core::Result<()>;
    fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumWiaItem_Vtbl {
    pub const fn new<Identity: IEnumWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, ppiwiaitem: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
                IEnumWiaItem_Impl::Clone(this, core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::GetCount(this, core::mem::transmute_copy(&celt)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumWiaItem {}
windows_core::imp::define_interface!(IEnumWiaItem2, IEnumWiaItem2_Vtbl, 0x59970af4_cd0d_44d9_ab24_52295630e582);
windows_core::imp::interface_hierarchy!(IEnumWiaItem2, windows_core::IUnknown);
impl IEnumWiaItem2 {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, ppiwiaitem2: *mut Option<IWiaItem2>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem2), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), celt as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumWiaItem2_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, ppiwiaitem2: windows_core::OutRef<IWiaItem2>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienum: windows_core::OutRef<IEnumWiaItem2>) -> windows_core::Result<()>;
    fn GetCount(&self, celt: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumWiaItem2_Vtbl {
    pub const fn new<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, ppiwiaitem2: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem2), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
                IEnumWiaItem2_Impl::Clone(this, core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::GetCount(this, core::mem::transmute_copy(&celt)).into()
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumWiaItem2 {}
windows_core::imp::define_interface!(IWiaAppErrorHandler, IWiaAppErrorHandler_Vtbl, 0x6c16186c_d0a6_400c_80f4_d26986a0e734);
windows_core::imp::interface_hierarchy!(IWiaAppErrorHandler, windows_core::IUnknown);
impl IWiaAppErrorHandler {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self, phwnd: *mut super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), phwnd as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ReportStatus<P1>(&self, lflags: super::LONG, pwiaitem2: P1, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::HRESULT
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
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
    #[cfg(feature = "shared")]
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, windows_core::HRESULT, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ReportStatus: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IWiaAppErrorHandler_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self, phwnd: *mut super::HWND) -> windows_core::Result<()>;
    fn ReportStatus(&self, lflags: super::LONG, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IWiaAppErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaAppErrorHandler_Impl::GetWindow(this, core::mem::transmute_copy(&phwnd)).into()
            }
        }
        unsafe extern "system" fn ReportStatus<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaAppErrorHandler {}
windows_core::imp::define_interface!(IWiaDataCallback, IWiaDataCallback_Vtbl, 0xa558a866_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataCallback, windows_core::IUnknown);
impl IWiaDataCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn BandedDataCallback(&self, lmessage: super::LONG, lstatus: super::LONG, lpercentcomplete: super::LONG, loffset: super::LONG, llength: super::LONG, lreserved: super::LONG, lreslength: super::LONG, pbbuffer: *const super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BandedDataCallback)(windows_core::Interface::as_raw(self), lmessage, lstatus, lpercentcomplete, loffset, llength, lreserved, lreslength, pbbuffer) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub BandedDataCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BandedDataCallback: usize,
}
#[cfg(feature = "shared")]
pub trait IWiaDataCallback_Impl: windows_core::IUnknownImpl {
    fn BandedDataCallback(&self, lmessage: super::LONG, lstatus: super::LONG, lpercentcomplete: super::LONG, loffset: super::LONG, llength: super::LONG, lreserved: super::LONG, lreslength: super::LONG, pbbuffer: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaDataCallback_Vtbl {
    pub const fn new<Identity: IWiaDataCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BandedDataCallback<Identity: IWiaDataCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmessage: super::LONG, lstatus: super::LONG, lpercentcomplete: super::LONG, loffset: super::LONG, llength: super::LONG, lreserved: super::LONG, lreslength: super::LONG, pbbuffer: *const super::BYTE) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWiaDataCallback {}
windows_core::imp::define_interface!(IWiaDataTransfer, IWiaDataTransfer_Vtbl, 0xa6cef998_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataTransfer, windows_core::IUnknown);
impl IWiaDataTransfer {
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn idtGetData<P1>(&self, pmedium: super::LPSTGMEDIUM, piwiadatacallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetData)(windows_core::Interface::as_raw(self), pmedium, piwiadatacallback.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn idtGetBandedData<P1>(&self, pwiadatatransinfo: PWIA_DATA_TRANSFER_INFO, piwiadatacallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetBandedData)(windows_core::Interface::as_raw(self), pwiadatatransinfo, piwiadatacallback.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn idtQueryGetData(&self, pfe: Option<*const WIA_FORMAT_INFO>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).idtQueryGetData)(windows_core::Interface::as_raw(self), pfe.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn idtEnumWIA_FORMAT_INFO(&self, ppenum: *mut Option<IEnumWIA_FORMAT_INFO>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).idtEnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: PWIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).idtGetExtendedTransferInfo)(windows_core::Interface::as_raw(self), pextendedtransferinfo as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub idtGetData: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPSTGMEDIUM, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase")))]
    idtGetData: usize,
    #[cfg(feature = "shared")]
    pub idtGetBandedData: unsafe extern "system" fn(*mut core::ffi::c_void, PWIA_DATA_TRANSFER_INFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    idtGetBandedData: usize,
    #[cfg(feature = "shared")]
    pub idtQueryGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WIA_FORMAT_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    idtQueryGetData: usize,
    pub idtEnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub idtGetExtendedTransferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, PWIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    idtGetExtendedTransferInfo: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWiaDataTransfer_Impl: windows_core::IUnknownImpl {
    fn idtGetData(&self, pmedium: super::LPSTGMEDIUM, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtGetBandedData(&self, pwiadatatransinfo: PWIA_DATA_TRANSFER_INFO, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> windows_core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(&self, ppenum: windows_core::OutRef<IEnumWIA_FORMAT_INFO>) -> windows_core::Result<()>;
    fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: PWIA_EXTENDED_TRANSFER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IWiaDataTransfer_Vtbl {
    pub const fn new<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn idtGetData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedium: super::LPSTGMEDIUM, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetData(this, core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&piwiadatacallback)).into()
            }
        }
        unsafe extern "system" fn idtGetBandedData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiadatatransinfo: PWIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
                IWiaDataTransfer_Impl::idtEnumWIA_FORMAT_INFO(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedtransferinfo: PWIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWiaDataTransfer {}
windows_core::imp::define_interface!(IWiaDevMgr, IWiaDevMgr_Vtbl, 0x5eb2502a_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IWiaDevMgr, windows_core::IUnknown);
impl IWiaDevMgr {
    #[cfg(feature = "shared")]
    pub unsafe fn EnumDeviceInfo(&self, lflag: super::LONG) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflag, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR, ppwiaitemroot: *mut Option<IWiaItem>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), core::mem::transmute(ppwiaitemroot)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetImageDlg<P4>(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, lintent: super::LONG, pitemroot: P4, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IWiaItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, lintent, pitemroot.param().abi(), core::mem::transmute_copy(bstrfilename), pguidformat as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrcommandline), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3, peventobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), core::mem::transmute(peventobject)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn AddDeviceDlg(&self, hwndparent: super::HWND, lflags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumDeviceInfo: usize,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SelectDeviceDlg: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SelectDeviceDlgID: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetImageDlg: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackProgram: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackInterface: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackCLSID: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub AddDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    AddDeviceDlg: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IWiaDevMgr_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflag: super::LONG) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR, ppwiaitemroot: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn SelectDeviceDlg(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn SelectDeviceDlgID(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn GetImageDlg(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, lintent: super::LONG, pitemroot: windows_core::Ref<IWiaItem>, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>, peventobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddDeviceDlg(&self, hwndparent: super::HWND, lflags: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IWiaDevMgr_Vtbl {
    pub const fn new<Identity: IWiaDevMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflag: super::LONG, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
                IWiaDevMgr_Impl::CreateDevice(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&ppwiaitemroot)).into()
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, lintent: super::LONG, pitemroot: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::GetImageDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&pitemroot), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pguidformat)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrcommandline: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrcommandline), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback), core::mem::transmute_copy(&peventobject)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, lflags: super::LONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaDevMgr {}
windows_core::imp::define_interface!(IWiaDevMgr2, IWiaDevMgr2_Vtbl, 0x79c07cf1_cbdd_41ee_8ec3_f00080cada7a);
windows_core::imp::interface_hierarchy!(IWiaDevMgr2, windows_core::IUnknown);
impl IWiaDevMgr2 {
    #[cfg(feature = "shared")]
    pub unsafe fn EnumDeviceInfo(&self, lflags: super::LONG) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateDevice(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, ppwiaitem2root: *mut Option<IWiaItem2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), core::mem::transmute(ppwiaitem2root)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3, peventobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), core::mem::transmute(peventobject)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrfullappname), core::mem::transmute_copy(bstrcommandlinearg), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetImageDlg(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, hwndparent: super::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut Option<IWiaItem2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, core::mem::transmute(ppitem)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumDeviceInfo: usize,
    #[cfg(feature = "shared")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateDevice: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SelectDeviceDlg: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SelectDeviceDlgID: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackInterface: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackProgram: usize,
    #[cfg(feature = "shared")]
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterEventCallbackCLSID: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetImageDlg: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IWiaDevMgr2_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflags: super::LONG) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, ppwiaitem2root: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn SelectDeviceDlg(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlgID(&self, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterEventCallbackInterface(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>, peventobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, lflags: super::LONG, bstrdeviceid: &windows_core::BSTR, hwndparent: super::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IWiaDevMgr2_Vtbl {
    pub const fn new<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateDevice<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, ppwiaitem2root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::CreateDevice(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&ppwiaitem2root)).into()
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, ldevicetype: super::LONG, lflags: super::LONG, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback), core::mem::transmute_copy(&peventobject)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrfullappname: *mut core::ffi::c_void, bstrcommandlinearg: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrfullappname), core::mem::transmute(&bstrcommandlinearg), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrdeviceid: *mut core::ffi::c_void, hwndparent: super::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaDevMgr2 {}
windows_core::imp::define_interface!(IWiaErrorHandler, IWiaErrorHandler_Vtbl, 0x0e4a51b1_bc1f_443d_a835_72e890759ef3);
windows_core::imp::interface_hierarchy!(IWiaErrorHandler, windows_core::IUnknown);
impl IWiaErrorHandler {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ReportStatus<P2>(&self, lflags: super::LONG, hwndparent: super::HWND, pwiaitem2: P2, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportStatus)(windows_core::Interface::as_raw(self), lflags, hwndparent, pwiaitem2.param().abi(), hrstatus, lpercentcomplete) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStatusDescription<P1>(&self, lflags: super::LONG, pwiaitem2: P1, hrstatus: windows_core::HRESULT, pbstrdescription: *mut windows_core::BSTR) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetStatusDescription)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), hrstatus, core::mem::transmute(pbstrdescription)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::HWND, *mut core::ffi::c_void, windows_core::HRESULT, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ReportStatus: usize,
    #[cfg(feature = "shared")]
    pub GetStatusDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStatusDescription: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IWiaErrorHandler_Impl: windows_core::IUnknownImpl {
    fn ReportStatus(&self, lflags: super::LONG, hwndparent: super::HWND, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::Result<()>;
    fn GetStatusDescription(&self, lflags: super::LONG, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IWiaErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportStatus<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, hwndparent: super::HWND, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
            }
        }
        unsafe extern "system" fn GetStatusDescription<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaErrorHandler_Impl::GetStatusDescription(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&pbstrdescription)).into()
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaErrorHandler {}
windows_core::imp::define_interface!(IWiaEventCallback, IWiaEventCallback_Vtbl, 0xae6287b0_0084_11d2_973b_00a0c9068f2e);
windows_core::imp::interface_hierarchy!(IWiaEventCallback, windows_core::IUnknown);
impl IWiaEventCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: super::DWORD, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut super::ULONG, ulreserved: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImageEventCallback)(windows_core::Interface::as_raw(self), peventguid, core::mem::transmute_copy(bstreventdescription), core::mem::transmute_copy(bstrdeviceid), core::mem::transmute_copy(bstrdevicedescription), dwdevicetype, core::mem::transmute_copy(bstrfullitemname), puleventtype as _, ulreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub ImageEventCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut super::ULONG, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ImageEventCallback: usize,
}
#[cfg(feature = "shared")]
pub trait IWiaEventCallback_Impl: windows_core::IUnknownImpl {
    fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: super::DWORD, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut super::ULONG, ulreserved: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaEventCallback_Vtbl {
    pub const fn new<Identity: IWiaEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageEventCallback<Identity: IWiaEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstreventdescription: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, bstrdevicedescription: *mut core::ffi::c_void, dwdevicetype: super::DWORD, bstrfullitemname: *mut core::ffi::c_void, puleventtype: *mut super::ULONG, ulreserved: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
    pub unsafe fn FilterPreviewImage<P1, P3>(&self, lflags: super::LONG, pwiachilditem2: P1, inputimageextents: super::RECT, pinputstream: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P3: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).FilterPreviewImage)(windows_core::Interface::as_raw(self), lflags, pwiachilditem2.param().abi(), inputimageextents, pinputstream.param().abi()) }
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
    #[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
    pub FilterPreviewImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, super::RECT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared", feature = "windef")))]
    FilterPreviewImage: usize,
    pub ApplyProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
pub trait IWiaImageFilter_Impl: windows_core::IUnknownImpl {
    fn InitializeFilter(&self, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn SetNewCallback(&self, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn FilterPreviewImage(&self, lflags: super::LONG, pwiachilditem2: windows_core::Ref<IWiaItem2>, inputimageextents: &super::RECT, pinputstream: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn ApplyProperties(&self, pwiapropertystorage: windows_core::Ref<IWiaPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
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
        unsafe extern "system" fn FilterPreviewImage<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pwiachilditem2: *mut core::ffi::c_void, inputimageextents: super::RECT, pinputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "objidlbase", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaImageFilter {}
windows_core::imp::define_interface!(IWiaItem, IWiaItem_Vtbl, 0x4db1ad10_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IWiaItem, windows_core::IUnknown);
impl IWiaItem {
    #[cfg(feature = "shared")]
    pub unsafe fn GetItemType(&self, pitemtype: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), pitemtype as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AnalyzeItem(&self, lflags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn EnumChildItems(&self, ppienumwiaitem: *mut Option<IEnumWiaItem>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienumwiaitem)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteItem(&self, lflags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateChildItem(&self, lflags: super::LONG, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR, ppiwiaitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), core::mem::transmute(ppiwiaitem)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: *mut Option<IEnumWIA_DEV_CAPS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindItemByName(&self, lflags: super::LONG, bstrfullitemname: &windows_core::BSTR, ppiwiaitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), core::mem::transmute(ppiwiaitem)) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn DeviceDlg(&self, hwndparent: super::HWND, lflags: super::LONG, lintent: super::LONG, plitemcount: *mut super::LONG, ppiwiaitem: *mut *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags, lintent, plitemcount as _, ppiwiaitem as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeviceCommand(&self, lflags: super::LONG, pcmdguid: *const windows_core::GUID, piwiaitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(piwiaitem)) }
    }
    pub unsafe fn GetRootItem(&self, ppiwiaitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiwiaitem)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: super::LONG, ppienumwia_dev_caps: *mut Option<IEnumWIA_DEV_CAPS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(ppienumwia_dev_caps)) }
    }
    pub unsafe fn DumpItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DumpItemData)(windows_core::Interface::as_raw(self), core::mem::transmute(bstrdata)) }
    }
    pub unsafe fn DumpDrvItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DumpDrvItemData)(windows_core::Interface::as_raw(self), core::mem::transmute(bstrdata)) }
    }
    pub unsafe fn DumpTreeItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DumpTreeItemData)(windows_core::Interface::as_raw(self), core::mem::transmute(bstrdata)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Diagnostic(&self, pbuffer: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), pbuffer.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetItemType: usize,
    #[cfg(feature = "shared")]
    pub AnalyzeItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AnalyzeItem: usize,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteItem: usize,
    #[cfg(feature = "shared")]
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateChildItem: usize,
    #[cfg(feature = "shared")]
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumRegisterEventInfo: usize,
    #[cfg(feature = "shared")]
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindItemByName: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::LONG, super::LONG, *mut super::LONG, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    DeviceDlg: usize,
    #[cfg(feature = "shared")]
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeviceCommand: usize,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumDeviceCapabilities: usize,
    pub DumpItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpDrvItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpTreeItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Diagnostic: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IWiaItem_Impl: windows_core::IUnknownImpl {
    fn GetItemType(&self, pitemtype: *mut super::LONG) -> windows_core::Result<()>;
    fn AnalyzeItem(&self, lflags: super::LONG) -> windows_core::Result<()>;
    fn EnumChildItems(&self, ppienumwiaitem: windows_core::OutRef<IEnumWiaItem>) -> windows_core::Result<()>;
    fn DeleteItem(&self, lflags: super::LONG) -> windows_core::Result<()>;
    fn CreateChildItem(&self, lflags: super::LONG, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR, ppiwiaitem: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn EnumRegisterEventInfo(&self, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: windows_core::OutRef<IEnumWIA_DEV_CAPS>) -> windows_core::Result<()>;
    fn FindItemByName(&self, lflags: super::LONG, bstrfullitemname: &windows_core::BSTR, ppiwiaitem: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn DeviceDlg(&self, hwndparent: super::HWND, lflags: super::LONG, lintent: super::LONG, plitemcount: *mut super::LONG, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: super::LONG, pcmdguid: *const windows_core::GUID, piwiaitem: *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn GetRootItem(&self, ppiwiaitem: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: super::LONG, ppienumwia_dev_caps: windows_core::OutRef<IEnumWIA_DEV_CAPS>) -> windows_core::Result<()>;
    fn DumpItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DumpDrvItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DumpTreeItemData(&self, bstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Diagnostic(&self, ulsize: super::ULONG, pbuffer: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IWiaItem_Vtbl {
    pub const fn new<Identity: IWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemType<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::GetItemType(this, core::mem::transmute_copy(&pitemtype)).into()
            }
        }
        unsafe extern "system" fn AnalyzeItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::AnalyzeItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::EnumChildItems(this, core::mem::transmute_copy(&ppienumwiaitem)).into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::CreateChildItem(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::HWND, lflags: super::LONG, lintent: super::LONG, plitemcount: *mut super::LONG, ppiwiaitem: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&plitemcount), core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pcmdguid: *const windows_core::GUID, piwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&piwiaitem)).into()
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::GetRootItem(this, core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppienumwia_dev_caps)).into()
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DumpItemData(this, core::mem::transmute_copy(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DumpDrvItemData(this, core::mem::transmute_copy(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DumpTreeItemData(this, core::mem::transmute_copy(&bstrdata)).into()
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: super::ULONG, pbuffer: *const super::BYTE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaItem {}
windows_core::imp::define_interface!(IWiaItem2, IWiaItem2_Vtbl, 0x6cba0075_1287_407d_9b77_cf0e030435cc);
windows_core::imp::interface_hierarchy!(IWiaItem2, windows_core::IUnknown);
impl IWiaItem2 {
    #[cfg(feature = "shared")]
    pub unsafe fn CreateChildItem(&self, litemflags: super::LONG, lcreationflags: super::LONG, bstritemname: &windows_core::BSTR, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), litemflags, lcreationflags, core::mem::transmute_copy(bstritemname), core::mem::transmute(ppiwiaitem2)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteItem(&self, lflags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn EnumChildItems(&self, pcategoryguid: Option<*const windows_core::GUID>, ppienumwiaitem2: *mut Option<IEnumWiaItem2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), pcategoryguid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppienumwiaitem2)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FindItemByName(&self, lflags: super::LONG, bstrfullitemname: &windows_core::BSTR, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), core::mem::transmute(ppiwiaitem2)) }
    }
    pub unsafe fn GetItemCategory(&self, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemCategory)(windows_core::Interface::as_raw(self), pitemcategoryguid as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetItemType(&self, pitemtype: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), pitemtype as _) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn DeviceDlg(&self, lflags: super::LONG, hwndparent: super::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: Option<*mut Option<Self>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), lflags, hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, ppitem.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeviceCommand(&self, lflags: super::LONG, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(ppiwiaitem2)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: super::LONG, ppienumwia_dev_caps: *mut Option<IEnumWIA_DEV_CAPS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(ppienumwia_dev_caps)) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn CheckExtension(&self, lflags: super::LONG, bstrname: &windows_core::BSTR, riidextensioninterface: *const super::IID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), riidextensioninterface, pbextensionexists as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetExtension<T>(&self, lflags: super::LONG, bstrname: &windows_core::BSTR) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    pub unsafe fn GetParentItem(&self, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiwiaitem2)) }
    }
    pub unsafe fn GetRootItem(&self, ppiwiaitem2: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), core::mem::transmute(ppiwiaitem2)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPreviewComponent(&self, lflags: super::LONG, ppwiapreview: *mut Option<IWiaPreview>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreviewComponent)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute(ppwiapreview)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: *mut Option<IEnumWIA_DEV_CAPS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, core::mem::transmute(ppienum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Diagnostic(&self, pbuffer: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), pbuffer.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateChildItem: usize,
    #[cfg(feature = "shared")]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteItem: usize,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FindItemByName: usize,
    pub GetItemCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetItemType: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    DeviceDlg: usize,
    #[cfg(feature = "shared")]
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeviceCommand: usize,
    #[cfg(feature = "shared")]
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumDeviceCapabilities: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub CheckExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const super::IID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    CheckExtension: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetExtension: usize,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetPreviewComponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPreviewComponent: usize,
    #[cfg(feature = "shared")]
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumRegisterEventInfo: usize,
    #[cfg(feature = "shared")]
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Diagnostic: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
pub trait IWiaItem2_Impl: windows_core::IUnknownImpl {
    fn CreateChildItem(&self, litemflags: super::LONG, lcreationflags: super::LONG, bstritemname: &windows_core::BSTR, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn DeleteItem(&self, lflags: super::LONG) -> windows_core::Result<()>;
    fn EnumChildItems(&self, pcategoryguid: *const windows_core::GUID, ppienumwiaitem2: windows_core::OutRef<IEnumWiaItem2>) -> windows_core::Result<()>;
    fn FindItemByName(&self, lflags: super::LONG, bstrfullitemname: &windows_core::BSTR, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn GetItemCategory(&self, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetItemType(&self, pitemtype: *mut super::LONG) -> windows_core::Result<()>;
    fn DeviceDlg(&self, lflags: super::LONG, hwndparent: super::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: super::LONG, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut Option<IWiaItem2>) -> windows_core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: super::LONG, ppienumwia_dev_caps: windows_core::OutRef<IEnumWIA_DEV_CAPS>) -> windows_core::Result<()>;
    fn CheckExtension(&self, lflags: super::LONG, bstrname: &windows_core::BSTR, riidextensioninterface: *const super::IID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExtension(&self, lflags: super::LONG, bstrname: &windows_core::BSTR, riidextensioninterface: *const super::IID, ppout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParentItem(&self, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn GetRootItem(&self, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn GetPreviewComponent(&self, lflags: super::LONG, ppwiapreview: windows_core::OutRef<IWiaPreview>) -> windows_core::Result<()>;
    fn EnumRegisterEventInfo(&self, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: windows_core::OutRef<IEnumWIA_DEV_CAPS>) -> windows_core::Result<()>;
    fn Diagnostic(&self, ulsize: super::ULONG, pbuffer: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
impl IWiaItem2_Vtbl {
    pub const fn new<Identity: IWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, litemflags: super::LONG, lcreationflags: super::LONG, bstritemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::CreateChildItem(this, core::mem::transmute_copy(&litemflags), core::mem::transmute_copy(&lcreationflags), core::mem::transmute(&bstritemname), core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryguid: *const windows_core::GUID, ppienumwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::EnumChildItems(this, core::mem::transmute_copy(&pcategoryguid), core::mem::transmute_copy(&ppienumwiaitem2)).into()
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn GetItemCategory<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetItemCategory(this, core::mem::transmute_copy(&pitemcategoryguid)).into()
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetItemType(this, core::mem::transmute_copy(&pitemtype)).into()
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, hwndparent: super::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut super::LONG, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppienumwia_dev_caps)).into()
            }
        }
        unsafe extern "system" fn CheckExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const super::IID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::CheckExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&pbextensionexists)).into()
            }
        }
        unsafe extern "system" fn GetExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const super::IID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&ppout)).into()
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetParentItem(this, core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetRootItem(this, core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, ppwiapreview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetPreviewComponent(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ppwiapreview)).into()
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&ppienum)).into()
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: super::ULONG, pbuffer: *const super::BYTE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IWiaItem2 {}
windows_core::imp::define_interface!(IWiaItemExtras, IWiaItemExtras_Vtbl, 0x6291ef2c_36ef_4532_876a_8e132593778d);
windows_core::imp::interface_hierarchy!(IWiaItemExtras, windows_core::IUnknown);
impl IWiaItemExtras {
    pub unsafe fn GetExtendedErrorInfo(&self, bstrerrortext: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(bstrerrortext)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Escape(&self, dwescapecode: super::DWORD, lpindata: &[super::BYTE], poutdata: *mut super::BYTE, dwoutdatasize: super::DWORD, pdwactualdatasize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), dwescapecode, lpindata.as_ptr(), lpindata.len().try_into().unwrap(), poutdata as _, dwoutdatasize, pdwactualdatasize as _) }
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
    #[cfg(feature = "shared")]
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE, super::DWORD, *mut super::BYTE, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Escape: usize,
    pub CancelPendingIO: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWiaItemExtras_Impl: windows_core::IUnknownImpl {
    fn GetExtendedErrorInfo(&self, bstrerrortext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Escape(&self, dwescapecode: super::DWORD, lpindata: *const super::BYTE, cbindatasize: super::DWORD, poutdata: *mut super::BYTE, dwoutdatasize: super::DWORD, pdwactualdatasize: *mut super::DWORD) -> windows_core::Result<()>;
    fn CancelPendingIO(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaItemExtras_Vtbl {
    pub const fn new<Identity: IWiaItemExtras_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrerrortext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItemExtras_Impl::GetExtendedErrorInfo(this, core::mem::transmute_copy(&bstrerrortext)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwescapecode: super::DWORD, lpindata: *const super::BYTE, cbindatasize: super::DWORD, poutdata: *mut super::BYTE, dwoutdatasize: super::DWORD, pdwactualdatasize: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWiaItemExtras {}
windows_core::imp::define_interface!(IWiaLog, IWiaLog_Vtbl, 0xa00c10b6_82a1_452f_8b6c_86062aad6890);
windows_core::imp::interface_hierarchy!(IWiaLog, windows_core::IUnknown);
impl IWiaLog {
    #[cfg(feature = "shared")]
    pub unsafe fn InitializeLog(&self, hinstance: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeLog)(windows_core::Interface::as_raw(self), hinstance) }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Log(&self, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub InitializeLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InitializeLog: usize,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Log: usize,
}
#[cfg(feature = "shared")]
pub trait IWiaLog_Impl: windows_core::IUnknownImpl {
    fn InitializeLog(&self, hinstance: super::LONG) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaLog_Vtbl {
    pub const fn new<Identity: IWiaLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLog<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: super::LONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Log<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWiaLog {}
windows_core::imp::define_interface!(IWiaLogEx, IWiaLogEx_Vtbl, 0xaf1f22ac_7a40_4787_b421_aeb47a1fbd0b);
windows_core::imp::interface_hierarchy!(IWiaLogEx, windows_core::IUnknown);
impl IWiaLogEx {
    #[cfg(feature = "shared")]
    pub unsafe fn InitializeLogEx(&self, hinstance: *const super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeLogEx)(windows_core::Interface::as_raw(self), hinstance) }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Log(&self, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn hResultEx(&self, lmethodid: super::LONG, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).hResultEx)(windows_core::Interface::as_raw(self), lmethodid, hresult) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn LogEx(&self, lmethodid: super::LONG, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LogEx)(windows_core::Interface::as_raw(self), lmethodid, lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub InitializeLogEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InitializeLogEx: usize,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Log: usize,
    #[cfg(feature = "shared")]
    pub hResultEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    hResultEx: usize,
    #[cfg(feature = "shared")]
    pub LogEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LogEx: usize,
}
#[cfg(feature = "shared")]
pub trait IWiaLogEx_Impl: windows_core::IUnknownImpl {
    fn InitializeLogEx(&self, hinstance: *const super::BYTE) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn hResultEx(&self, lmethodid: super::LONG, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn LogEx(&self, lmethodid: super::LONG, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaLogEx_Vtbl {
    pub const fn new<Identity: IWiaLogEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: *const super::BYTE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Log<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        unsafe extern "system" fn hResultEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: super::LONG, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::hResultEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn LogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: super::LONG, lflags: super::LONG, lresid: super::LONG, ldetail: super::LONG, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetNewPreview<P1, P2>(&self, lflags: super::LONG, pwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNewPreview)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), pwiatransfercallback.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn UpdatePreview<P1, P2>(&self, lflags: super::LONG, pchildwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdatePreview)(windows_core::Interface::as_raw(self), lflags, pchildwiaitem2.param().abi(), pwiatransfercallback.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DetectRegions(&self, lflags: super::LONG) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub GetNewPreview: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNewPreview: usize,
    #[cfg(feature = "shared")]
    pub UpdatePreview: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UpdatePreview: usize,
    #[cfg(feature = "shared")]
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DetectRegions: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWiaPreview_Impl: windows_core::IUnknownImpl {
    fn GetNewPreview(&self, lflags: super::LONG, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn UpdatePreview(&self, lflags: super::LONG, pchildwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn DetectRegions(&self, lflags: super::LONG) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWiaPreview_Vtbl {
    pub const fn new<Identity: IWiaPreview_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewPreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::GetNewPreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn UpdatePreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pchildwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::UpdatePreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pchildwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn DetectRegions<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWiaPreview {}
windows_core::imp::define_interface!(IWiaPropertyStorage, IWiaPropertyStorage_Vtbl, 0x98b5e8a0_29cc_491a_aac0_e6db4fdcceb6);
windows_core::imp::interface_hierarchy!(IWiaPropertyStorage, windows_core::IUnknown);
impl IWiaPropertyStorage {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadMultiple(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WriteMultiple(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *const super::PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgpropvar, propidnamefirst) }
    }
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[super::PROPSPEC]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), rgpspec.len().try_into().unwrap(), rgpspec.as_ptr()) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ReadPropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn WritePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[super::PROPID]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), rgpropid.len().try_into().unwrap(), rgpropid.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "propidlbase")]
    pub unsafe fn Enum(&self, ppenum: *mut Option<super::IEnumSTATPROPSTG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetTimes(&self, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetClass(&self, clsid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "propidlbase", feature = "shared"))]
    pub unsafe fn Stat(&self, pstatpsstg: *mut super::STATPROPSETSTG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyAttributes(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgflags: *mut super::ULONG, rgpropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyAttributes)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgflags as _, rgpropvar) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pulnumprops: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pulnumprops as _) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut Option<super::IStream>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid as _, core::mem::transmute(ppistream)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SetPropertyStream<P1>(&self, pcompatibilityid: *const windows_core::GUID, pistream: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid, pistream.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPSPEC, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPSPEC, *const super::PROPVARIANT, super::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    WriteMultiple: usize,
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPSPEC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    DeleteMultiple: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID, *mut super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    ReadPropertyNames: usize,
    #[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID, *const super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    WritePropertyNames: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    DeletePropertyNames: usize,
    #[cfg(feature = "shared")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "propidlbase")]
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propidlbase"))]
    Enum: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::FILETIME, *const super::FILETIME, *const super::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetTimes: usize,
    #[cfg(feature = "guiddef")]
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetClass: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "propidlbase", feature = "shared"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::STATPROPSETSTG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "propidlbase", feature = "shared")))]
    Stat: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::PROPSPEC, *mut super::ULONG, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyAttributes: usize,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(feature = "objidlbase")]
    pub GetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    GetPropertyStream: usize,
    #[cfg(feature = "objidlbase")]
    pub SetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SetPropertyStream: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWiaPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *const super::PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: super::ULONG, rgpropid: *const super::PROPID) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self, ppenum: windows_core::OutRef<super::IEnumSTATPROPSTG>) -> windows_core::Result<()>;
    fn SetTimes(&self, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const super::IID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut super::STATPROPSETSTG) -> windows_core::Result<()>;
    fn GetPropertyAttributes(&self, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgflags: *mut super::ULONG, rgpropvar: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self, pulnumprops: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: windows_core::OutRef<super::IStream>) -> windows_core::Result<()>;
    fn SetPropertyStream(&self, pcompatibilityid: *const windows_core::GUID, pistream: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWiaPropertyStorage_Vtbl {
    pub const fn new<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgpropvar: *const super::PROPVARIANT, propidnamefirst: super::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *mut super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID, rglpwstrname: *const super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: super::ULONG, rgpropid: *const super::PROPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
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
                IWiaPropertyStorage_Impl::Enum(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn SetTimes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::FILETIME, patime: *const super::FILETIME, pmtime: *const super::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut super::STATPROPSETSTG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: super::ULONG, rgpspec: *const super::PROPSPEC, rgflags: *mut super::ULONG, rgpropvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetPropertyAttributes(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgflags), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumprops: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetCount(this, core::mem::transmute_copy(&pulnumprops)).into()
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWiaPropertyStorage {}
windows_core::imp::define_interface!(IWiaSegmentationFilter, IWiaSegmentationFilter_Vtbl, 0xec46a697_ac04_4447_8f65_ff63d5154b21);
windows_core::imp::interface_hierarchy!(IWiaSegmentationFilter, windows_core::IUnknown);
impl IWiaSegmentationFilter {
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn DetectRegions<P1, P2>(&self, lflags: super::LONG, pinputstream: P1, pwiaitem2: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStream>,
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectRegions)(windows_core::Interface::as_raw(self), lflags, pinputstream.param().abi(), pwiaitem2.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    DetectRegions: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait IWiaSegmentationFilter_Impl: windows_core::IUnknownImpl {
    fn DetectRegions(&self, lflags: super::LONG, pinputstream: windows_core::Ref<super::IStream>, pwiaitem2: windows_core::Ref<IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl IWiaSegmentationFilter_Vtbl {
    pub const fn new<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectRegions<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pinputstream: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for IWiaSegmentationFilter {}
windows_core::imp::define_interface!(IWiaTransfer, IWiaTransfer_Vtbl, 0xc39d6942_2f4e_4d04_92fe_4ef4d3a1de5a);
windows_core::imp::interface_hierarchy!(IWiaTransfer, windows_core::IUnknown);
impl IWiaTransfer {
    #[cfg(feature = "shared")]
    pub unsafe fn Download<P1>(&self, lflags: super::LONG, piwiatransfercallback: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), lflags, piwiatransfercallback.param().abi()) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn Upload<P1, P2>(&self, lflags: super::LONG, psource: P1, piwiatransfercallback: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStream>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Upload)(windows_core::Interface::as_raw(self), lflags, psource.param().abi(), piwiatransfercallback.param().abi()) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumWIA_FORMAT_INFO(&self, ppenum: *mut Option<IEnumWIA_FORMAT_INFO>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Download: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub Upload: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    Upload: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait IWiaTransfer_Impl: windows_core::IUnknownImpl {
    fn Download(&self, lflags: super::LONG, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Upload(&self, lflags: super::LONG, psource: windows_core::Ref<super::IStream>, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EnumWIA_FORMAT_INFO(&self, ppenum: windows_core::OutRef<IEnumWIA_FORMAT_INFO>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl IWiaTransfer_Vtbl {
    pub const fn new<Identity: IWiaTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Download<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Download(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&piwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn Upload<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, psource: *mut core::ffi::c_void, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
                IWiaTransfer_Impl::EnumWIA_FORMAT_INFO(this, core::mem::transmute_copy(&ppenum)).into()
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for IWiaTransfer {}
windows_core::imp::define_interface!(IWiaTransferCallback, IWiaTransferCallback_Vtbl, 0x27d4eaaf_28a6_4ca5_9aab_e678168b9527);
windows_core::imp::interface_hierarchy!(IWiaTransferCallback, windows_core::IUnknown);
impl IWiaTransferCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn TransferCallback(&self, lflags: super::LONG, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TransferCallback)(windows_core::Interface::as_raw(self), lflags, pwiatransferparams) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn GetNextStream(&self, lflags: super::LONG, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR, ppdestination: *mut Option<super::IStream>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextStream)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), core::mem::transmute(ppdestination)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub TransferCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *const WiaTransferParams) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    TransferCallback: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub GetNextStream: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    GetNextStream: usize,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait IWiaTransferCallback_Impl: windows_core::IUnknownImpl {
    fn TransferCallback(&self, lflags: super::LONG, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
    fn GetNextStream(&self, lflags: super::LONG, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR, ppdestination: *mut Option<super::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl IWiaTransferCallback_Vtbl {
    pub const fn new<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferCallback<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransferCallback_Impl::TransferCallback(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
            }
        }
        unsafe extern "system" fn GetNextStream<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: super::LONG, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&ppdestination)).into()
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl windows_core::RuntimeName for IWiaTransferCallback {}
#[cfg(feature = "shared")]
pub type PWIA_DATA_CALLBACK_HEADER = *mut WIA_DATA_CALLBACK_HEADER;
#[cfg(feature = "shared")]
pub type PWIA_DATA_TRANSFER_INFO = *mut WIA_DATA_TRANSFER_INFO;
#[cfg(feature = "shared")]
pub type PWIA_DEV_CAP = *mut WIA_DEV_CAP;
#[cfg(feature = "shared")]
pub type PWIA_DITHER_PATTERN_DATA = *mut WIA_DITHER_PATTERN_DATA;
#[cfg(feature = "shared")]
pub type PWIA_EVENT_HANDLER = *mut WIA_DEV_CAP;
#[cfg(feature = "shared")]
pub type PWIA_EXTENDED_TRANSFER_INFO = *mut WIA_EXTENDED_TRANSFER_INFO;
#[cfg(feature = "shared")]
pub type PWIA_FORMAT_INFO = *mut WIA_FORMAT_INFO;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type PWIA_PROPID_TO_NAME = *mut WIA_PROPID_TO_NAME;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: super::LONG,
    pub guidFormatID: windows_core::GUID,
    pub lBufferSize: super::LONG,
    pub lPageCount: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: super::ULONG,
    pub ulSection: super::ULONG,
    pub ulBufferSize: super::ULONG,
    pub bDoubleBuffer: windows_core::BOOL,
    pub ulReserved1: super::ULONG,
    pub ulReserved2: super::ULONG,
    pub ulReserved3: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WIA_DEV_CAP {
    pub guid: windows_core::GUID,
    pub ulFlags: super::ULONG,
    pub bstrName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrIcon: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrCommandline: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: super::LONG,
    pub bstrPatternName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub lPatternWidth: super::LONG,
    pub lPatternLength: super::LONG,
    pub cbPattern: super::LONG,
    pub pbPattern: *mut super::BYTE,
}
#[cfg(feature = "shared")]
pub type WIA_EVENT_HANDLER = WIA_DEV_CAP;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: super::ULONG,
    pub ulMinBufferSize: super::ULONG,
    pub ulOptimalBufferSize: super::ULONG,
    pub ulMaxBufferSize: super::ULONG,
    pub ulNumBuffers: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: windows_core::GUID,
    pub lTymed: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: super::PROPID,
    pub pszName: super::LPOLESTR,
}
pub const WiaDevMgr: windows_core::GUID = windows_core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: windows_core::GUID = windows_core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaLog: windows_core::GUID = windows_core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WiaTransferParams {
    pub lMessage: super::LONG,
    pub lPercentComplete: super::LONG,
    pub ulTransferredBytes: super::ULONG64,
    pub hrErrorStatus: windows_core::HRESULT,
}
