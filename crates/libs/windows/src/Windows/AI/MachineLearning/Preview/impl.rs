#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IImageVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IImageVariableDescriptorPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageVariableDescriptorPreviewImpl, const OFFSET: isize>() -> IImageVariableDescriptorPreviewVtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: IImageVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitmapPixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IImageVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IImageVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImageVariableDescriptorPreview>, ::windows::core::GetTrustLevel, BitmapPixelFormat::<Impl, OFFSET>, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IInferencingOptionsPreviewImpl: Sized {
    fn PreferredDeviceKind(&self) -> ::windows::core::Result<LearningModelDeviceKindPreview>;
    fn SetPreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> ::windows::core::Result<()>;
    fn IsTracingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTracingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<i32>;
    fn SetMaxBatchSize(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinimizeMemoryAllocation(&self) -> ::windows::core::Result<bool>;
    fn SetMinimizeMemoryAllocation(&self, value: bool) -> ::windows::core::Result<()>;
    fn ReclaimMemoryAfterEvaluation(&self) -> ::windows::core::Result<bool>;
    fn SetReclaimMemoryAfterEvaluation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInferencingOptionsPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IInferencingOptionsPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>() -> IInferencingOptionsPreviewVtbl {
        unsafe extern "system" fn PreferredDeviceKind<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelDeviceKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredDeviceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredDeviceKind<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LearningModelDeviceKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredDeviceKind(value).into()
        }
        unsafe extern "system" fn IsTracingEnabled<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTracingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTracingEnabled<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTracingEnabled(value).into()
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBatchSize<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBatchSize(value).into()
        }
        unsafe extern "system" fn MinimizeMemoryAllocation<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimizeMemoryAllocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimizeMemoryAllocation<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimizeMemoryAllocation(value).into()
        }
        unsafe extern "system" fn ReclaimMemoryAfterEvaluation<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReclaimMemoryAfterEvaluation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReclaimMemoryAfterEvaluation<Impl: IInferencingOptionsPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReclaimMemoryAfterEvaluation(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInferencingOptionsPreview>,
            ::windows::core::GetTrustLevel,
            PreferredDeviceKind::<Impl, OFFSET>,
            SetPreferredDeviceKind::<Impl, OFFSET>,
            IsTracingEnabled::<Impl, OFFSET>,
            SetIsTracingEnabled::<Impl, OFFSET>,
            MaxBatchSize::<Impl, OFFSET>,
            SetMaxBatchSize::<Impl, OFFSET>,
            MinimizeMemoryAllocation::<Impl, OFFSET>,
            SetMinimizeMemoryAllocation::<Impl, OFFSET>,
            ReclaimMemoryAfterEvaluation::<Impl, OFFSET>,
            SetReclaimMemoryAfterEvaluation::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreviewImpl: Sized + IIterableImpl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapViewImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Bind(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BindWithProperties(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>, metadata: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelBindingPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelBindingPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingPreviewImpl, const OFFSET: isize>() -> ILearningModelBindingPreviewVtbl {
        unsafe extern "system" fn Bind<Impl: ILearningModelBindingPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindWithProperties<Impl: ILearningModelBindingPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, metadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .BindWithProperties(
                    &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&metadata as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Clear<Impl: ILearningModelBindingPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelBindingPreview>, ::windows::core::GetTrustLevel, Bind::<Impl, OFFSET>, BindWithProperties::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreviewFactoryImpl: Sized {
    fn CreateFromModel(&self, model: &::core::option::Option<LearningModelPreview>) -> ::windows::core::Result<LearningModelBindingPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelBindingPreviewFactory {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelBindingPreviewFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingPreviewFactoryImpl, const OFFSET: isize>() -> ILearningModelBindingPreviewFactoryVtbl {
        unsafe extern "system" fn CreateFromModel<Impl: ILearningModelBindingPreviewFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromModel(&*(&model as *const <LearningModelPreview as ::windows::core::Abi>::Abi as *const <LearningModelPreview as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelBindingPreviewFactory>, ::windows::core::GetTrustLevel, CreateFromModel::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelDescriptionPreviewImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<i64>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn InputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>;
    fn OutputFeatures(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelDescriptionPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelDescriptionPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>() -> ILearningModelDescriptionPreviewVtbl {
        unsafe extern "system" fn Author<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Domain<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Version<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Metadata<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputFeatures<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFeatures<Impl: ILearningModelDescriptionPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILearningModelDescriptionPreview>,
            ::windows::core::GetTrustLevel,
            Author::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Domain::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            Metadata::<Impl, OFFSET>,
            InputFeatures::<Impl, OFFSET>,
            OutputFeatures::<Impl, OFFSET>,
        )
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelEvaluationResultPreviewImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Outputs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelEvaluationResultPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelEvaluationResultPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelEvaluationResultPreviewImpl, const OFFSET: isize>() -> ILearningModelEvaluationResultPreviewVtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILearningModelEvaluationResultPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Outputs<Impl: ILearningModelEvaluationResultPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Outputs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelEvaluationResultPreview>, ::windows::core::GetTrustLevel, CorrelationId::<Impl, OFFSET>, Outputs::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelPreviewImpl: Sized {
    fn EvaluateAsync(&self, binding: &::core::option::Option<LearningModelBindingPreview>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>;
    fn EvaluateFeaturesAsync(&self, features: &::core::option::Option<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>;
    fn Description(&self) -> ::windows::core::Result<LearningModelDescriptionPreview>;
    fn InferencingOptions(&self) -> ::windows::core::Result<InferencingOptionsPreview>;
    fn SetInferencingOptions(&self, value: &::core::option::Option<InferencingOptionsPreview>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelPreviewImpl, const OFFSET: isize>() -> ILearningModelPreviewVtbl {
        unsafe extern "system" fn EvaluateAsync<Impl: ILearningModelPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateAsync(&*(&binding as *const <LearningModelBindingPreview as ::windows::core::Abi>::Abi as *const <LearningModelBindingPreview as ::windows::core::DefaultType>::DefaultType), &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateFeaturesAsync<Impl: ILearningModelPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateFeaturesAsync(
                &*(&features as *const <super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILearningModelPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InferencingOptions<Impl: ILearningModelPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InferencingOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInferencingOptions<Impl: ILearningModelPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInferencingOptions(&*(&value as *const <InferencingOptionsPreview as ::windows::core::Abi>::Abi as *const <InferencingOptionsPreview as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelPreview>, ::windows::core::GetTrustLevel, EvaluateAsync::<Impl, OFFSET>, EvaluateFeaturesAsync::<Impl, OFFSET>, Description::<Impl, OFFSET>, InferencingOptions::<Impl, OFFSET>, SetInferencingOptions::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelPreviewStaticsImpl: Sized {
    fn LoadModelFromStorageFileAsync(&self, modelfile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
    fn LoadModelFromStreamAsync(&self, modelstream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelPreviewStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelPreviewStaticsImpl, const OFFSET: isize>() -> ILearningModelPreviewStaticsVtbl {
        unsafe extern "system" fn LoadModelFromStorageFileAsync<Impl: ILearningModelPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadModelFromStorageFileAsync(&*(&modelfile as *const <super::super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadModelFromStreamAsync<Impl: ILearningModelPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadModelFromStreamAsync(&*(&modelstream as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelPreviewStatics>, ::windows::core::GetTrustLevel, LoadModelFromStorageFileAsync::<Impl, OFFSET>, LoadModelFromStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "deprecated")]
pub trait ILearningModelVariableDescriptorPreviewImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ILearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelVariableDescriptorPreview";
}
#[cfg(feature = "deprecated")]
impl ILearningModelVariableDescriptorPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelVariableDescriptorPreviewImpl, const OFFSET: isize>() -> ILearningModelVariableDescriptorPreviewVtbl {
        unsafe extern "system" fn Name<Impl: ILearningModelVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: ILearningModelVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelFeatureKind<Impl: ILearningModelVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKindPreview) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRequired<Impl: ILearningModelVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILearningModelVariableDescriptorPreview>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Description::<Impl, OFFSET>, ModelFeatureKind::<Impl, OFFSET>, IsRequired::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMapVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn KeyKind(&self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn ValidStringKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>;
    fn ValidIntegerKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
    fn Fields(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IMapVariableDescriptorPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapVariableDescriptorPreviewImpl, const OFFSET: isize>() -> IMapVariableDescriptorPreviewVtbl {
        unsafe extern "system" fn KeyKind<Impl: IMapVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidStringKeys<Impl: IMapVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidStringKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidIntegerKeys<Impl: IMapVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidIntegerKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fields<Impl: IMapVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapVariableDescriptorPreview>, ::windows::core::GetTrustLevel, KeyKind::<Impl, OFFSET>, ValidStringKeys::<Impl, OFFSET>, ValidIntegerKeys::<Impl, OFFSET>, Fields::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISequenceVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn ElementType(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISequenceVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISequenceVariableDescriptorPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISequenceVariableDescriptorPreviewImpl, const OFFSET: isize>() -> ISequenceVariableDescriptorPreviewVtbl {
        unsafe extern "system" fn ElementType<Impl: ISequenceVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISequenceVariableDescriptorPreview>, ::windows::core::GetTrustLevel, ElementType::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITensorVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn DataType(&self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn Shape(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ITensorVariableDescriptorPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorVariableDescriptorPreviewImpl, const OFFSET: isize>() -> ITensorVariableDescriptorPreviewVtbl {
        unsafe extern "system" fn DataType<Impl: ITensorVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: ITensorVariableDescriptorPreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITensorVariableDescriptorPreview>, ::windows::core::GetTrustLevel, DataType::<Impl, OFFSET>, Shape::<Impl, OFFSET>)
    }
}
