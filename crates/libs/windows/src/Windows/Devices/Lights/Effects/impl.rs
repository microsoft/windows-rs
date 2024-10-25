pub trait ILampArrayEffect_Impl: Sized + windows_core::IUnknownImpl {
    fn ZIndex(&self) -> windows_core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffect_Vtbl {
    pub const fn new<Identity: ILampArrayEffect_Impl, const OFFSET: isize>() -> ILampArrayEffect_Vtbl {
        unsafe extern "system" fn ZIndex<Identity: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILampArrayEffect_Impl::ZIndex(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Identity: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILampArrayEffect_Impl::SetZIndex(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILampArrayEffect, OFFSET>(),
            ZIndex: ZIndex::<Identity, OFFSET>,
            SetZIndex: SetZIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILampArrayEffect as windows_core::Interface>::IID
    }
}
