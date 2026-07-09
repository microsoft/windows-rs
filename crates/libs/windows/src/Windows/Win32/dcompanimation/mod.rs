windows_core::imp::define_interface!(IDCompositionAnimation, IDCompositionAnimation_Vtbl, 0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
windows_core::imp::interface_hierarchy!(IDCompositionAnimation, windows_core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAbsoluteBeginTime)(windows_core::Interface::as_raw(self), begintime) }
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddCubic)(windows_core::Interface::as_raw(self), beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient) }
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSinusoidal)(windows_core::Interface::as_raw(self), beginoffset, bias, amplitude, frequency, phase) }
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRepeat)(windows_core::Interface::as_raw(self), beginoffset, durationtorepeat) }
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), endoffset, endvalue) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f32) -> windows_core::HRESULT,
}
pub trait IDCompositionAnimation_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetAbsoluteBeginTime(&self, begintime: i64) -> windows_core::Result<()>;
    fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
    fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> windows_core::Result<()>;
    fn End(&self, endoffset: f64, endvalue: f32) -> windows_core::Result<()>;
}
impl IDCompositionAnimation_Vtbl {
    pub const fn new<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, begintime: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::SetAbsoluteBeginTime(this, core::mem::transmute_copy(&begintime)).into()
            }
        }
        unsafe extern "system" fn AddCubic<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddCubic(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
            }
        }
        unsafe extern "system" fn AddSinusoidal<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
            }
        }
        unsafe extern "system" fn AddRepeat<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::AddRepeat(this, core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&durationtorepeat)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IDCompositionAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endoffset: f64, endvalue: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDCompositionAnimation_Impl::End(this, core::mem::transmute_copy(&endoffset), core::mem::transmute_copy(&endvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Identity, OFFSET>,
            AddCubic: AddCubic::<Identity, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, OFFSET>,
            AddRepeat: AddRepeat::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCompositionAnimation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDCompositionAnimation {}
