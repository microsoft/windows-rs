#[doc = "*Required features: `\"Media_SpeechRecognition\"`, `\"implement\"`*"]
pub trait ISpeechRecognitionConstraint_Impl: Sized {
    fn IsEnabled(&self) -> ::windows_core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()>;
    fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Type(&self) -> ::windows_core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&self) -> ::windows_core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpeechRecognitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
}
impl ISpeechRecognitionConstraint_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>() -> ISpeechRecognitionConstraint_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Tag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTag(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Probability<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Probability() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProbability<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProbability(value).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISpeechRecognitionConstraint, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Tag: Tag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Probability: Probability::<Identity, Impl, OFFSET>,
            SetProbability: SetProbability::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecognitionConstraint as ::windows_core::ComInterface>::IID
    }
}
