pub trait ILampArrayEffect_Impl: Sized {
    fn ZIndex(&self) -> ::windows_core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffect_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: isize>() -> ILampArrayEffect_Vtbl {
        unsafe extern "system" fn ZIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZIndex(value).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILampArrayEffect, OFFSET>(),
            ZIndex: ZIndex::<Identity, Impl, OFFSET>,
            SetZIndex: SetZIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ILampArrayEffect as ::windows_core::ComInterface>::IID
    }
}
