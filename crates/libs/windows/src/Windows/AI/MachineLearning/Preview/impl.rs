#[cfg(all(feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IImageVariableDescriptorPreview_Impl: Sized + ILearningModelVariableDescriptorPreview_Impl {
    fn BitmapPixelFormat(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IImageVariableDescriptorPreview";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "deprecated", feature = "implement_exclusive"))]
impl IImageVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageVariableDescriptorPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn BitmapPixelFormat<Impl: IImageVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Width<Impl: IImageVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Height<Impl: IImageVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageVariableDescriptorPreview, BASE_OFFSET>(),
            BitmapPixelFormat: BitmapPixelFormat::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageVariableDescriptorPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IInferencingOptionsPreview_Impl: Sized {
    fn PreferredDeviceKind(&mut self) -> ::windows::core::Result<LearningModelDeviceKindPreview>;
    fn SetPreferredDeviceKind(&mut self, value: LearningModelDeviceKindPreview) -> ::windows::core::Result<()>;
    fn IsTracingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsTracingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MaxBatchSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxBatchSize(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MinimizeMemoryAllocation(&mut self) -> ::windows::core::Result<bool>;
    fn SetMinimizeMemoryAllocation(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ReclaimMemoryAfterEvaluation(&mut self) -> ::windows::core::Result<bool>;
    fn SetReclaimMemoryAfterEvaluation(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInferencingOptionsPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IInferencingOptionsPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IInferencingOptionsPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInferencingOptionsPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInferencingOptionsPreview_Vtbl {
        unsafe extern "system" fn PreferredDeviceKind<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelDeviceKindPreview) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredDeviceKind<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LearningModelDeviceKindPreview) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredDeviceKind(value).into()
        }
        unsafe extern "system" fn IsTracingEnabled<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsTracingEnabled<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTracingEnabled(value).into()
        }
        unsafe extern "system" fn MaxBatchSize<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxBatchSize<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxBatchSize(value).into()
        }
        unsafe extern "system" fn MinimizeMemoryAllocation<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinimizeMemoryAllocation<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimizeMemoryAllocation(value).into()
        }
        unsafe extern "system" fn ReclaimMemoryAfterEvaluation<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReclaimMemoryAfterEvaluation<Impl: IInferencingOptionsPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReclaimMemoryAfterEvaluation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInferencingOptionsPreview, BASE_OFFSET>(),
            PreferredDeviceKind: PreferredDeviceKind::<Impl, IMPL_OFFSET>,
            SetPreferredDeviceKind: SetPreferredDeviceKind::<Impl, IMPL_OFFSET>,
            IsTracingEnabled: IsTracingEnabled::<Impl, IMPL_OFFSET>,
            SetIsTracingEnabled: SetIsTracingEnabled::<Impl, IMPL_OFFSET>,
            MaxBatchSize: MaxBatchSize::<Impl, IMPL_OFFSET>,
            SetMaxBatchSize: SetMaxBatchSize::<Impl, IMPL_OFFSET>,
            MinimizeMemoryAllocation: MinimizeMemoryAllocation::<Impl, IMPL_OFFSET>,
            SetMinimizeMemoryAllocation: SetMinimizeMemoryAllocation::<Impl, IMPL_OFFSET>,
            ReclaimMemoryAfterEvaluation: ReclaimMemoryAfterEvaluation::<Impl, IMPL_OFFSET>,
            SetReclaimMemoryAfterEvaluation: SetReclaimMemoryAfterEvaluation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInferencingOptionsPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreview_Impl: Sized + super::super::super::Foundation::Collections::IIterable_Impl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + super::super::super::Foundation::Collections::IMapView_Impl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Bind(&mut self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BindWithProperties(&mut self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>, metadata: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelBindingPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelBindingPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelBindingPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelBindingPreview_Vtbl {
        unsafe extern "system" fn Bind<Impl: ILearningModelBindingPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BindWithProperties<Impl: ILearningModelBindingPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, metadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .BindWithProperties(
                    &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&metadata as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Clear<Impl: ILearningModelBindingPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelBindingPreview, BASE_OFFSET>(),
            Bind: Bind::<Impl, IMPL_OFFSET>,
            BindWithProperties: BindWithProperties::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelBindingPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreviewFactory_Impl: Sized {
    fn CreateFromModel(&mut self, model: &::core::option::Option<LearningModelPreview>) -> ::windows::core::Result<LearningModelBindingPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelBindingPreviewFactory {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelBindingPreviewFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelBindingPreviewFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelBindingPreviewFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelBindingPreviewFactory_Vtbl {
        unsafe extern "system" fn CreateFromModel<Impl: ILearningModelBindingPreviewFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, model: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelBindingPreviewFactory, BASE_OFFSET>(),
            CreateFromModel: CreateFromModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelBindingPreviewFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelDescriptionPreview_Impl: Sized {
    fn Author(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&mut self) -> ::windows::core::Result<i64>;
    fn Metadata(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn InputFeatures(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>;
    fn OutputFeatures(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelDescriptionPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelDescriptionPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelDescriptionPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelDescriptionPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelDescriptionPreview_Vtbl {
        unsafe extern "system" fn Author<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Domain<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Version<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Metadata<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputFeatures<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputFeatures<Impl: ILearningModelDescriptionPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelDescriptionPreview, BASE_OFFSET>(),
            Author: Author::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            Metadata: Metadata::<Impl, IMPL_OFFSET>,
            InputFeatures: InputFeatures::<Impl, IMPL_OFFSET>,
            OutputFeatures: OutputFeatures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelDescriptionPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelEvaluationResultPreview_Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Outputs(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelEvaluationResultPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelEvaluationResultPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelEvaluationResultPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelEvaluationResultPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelEvaluationResultPreview_Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILearningModelEvaluationResultPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Outputs<Impl: ILearningModelEvaluationResultPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelEvaluationResultPreview, BASE_OFFSET>(),
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            Outputs: Outputs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelEvaluationResultPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelPreview_Impl: Sized {
    fn EvaluateAsync(&mut self, binding: &::core::option::Option<LearningModelBindingPreview>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>;
    fn EvaluateFeaturesAsync(&mut self, features: &::core::option::Option<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>;
    fn Description(&mut self) -> ::windows::core::Result<LearningModelDescriptionPreview>;
    fn InferencingOptions(&mut self) -> ::windows::core::Result<InferencingOptionsPreview>;
    fn SetInferencingOptions(&mut self, value: &::core::option::Option<InferencingOptionsPreview>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelPreview";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelPreview_Vtbl {
        unsafe extern "system" fn EvaluateAsync<Impl: ILearningModelPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EvaluateFeaturesAsync<Impl: ILearningModelPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, features: ::windows::core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: ILearningModelPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InferencingOptions<Impl: ILearningModelPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInferencingOptions<Impl: ILearningModelPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInferencingOptions(&*(&value as *const <InferencingOptionsPreview as ::windows::core::Abi>::Abi as *const <InferencingOptionsPreview as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelPreview, BASE_OFFSET>(),
            EvaluateAsync: EvaluateAsync::<Impl, IMPL_OFFSET>,
            EvaluateFeaturesAsync: EvaluateFeaturesAsync::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            InferencingOptions: InferencingOptions::<Impl, IMPL_OFFSET>,
            SetInferencingOptions: SetInferencingOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelPreviewStatics_Impl: Sized {
    fn LoadModelFromStorageFileAsync(&mut self, modelfile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
    fn LoadModelFromStreamAsync(&mut self, modelstream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILearningModelPreviewStatics {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ILearningModelPreviewStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ILearningModelPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILearningModelPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILearningModelPreviewStatics_Vtbl {
        unsafe extern "system" fn LoadModelFromStorageFileAsync<Impl: ILearningModelPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadModelFromStreamAsync<Impl: ILearningModelPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILearningModelPreviewStatics, BASE_OFFSET>(),
            LoadModelFromStorageFileAsync: LoadModelFromStorageFileAsync::<Impl, IMPL_OFFSET>,
            LoadModelFromStreamAsync: LoadModelFromStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILearningModelPreviewStatics as ::windows::core::Interface>::IID
    }
}
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
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMapVariableDescriptorPreview_Impl: Sized + ILearningModelVariableDescriptorPreview_Impl {
    fn KeyKind(&mut self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn ValidStringKeys(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>;
    fn ValidIntegerKeys(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
    fn Fields(&mut self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMapVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.IMapVariableDescriptorPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IMapVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapVariableDescriptorPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn KeyKind<Impl: IMapVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValidStringKeys<Impl: IMapVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValidIntegerKeys<Impl: IMapVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Fields<Impl: IMapVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMapVariableDescriptorPreview, BASE_OFFSET>(),
            KeyKind: KeyKind::<Impl, IMPL_OFFSET>,
            ValidStringKeys: ValidStringKeys::<Impl, IMPL_OFFSET>,
            ValidIntegerKeys: ValidIntegerKeys::<Impl, IMPL_OFFSET>,
            Fields: Fields::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapVariableDescriptorPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISequenceVariableDescriptorPreview_Impl: Sized + ILearningModelVariableDescriptorPreview_Impl {
    fn ElementType(&mut self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISequenceVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ISequenceVariableDescriptorPreview";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISequenceVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISequenceVariableDescriptorPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISequenceVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn ElementType<Impl: ISequenceVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISequenceVariableDescriptorPreview, BASE_OFFSET>(),
            ElementType: ElementType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISequenceVariableDescriptorPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITensorVariableDescriptorPreview_Impl: Sized + ILearningModelVariableDescriptorPreview_Impl {
    fn DataType(&mut self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn Shape(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITensorVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ITensorVariableDescriptorPreview";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ITensorVariableDescriptorPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITensorVariableDescriptorPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITensorVariableDescriptorPreview_Vtbl {
        unsafe extern "system" fn DataType<Impl: ITensorVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Shape<Impl: ITensorVariableDescriptorPreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITensorVariableDescriptorPreview, BASE_OFFSET>(),
            DataType: DataType::<Impl, IMPL_OFFSET>,
            Shape: Shape::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITensorVariableDescriptorPreview as ::windows::core::Interface>::IID
    }
}
