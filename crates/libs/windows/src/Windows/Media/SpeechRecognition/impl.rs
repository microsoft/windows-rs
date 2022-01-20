pub trait ISpeechRecognitionConstraint_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&mut self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&mut self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpeechRecognitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
}
impl ISpeechRecognitionConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>() -> ISpeechRecognitionConstraint_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Tag<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Probability<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Probability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProbability<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProbability(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechRecognitionConstraint, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Tag: Tag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Probability: Probability::<Identity, Impl, OFFSET>,
            SetProbability: SetProbability::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechRecognitionConstraint as ::windows::core::Interface>::IID
    }
}
