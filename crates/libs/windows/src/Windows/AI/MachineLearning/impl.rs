#[cfg(feature = "implement_exclusive")]
pub trait IImageFeatureDescriptorImpl: Sized {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageFeatureDescriptor2Impl: Sized {
    fn PixelRange(&self) -> ::windows::core::Result<LearningModelPixelRange>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageFeatureValueImpl: Sized {
    fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageFeatureValueStaticsImpl: Sized {
    fn CreateFromVideoFrame(&self, image: &::core::option::Option<super::super::Media::VideoFrame>) -> ::windows::core::Result<ImageFeatureValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<i64>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn InputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>;
    fn OutputFeatures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelBindingImpl: Sized {
    fn Bind(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn BindWithProperties(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelBindingFactoryImpl: Sized {
    fn CreateFromSession(&self, session: &::core::option::Option<LearningModelSession>) -> ::windows::core::Result<LearningModelBinding>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelDeviceImpl: Sized {
    fn AdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId>;
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelDeviceFactoryImpl: Sized {
    fn Create(&self, devicekind: LearningModelDeviceKind) -> ::windows::core::Result<LearningModelDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelDeviceStaticsImpl: Sized {
    fn CreateFromDirect3D11Device(&self, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<LearningModelDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelEvaluationResultImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ErrorStatus(&self) -> ::windows::core::Result<i32>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn Outputs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
pub trait ILearningModelFeatureDescriptorImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
pub trait ILearningModelFeatureValueImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<LearningModelFeatureKind>;
}
pub trait ILearningModelOperatorProviderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionImpl: Sized {
    fn Model(&self) -> ::windows::core::Result<LearningModel>;
    fn Device(&self) -> ::windows::core::Result<LearningModelDevice>;
    fn EvaluationProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn EvaluateAsync(&self, bindings: &::core::option::Option<LearningModelBinding>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>;
    fn EvaluateFeaturesAsync(&self, features: &::core::option::Option<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>;
    fn Evaluate(&self, bindings: &::core::option::Option<LearningModelBinding>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult>;
    fn EvaluateFeatures(&self, features: &::core::option::Option<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>, correlationid: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModelEvaluationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionFactoryImpl: Sized {
    fn CreateFromModel(&self, model: &::core::option::Option<LearningModel>) -> ::windows::core::Result<LearningModelSession>;
    fn CreateFromModelOnDevice(&self, model: &::core::option::Option<LearningModel>, devicetorunon: &::core::option::Option<LearningModelDevice>) -> ::windows::core::Result<LearningModelSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionFactory2Impl: Sized {
    fn CreateFromModelOnDeviceWithSessionOptions(&self, model: &::core::option::Option<LearningModel>, devicetorunon: &::core::option::Option<LearningModelDevice>, learningmodelsessionoptions: &::core::option::Option<LearningModelSessionOptions>) -> ::windows::core::Result<LearningModelSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptionsImpl: Sized {
    fn BatchSizeOverride(&self) -> ::windows::core::Result<u32>;
    fn SetBatchSizeOverride(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptions2Impl: Sized {
    fn CloseModelOnSessionCreation(&self) -> ::windows::core::Result<bool>;
    fn SetCloseModelOnSessionCreation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelSessionOptions3Impl: Sized {
    fn OverrideNamedDimension(&self, name: &::windows::core::HSTRING, dimension: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILearningModelStaticsImpl: Sized {
    fn LoadFromStorageFileAsync(&self, modelfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromStreamAsync(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromFilePath(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStream(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStorageFileWithOperatorProviderAsync(&self, modelfile: &::core::option::Option<super::super::Storage::IStorageFile>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromStreamWithOperatorProviderAsync(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>;
    fn LoadFromFilePathWithOperatorProvider(&self, filepath: &::windows::core::HSTRING, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<LearningModel>;
    fn LoadFromStreamWithOperatorProvider(&self, modelstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, operatorprovider: &::core::option::Option<ILearningModelOperatorProvider>) -> ::windows::core::Result<LearningModel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapFeatureDescriptorImpl: Sized {
    fn KeyKind(&self) -> ::windows::core::Result<TensorKind>;
    fn ValueDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISequenceFeatureDescriptorImpl: Sized {
    fn ElementDescriptor(&self) -> ::windows::core::Result<ILearningModelFeatureDescriptor>;
}
pub trait ITensorImpl: Sized + ILearningModelFeatureValueImpl {
    fn TensorKind(&self) -> ::windows::core::Result<TensorKind>;
    fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorBooleanImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorBooleanStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorBoolean>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<bool>>) -> ::windows::core::Result<TensorBoolean>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorBooleanStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorBoolean>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorBoolean>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorDoubleImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorDoubleStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorDouble>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f64>>) -> ::windows::core::Result<TensorDouble>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorDoubleStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorDouble>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorDouble>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFeatureDescriptorImpl: Sized {
    fn TensorKind(&self) -> ::windows::core::Result<TensorKind>;
    fn Shape(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloatImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloat16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloat16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorFloat16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f32>>) -> ::windows::core::Result<TensorFloat16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloat16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorFloat16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloatStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorFloat>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<f32>>) -> ::windows::core::Result<TensorFloat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorFloatStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorFloat>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorFloat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i16>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i16>>) -> ::windows::core::Result<TensorInt16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt32BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt32BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt32Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<TensorInt32Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt32BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt32Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt32Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt64BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt64BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt64Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt64Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt64BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt64Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt64Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt8BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt8BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorInt8Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u8>>) -> ::windows::core::Result<TensorInt8Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorInt8BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorInt8Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorInt8Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorStringImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorStringStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorString>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorString>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorString>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<TensorString>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorStringStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorString>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt16BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u16>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt16BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt16Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u16>>) -> ::windows::core::Result<TensorUInt16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt16BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt16Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt16Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt32BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt32BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt32Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<TensorUInt32Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt32BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt32Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt32Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt64BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt64BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt64Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u64>>) -> ::windows::core::Result<TensorUInt64Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt64BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt64Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt64Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt8BitImpl: Sized {
    fn GetAsVectorView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt8BitStaticsImpl: Sized {
    fn Create(&self) -> ::windows::core::Result<TensorUInt8Bit>;
    fn Create2(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromArray(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromIterable(&self, shape: &::core::option::Option<super::super::Foundation::Collections::IIterable<i64>>, data: &::core::option::Option<super::super::Foundation::Collections::IIterable<u8>>) -> ::windows::core::Result<TensorUInt8Bit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITensorUInt8BitStatics2Impl: Sized {
    fn CreateFromShapeArrayAndDataArray(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], data: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TensorUInt8Bit>;
    fn CreateFromBuffer(&self, shape: &[<i64 as ::windows::core::DefaultType>::DefaultType], buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<TensorUInt8Bit>;
}
