windows_core::imp::define_interface!(IInertiaProcessor, IInertiaProcessor_Vtbl, 0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
windows_core::imp::interface_hierarchy!(IInertiaProcessor, windows_core::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn get_InitialOriginX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialOriginX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialOriginX(&self, x: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialOriginX)(windows_core::Interface::as_raw(self), x) }
    }
    pub unsafe fn get_InitialOriginY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialOriginY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialOriginY(&self, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialOriginY)(windows_core::Interface::as_raw(self), y) }
    }
    pub unsafe fn get_InitialVelocityX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialVelocityX(&self, x: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialVelocityX)(windows_core::Interface::as_raw(self), x) }
    }
    pub unsafe fn get_InitialVelocityY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialVelocityY(&self, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialVelocityY)(windows_core::Interface::as_raw(self), y) }
    }
    pub unsafe fn get_InitialAngularVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialAngularVelocity(&self, velocity: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialAngularVelocity)(windows_core::Interface::as_raw(self), velocity) }
    }
    pub unsafe fn get_InitialExpansionVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialExpansionVelocity(&self, velocity: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialExpansionVelocity)(windows_core::Interface::as_raw(self), velocity) }
    }
    pub unsafe fn get_InitialRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialRadius(&self, radius: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialRadius)(windows_core::Interface::as_raw(self), radius) }
    }
    pub unsafe fn get_BoundaryLeft(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoundaryLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BoundaryLeft(&self, left: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BoundaryLeft)(windows_core::Interface::as_raw(self), left) }
    }
    pub unsafe fn get_BoundaryTop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoundaryTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BoundaryTop(&self, top: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BoundaryTop)(windows_core::Interface::as_raw(self), top) }
    }
    pub unsafe fn get_BoundaryRight(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoundaryRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BoundaryRight(&self, right: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BoundaryRight)(windows_core::Interface::as_raw(self), right) }
    }
    pub unsafe fn get_BoundaryBottom(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoundaryBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BoundaryBottom(&self, bottom: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BoundaryBottom)(windows_core::Interface::as_raw(self), bottom) }
    }
    pub unsafe fn get_ElasticMarginLeft(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElasticMarginLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ElasticMarginLeft(&self, left: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ElasticMarginLeft)(windows_core::Interface::as_raw(self), left) }
    }
    pub unsafe fn get_ElasticMarginTop(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElasticMarginTop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ElasticMarginTop(&self, top: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ElasticMarginTop)(windows_core::Interface::as_raw(self), top) }
    }
    pub unsafe fn get_ElasticMarginRight(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElasticMarginRight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ElasticMarginRight(&self, right: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ElasticMarginRight)(windows_core::Interface::as_raw(self), right) }
    }
    pub unsafe fn get_ElasticMarginBottom(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElasticMarginBottom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ElasticMarginBottom(&self, bottom: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ElasticMarginBottom)(windows_core::Interface::as_raw(self), bottom) }
    }
    pub unsafe fn get_DesiredDisplacement(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredDisplacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredDisplacement(&self, displacement: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredDisplacement)(windows_core::Interface::as_raw(self), displacement) }
    }
    pub unsafe fn get_DesiredRotation(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredRotation(&self, rotation: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredRotation)(windows_core::Interface::as_raw(self), rotation) }
    }
    pub unsafe fn get_DesiredExpansion(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredExpansion(&self, expansion: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredExpansion)(windows_core::Interface::as_raw(self), expansion) }
    }
    pub unsafe fn get_DesiredDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredDeceleration(&self, deceleration: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredDeceleration)(windows_core::Interface::as_raw(self), deceleration) }
    }
    pub unsafe fn get_DesiredAngularDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredAngularDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredAngularDeceleration)(windows_core::Interface::as_raw(self), deceleration) }
    }
    pub unsafe fn get_DesiredExpansionDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredExpansionDeceleration)(windows_core::Interface::as_raw(self), deceleration) }
    }
    pub unsafe fn get_InitialTimestamp(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InitialTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_InitialTimestamp(&self, timestamp: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InitialTimestamp)(windows_core::Interface::as_raw(self), timestamp) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Process(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessTime)(windows_core::Interface::as_raw(self), timestamp, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Complete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteTime)(windows_core::Interface::as_raw(self), timestamp) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_InitialOriginX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialOriginX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialOriginY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialOriginY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_InitialRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_BoundaryLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_BoundaryLeft: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_BoundaryTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_BoundaryTop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_BoundaryRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_BoundaryRight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_BoundaryBottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_BoundaryBottom: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_ElasticMarginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_ElasticMarginLeft: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_ElasticMarginTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_ElasticMarginTop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_ElasticMarginRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_ElasticMarginRight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_ElasticMarginBottom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_ElasticMarginBottom: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredAngularDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredAngularDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_DesiredExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_DesiredExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_InitialTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_InitialTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ProcessTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IInertiaProcessor_Impl: windows_core::IUnknownImpl {
    fn get_InitialOriginX(&self) -> windows_core::Result<f32>;
    fn put_InitialOriginX(&self, x: f32) -> windows_core::Result<()>;
    fn get_InitialOriginY(&self) -> windows_core::Result<f32>;
    fn put_InitialOriginY(&self, y: f32) -> windows_core::Result<()>;
    fn get_InitialVelocityX(&self) -> windows_core::Result<f32>;
    fn put_InitialVelocityX(&self, x: f32) -> windows_core::Result<()>;
    fn get_InitialVelocityY(&self) -> windows_core::Result<f32>;
    fn put_InitialVelocityY(&self, y: f32) -> windows_core::Result<()>;
    fn get_InitialAngularVelocity(&self) -> windows_core::Result<f32>;
    fn put_InitialAngularVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn get_InitialExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn put_InitialExpansionVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn get_InitialRadius(&self) -> windows_core::Result<f32>;
    fn put_InitialRadius(&self, radius: f32) -> windows_core::Result<()>;
    fn get_BoundaryLeft(&self) -> windows_core::Result<f32>;
    fn put_BoundaryLeft(&self, left: f32) -> windows_core::Result<()>;
    fn get_BoundaryTop(&self) -> windows_core::Result<f32>;
    fn put_BoundaryTop(&self, top: f32) -> windows_core::Result<()>;
    fn get_BoundaryRight(&self) -> windows_core::Result<f32>;
    fn put_BoundaryRight(&self, right: f32) -> windows_core::Result<()>;
    fn get_BoundaryBottom(&self) -> windows_core::Result<f32>;
    fn put_BoundaryBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn get_ElasticMarginLeft(&self) -> windows_core::Result<f32>;
    fn put_ElasticMarginLeft(&self, left: f32) -> windows_core::Result<()>;
    fn get_ElasticMarginTop(&self) -> windows_core::Result<f32>;
    fn put_ElasticMarginTop(&self, top: f32) -> windows_core::Result<()>;
    fn get_ElasticMarginRight(&self) -> windows_core::Result<f32>;
    fn put_ElasticMarginRight(&self, right: f32) -> windows_core::Result<()>;
    fn get_ElasticMarginBottom(&self) -> windows_core::Result<f32>;
    fn put_ElasticMarginBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn get_DesiredDisplacement(&self) -> windows_core::Result<f32>;
    fn put_DesiredDisplacement(&self, displacement: f32) -> windows_core::Result<()>;
    fn get_DesiredRotation(&self) -> windows_core::Result<f32>;
    fn put_DesiredRotation(&self, rotation: f32) -> windows_core::Result<()>;
    fn get_DesiredExpansion(&self) -> windows_core::Result<f32>;
    fn put_DesiredExpansion(&self, expansion: f32) -> windows_core::Result<()>;
    fn get_DesiredDeceleration(&self) -> windows_core::Result<f32>;
    fn put_DesiredDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn get_DesiredAngularDeceleration(&self) -> windows_core::Result<f32>;
    fn put_DesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn get_DesiredExpansionDeceleration(&self) -> windows_core::Result<f32>;
    fn put_DesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn get_InitialTimestamp(&self) -> windows_core::Result<u32>;
    fn put_InitialTimestamp(&self, timestamp: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Process(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<windows_core::BOOL>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn CompleteTime(&self, timestamp: u32) -> windows_core::Result<()>;
}
impl IInertiaProcessor_Vtbl {
    pub const fn new<Identity: IInertiaProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_InitialOriginX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialOriginX(this) {
                    Ok(ok__) => {
                        x.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialOriginX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialOriginX(this, core::mem::transmute_copy(&x)).into()
            }
        }
        unsafe extern "system" fn get_InitialOriginY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialOriginY(this) {
                    Ok(ok__) => {
                        y.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialOriginY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialOriginY(this, core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn get_InitialVelocityX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialVelocityX(this) {
                    Ok(ok__) => {
                        x.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialVelocityX<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialVelocityX(this, core::mem::transmute_copy(&x)).into()
            }
        }
        unsafe extern "system" fn get_InitialVelocityY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialVelocityY(this) {
                    Ok(ok__) => {
                        y.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialVelocityY<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialVelocityY(this, core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn get_InitialAngularVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialAngularVelocity(this) {
                    Ok(ok__) => {
                        velocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialAngularVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialAngularVelocity(this, core::mem::transmute_copy(&velocity)).into()
            }
        }
        unsafe extern "system" fn get_InitialExpansionVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialExpansionVelocity(this) {
                    Ok(ok__) => {
                        velocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialExpansionVelocity<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialExpansionVelocity(this, core::mem::transmute_copy(&velocity)).into()
            }
        }
        unsafe extern "system" fn get_InitialRadius<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialRadius(this) {
                    Ok(ok__) => {
                        radius.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialRadius<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialRadius(this, core::mem::transmute_copy(&radius)).into()
            }
        }
        unsafe extern "system" fn get_BoundaryLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_BoundaryLeft(this) {
                    Ok(ok__) => {
                        left.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BoundaryLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_BoundaryLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn get_BoundaryTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_BoundaryTop(this) {
                    Ok(ok__) => {
                        top.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BoundaryTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_BoundaryTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn get_BoundaryRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_BoundaryRight(this) {
                    Ok(ok__) => {
                        right.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BoundaryRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_BoundaryRight(this, core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn get_BoundaryBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_BoundaryBottom(this) {
                    Ok(ok__) => {
                        bottom.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BoundaryBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_BoundaryBottom(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn get_ElasticMarginLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_ElasticMarginLeft(this) {
                    Ok(ok__) => {
                        left.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ElasticMarginLeft<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_ElasticMarginLeft(this, core::mem::transmute_copy(&left)).into()
            }
        }
        unsafe extern "system" fn get_ElasticMarginTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_ElasticMarginTop(this) {
                    Ok(ok__) => {
                        top.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ElasticMarginTop<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_ElasticMarginTop(this, core::mem::transmute_copy(&top)).into()
            }
        }
        unsafe extern "system" fn get_ElasticMarginRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_ElasticMarginRight(this) {
                    Ok(ok__) => {
                        right.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ElasticMarginRight<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_ElasticMarginRight(this, core::mem::transmute_copy(&right)).into()
            }
        }
        unsafe extern "system" fn get_ElasticMarginBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_ElasticMarginBottom(this) {
                    Ok(ok__) => {
                        bottom.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ElasticMarginBottom<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_ElasticMarginBottom(this, core::mem::transmute_copy(&bottom)).into()
            }
        }
        unsafe extern "system" fn get_DesiredDisplacement<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredDisplacement(this) {
                    Ok(ok__) => {
                        displacement.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredDisplacement<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredDisplacement(this, core::mem::transmute_copy(&displacement)).into()
            }
        }
        unsafe extern "system" fn get_DesiredRotation<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredRotation(this) {
                    Ok(ok__) => {
                        rotation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredRotation<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn get_DesiredExpansion<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredExpansion(this) {
                    Ok(ok__) => {
                        expansion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredExpansion<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredExpansion(this, core::mem::transmute_copy(&expansion)).into()
            }
        }
        unsafe extern "system" fn get_DesiredDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn get_DesiredAngularDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredAngularDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredAngularDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredAngularDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn get_DesiredExpansionDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_DesiredExpansionDeceleration(this) {
                    Ok(ok__) => {
                        deceleration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredExpansionDeceleration<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_DesiredExpansionDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
            }
        }
        unsafe extern "system" fn get_InitialTimestamp<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::get_InitialTimestamp(this) {
                    Ok(ok__) => {
                        timestamp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InitialTimestamp<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::put_InitialTimestamp(this, core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Process<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::Process(this) {
                    Ok(ok__) => {
                        completed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessTime<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32, completed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaProcessor_Impl::ProcessTime(this, core::mem::transmute_copy(&timestamp)) {
                    Ok(ok__) => {
                        completed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Complete<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::Complete(this).into()
            }
        }
        unsafe extern "system" fn CompleteTime<Identity: IInertiaProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaProcessor_Impl::CompleteTime(this, core::mem::transmute_copy(&timestamp)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_InitialOriginX: get_InitialOriginX::<Identity, OFFSET>,
            put_InitialOriginX: put_InitialOriginX::<Identity, OFFSET>,
            get_InitialOriginY: get_InitialOriginY::<Identity, OFFSET>,
            put_InitialOriginY: put_InitialOriginY::<Identity, OFFSET>,
            get_InitialVelocityX: get_InitialVelocityX::<Identity, OFFSET>,
            put_InitialVelocityX: put_InitialVelocityX::<Identity, OFFSET>,
            get_InitialVelocityY: get_InitialVelocityY::<Identity, OFFSET>,
            put_InitialVelocityY: put_InitialVelocityY::<Identity, OFFSET>,
            get_InitialAngularVelocity: get_InitialAngularVelocity::<Identity, OFFSET>,
            put_InitialAngularVelocity: put_InitialAngularVelocity::<Identity, OFFSET>,
            get_InitialExpansionVelocity: get_InitialExpansionVelocity::<Identity, OFFSET>,
            put_InitialExpansionVelocity: put_InitialExpansionVelocity::<Identity, OFFSET>,
            get_InitialRadius: get_InitialRadius::<Identity, OFFSET>,
            put_InitialRadius: put_InitialRadius::<Identity, OFFSET>,
            get_BoundaryLeft: get_BoundaryLeft::<Identity, OFFSET>,
            put_BoundaryLeft: put_BoundaryLeft::<Identity, OFFSET>,
            get_BoundaryTop: get_BoundaryTop::<Identity, OFFSET>,
            put_BoundaryTop: put_BoundaryTop::<Identity, OFFSET>,
            get_BoundaryRight: get_BoundaryRight::<Identity, OFFSET>,
            put_BoundaryRight: put_BoundaryRight::<Identity, OFFSET>,
            get_BoundaryBottom: get_BoundaryBottom::<Identity, OFFSET>,
            put_BoundaryBottom: put_BoundaryBottom::<Identity, OFFSET>,
            get_ElasticMarginLeft: get_ElasticMarginLeft::<Identity, OFFSET>,
            put_ElasticMarginLeft: put_ElasticMarginLeft::<Identity, OFFSET>,
            get_ElasticMarginTop: get_ElasticMarginTop::<Identity, OFFSET>,
            put_ElasticMarginTop: put_ElasticMarginTop::<Identity, OFFSET>,
            get_ElasticMarginRight: get_ElasticMarginRight::<Identity, OFFSET>,
            put_ElasticMarginRight: put_ElasticMarginRight::<Identity, OFFSET>,
            get_ElasticMarginBottom: get_ElasticMarginBottom::<Identity, OFFSET>,
            put_ElasticMarginBottom: put_ElasticMarginBottom::<Identity, OFFSET>,
            get_DesiredDisplacement: get_DesiredDisplacement::<Identity, OFFSET>,
            put_DesiredDisplacement: put_DesiredDisplacement::<Identity, OFFSET>,
            get_DesiredRotation: get_DesiredRotation::<Identity, OFFSET>,
            put_DesiredRotation: put_DesiredRotation::<Identity, OFFSET>,
            get_DesiredExpansion: get_DesiredExpansion::<Identity, OFFSET>,
            put_DesiredExpansion: put_DesiredExpansion::<Identity, OFFSET>,
            get_DesiredDeceleration: get_DesiredDeceleration::<Identity, OFFSET>,
            put_DesiredDeceleration: put_DesiredDeceleration::<Identity, OFFSET>,
            get_DesiredAngularDeceleration: get_DesiredAngularDeceleration::<Identity, OFFSET>,
            put_DesiredAngularDeceleration: put_DesiredAngularDeceleration::<Identity, OFFSET>,
            get_DesiredExpansionDeceleration: get_DesiredExpansionDeceleration::<Identity, OFFSET>,
            put_DesiredExpansionDeceleration: put_DesiredExpansionDeceleration::<Identity, OFFSET>,
            get_InitialTimestamp: get_InitialTimestamp::<Identity, OFFSET>,
            put_InitialTimestamp: put_InitialTimestamp::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            ProcessTime: ProcessTime::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            CompleteTime: CompleteTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInertiaProcessor {}
windows_core::imp::define_interface!(IManipulationProcessor, IManipulationProcessor_Vtbl, 0xa22ac519_8300_48a0_bef4_f1be8737dba4);
windows_core::imp::interface_hierarchy!(IManipulationProcessor, windows_core::IUnknown);
impl IManipulationProcessor {
    pub unsafe fn get_SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedManipulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SupportedManipulations)(windows_core::Interface::as_raw(self), manipulations) }
    }
    pub unsafe fn get_PivotPointX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PivotPointX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PivotPointX(&self, pivotpointx: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PivotPointX)(windows_core::Interface::as_raw(self), pivotpointx) }
    }
    pub unsafe fn get_PivotPointY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PivotPointY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PivotPointY(&self, pivotpointy: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PivotPointY)(windows_core::Interface::as_raw(self), pivotpointy) }
    }
    pub unsafe fn get_PivotRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PivotRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PivotRadius(&self, pivotradius: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PivotRadius)(windows_core::Interface::as_raw(self), pivotradius) }
    }
    pub unsafe fn CompleteManipulation(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteManipulation)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ProcessDown(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessDown)(windows_core::Interface::as_raw(self), manipulatorid, x, y) }
    }
    pub unsafe fn ProcessMove(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMove)(windows_core::Interface::as_raw(self), manipulatorid, x, y) }
    }
    pub unsafe fn ProcessUp(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessUp)(windows_core::Interface::as_raw(self), manipulatorid, x, y) }
    }
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessDownWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp) }
    }
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMoveWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp) }
    }
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessUpWithTime)(windows_core::Interface::as_raw(self), manipulatorid, x, y, timestamp) }
    }
    pub unsafe fn GetVelocityX(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocityX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVelocityY(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocityY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetExpansionVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExpansionVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAngularVelocity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAngularVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_MinimumScaleRotateRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinimumScaleRotateRadius)(windows_core::Interface::as_raw(self), minradius) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_SupportedManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT,
    pub put_SupportedManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT,
    pub get_PivotPointX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_PivotPointX: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_PivotPointY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_PivotPointY: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub get_PivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_PivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub CompleteManipulation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessDown: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32) -> windows_core::HRESULT,
    pub ProcessMove: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32) -> windows_core::HRESULT,
    pub ProcessUp: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32) -> windows_core::HRESULT,
    pub ProcessDownWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32, u32) -> windows_core::HRESULT,
    pub ProcessMoveWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32, u32) -> windows_core::HRESULT,
    pub ProcessUpWithTime: unsafe extern "system" fn(*mut core::ffi::c_void, MANIPULATOR_ID, f32, f32, u32) -> windows_core::HRESULT,
    pub GetVelocityX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetVelocityY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetExpansionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetAngularVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub get_MinimumScaleRotateRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub put_MinimumScaleRotateRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IManipulationProcessor_Impl: windows_core::IUnknownImpl {
    fn get_SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn put_SupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::Result<()>;
    fn get_PivotPointX(&self) -> windows_core::Result<f32>;
    fn put_PivotPointX(&self, pivotpointx: f32) -> windows_core::Result<()>;
    fn get_PivotPointY(&self) -> windows_core::Result<f32>;
    fn put_PivotPointY(&self, pivotpointy: f32) -> windows_core::Result<()>;
    fn get_PivotRadius(&self) -> windows_core::Result<f32>;
    fn put_PivotRadius(&self, pivotradius: f32) -> windows_core::Result<()>;
    fn CompleteManipulation(&self) -> windows_core::Result<()>;
    fn ProcessDown(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessMove(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessUp(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessDownWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessMoveWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessUpWithTime(&self, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn GetVelocityX(&self) -> windows_core::Result<f32>;
    fn GetVelocityY(&self) -> windows_core::Result<f32>;
    fn GetExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn GetAngularVelocity(&self) -> windows_core::Result<f32>;
    fn get_MinimumScaleRotateRadius(&self) -> windows_core::Result<f32>;
    fn put_MinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::Result<()>;
}
impl IManipulationProcessor_Vtbl {
    pub const fn new<Identity: IManipulationProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SupportedManipulations<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::get_SupportedManipulations(this) {
                    Ok(ok__) => {
                        manipulations.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SupportedManipulations<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::put_SupportedManipulations(this, core::mem::transmute_copy(&manipulations)).into()
            }
        }
        unsafe extern "system" fn get_PivotPointX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::get_PivotPointX(this) {
                    Ok(ok__) => {
                        pivotpointx.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PivotPointX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::put_PivotPointX(this, core::mem::transmute_copy(&pivotpointx)).into()
            }
        }
        unsafe extern "system" fn get_PivotPointY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::get_PivotPointY(this) {
                    Ok(ok__) => {
                        pivotpointy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PivotPointY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::put_PivotPointY(this, core::mem::transmute_copy(&pivotpointy)).into()
            }
        }
        unsafe extern "system" fn get_PivotRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::get_PivotRadius(this) {
                    Ok(ok__) => {
                        pivotradius.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PivotRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::put_PivotRadius(this, core::mem::transmute_copy(&pivotradius)).into()
            }
        }
        unsafe extern "system" fn CompleteManipulation<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::CompleteManipulation(this).into()
            }
        }
        unsafe extern "system" fn ProcessDown<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessDown(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessMove<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessMove(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessUp<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessUp(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessDownWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessMoveWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: MANIPULATOR_ID, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::ProcessUpWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
            }
        }
        unsafe extern "system" fn GetVelocityX<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityx: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetVelocityX(this) {
                    Ok(ok__) => {
                        velocityx.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVelocityY<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityy: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetVelocityY(this) {
                    Ok(ok__) => {
                        velocityy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansionvelocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetExpansionVelocity(this) {
                    Ok(ok__) => {
                        expansionvelocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, angularvelocity: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::GetAngularVelocity(this) {
                    Ok(ok__) => {
                        angularvelocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MinimumScaleRotateRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationProcessor_Impl::get_MinimumScaleRotateRadius(this) {
                    Ok(ok__) => {
                        minradius.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinimumScaleRotateRadius<Identity: IManipulationProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationProcessor_Impl::put_MinimumScaleRotateRadius(this, core::mem::transmute_copy(&minradius)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_SupportedManipulations: get_SupportedManipulations::<Identity, OFFSET>,
            put_SupportedManipulations: put_SupportedManipulations::<Identity, OFFSET>,
            get_PivotPointX: get_PivotPointX::<Identity, OFFSET>,
            put_PivotPointX: put_PivotPointX::<Identity, OFFSET>,
            get_PivotPointY: get_PivotPointY::<Identity, OFFSET>,
            put_PivotPointY: put_PivotPointY::<Identity, OFFSET>,
            get_PivotRadius: get_PivotRadius::<Identity, OFFSET>,
            put_PivotRadius: put_PivotRadius::<Identity, OFFSET>,
            CompleteManipulation: CompleteManipulation::<Identity, OFFSET>,
            ProcessDown: ProcessDown::<Identity, OFFSET>,
            ProcessMove: ProcessMove::<Identity, OFFSET>,
            ProcessUp: ProcessUp::<Identity, OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Identity, OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Identity, OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Identity, OFFSET>,
            GetVelocityX: GetVelocityX::<Identity, OFFSET>,
            GetVelocityY: GetVelocityY::<Identity, OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Identity, OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Identity, OFFSET>,
            get_MinimumScaleRotateRadius: get_MinimumScaleRotateRadius::<Identity, OFFSET>,
            put_MinimumScaleRotateRadius: put_MinimumScaleRotateRadius::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationProcessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManipulationProcessor {}
pub const InertiaProcessor: windows_core::GUID = windows_core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = 15;
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = 0;
pub type MANIPULATION_PROCESSOR_MANIPULATIONS = u32;
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = 8;
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = 4;
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = 1;
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = 2;
pub type MANIPULATOR_ID = u32;
pub const ManipulationProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
windows_core::imp::define_interface!(_IManipulationEvents, _IManipulationEvents_Vtbl, 0x4f62c8da_9c53_4b22_93df_927a862bbb03);
windows_core::imp::interface_hierarchy!(_IManipulationEvents, windows_core::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ManipulationStarted)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ManipulationDelta)(windows_core::Interface::as_raw(self), x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation) }
    }
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ManipulationCompleted)(windows_core::Interface::as_raw(self), x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub trait _IManipulationEvents_Impl: windows_core::IUnknownImpl {
    fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::Result<()>;
    fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
    fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
}
impl _IManipulationEvents_Vtbl {
    pub const fn new<Identity: _IManipulationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ManipulationStarted<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationStarted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn ManipulationDelta<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationDelta(
                    this,
                    core::mem::transmute_copy(&x),
                    core::mem::transmute_copy(&y),
                    core::mem::transmute_copy(&translationdeltax),
                    core::mem::transmute_copy(&translationdeltay),
                    core::mem::transmute_copy(&scaledelta),
                    core::mem::transmute_copy(&expansiondelta),
                    core::mem::transmute_copy(&rotationdelta),
                    core::mem::transmute_copy(&cumulativetranslationx),
                    core::mem::transmute_copy(&cumulativetranslationy),
                    core::mem::transmute_copy(&cumulativescale),
                    core::mem::transmute_copy(&cumulativeexpansion),
                    core::mem::transmute_copy(&cumulativerotation),
                )
                .into()
            }
        }
        unsafe extern "system" fn ManipulationCompleted<Identity: _IManipulationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                _IManipulationEvents_Impl::ManipulationCompleted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cumulativetranslationx), core::mem::transmute_copy(&cumulativetranslationy), core::mem::transmute_copy(&cumulativescale), core::mem::transmute_copy(&cumulativeexpansion), core::mem::transmute_copy(&cumulativerotation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Identity, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IManipulationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for _IManipulationEvents {}
