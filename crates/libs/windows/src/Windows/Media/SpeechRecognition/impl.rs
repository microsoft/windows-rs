pub trait ISpeechRecognitionConstraint_Impl: Sized {
    fn IsEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn Tag(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&self) -> windows_core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpeechRecognitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
}
impl ISpeechRecognitionConstraint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>() -> ISpeechRecognitionConstraint_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISpeechRecognitionConstraint_Impl::IsEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISpeechRecognitionConstraint_Impl::SetIsEnabled(this, value).into()
        }
        unsafe extern "system" fn Tag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISpeechRecognitionConstraint_Impl::Tag(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISpeechRecognitionConstraint_Impl::SetTag(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISpeechRecognitionConstraint_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Probability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISpeechRecognitionConstraint_Impl::Probability(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProbability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISpeechRecognitionConstraint_Impl::SetProbability(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpeechRecognitionConstraint, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Tag: Tag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Probability: Probability::<Identity, Impl, OFFSET>,
            SetProbability: SetProbability::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecognitionConstraint as windows_core::Interface>::IID
    }
}
