#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IImageVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
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
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreviewImpl: Sized + IIterableImpl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapViewImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Bind(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BindWithProperties(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>, metadata: &::core::option::Option<super::super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ILearningModelBindingPreviewFactoryImpl: Sized {
    fn CreateFromModel(&self, model: &::core::option::Option<LearningModelPreview>) -> ::windows::core::Result<LearningModelBindingPreview>;
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
pub trait ILearningModelEvaluationResultPreviewImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Outputs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
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
pub trait ILearningModelPreviewStaticsImpl: Sized {
    fn LoadModelFromStorageFileAsync(&self, modelfile: &::core::option::Option<super::super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
    fn LoadModelFromStreamAsync(&self, modelstream: &::core::option::Option<super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LearningModelPreview>>;
}
#[cfg(feature = "deprecated")]
pub trait ILearningModelVariableDescriptorPreviewImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelFeatureKind(&self) -> ::windows::core::Result<LearningModelFeatureKindPreview>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IMapVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn KeyKind(&self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn ValidStringKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>;
    fn ValidIntegerKeys(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
    fn Fields(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISequenceVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn ElementType(&self) -> ::windows::core::Result<ILearningModelVariableDescriptorPreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITensorVariableDescriptorPreviewImpl: Sized + ILearningModelVariableDescriptorPreviewImpl {
    fn DataType(&self) -> ::windows::core::Result<FeatureElementKindPreview>;
    fn Shape(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterable<i64>>;
}
