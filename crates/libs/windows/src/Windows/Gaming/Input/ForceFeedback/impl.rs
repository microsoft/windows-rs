pub trait IForceFeedbackEffect_Impl: Sized {
    fn Gain(&self) -> ::windows_core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows_core::Result<()>;
    fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState>;
    fn Start(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IForceFeedbackEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect";
}
impl IForceFeedbackEffect_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>() -> IForceFeedbackEffect_Vtbl {
        unsafe extern "system" fn Gain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Gain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGain(value).into()
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IForceFeedbackEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IForceFeedbackEffect, OFFSET>(),
            Gain: Gain::<Identity, Impl, OFFSET>,
            SetGain: SetGain::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IForceFeedbackEffect as ::windows_core::ComInterface>::IID
    }
}
