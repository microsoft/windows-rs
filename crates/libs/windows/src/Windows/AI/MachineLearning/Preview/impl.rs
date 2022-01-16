#[cfg(feature = "deprecated")]
pub trait ILearningModelVariableDescriptorPreview_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelFeatureKind(&mut self) -> ::windows::core::Result<LearningModelFeatureKindPreview>;
    fn IsRequired(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ILearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ILearningModelVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelVariableDescriptorPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn Name<Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelFeatureKind<Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelFeatureKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Impl: ILearningModelVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelVariableDescriptorPreview, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            ModelFeatureKind: ModelFeatureKind::<Impl, IMPL_OFFSET>,
            IsRequired: IsRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelVariableDescriptorPreview as ::windows::core::Interface>::IID
    }
}
