#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DManagerImpl: Sized {
    fn TaskRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTaskRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<Print3DManager>;
    fn ShowPrintUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage>;
    fn Submitting(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceChanged(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskCompletedEventArgsImpl: Sized {
    fn Completion(&self) -> ::windows::core::Result<Print3DTaskCompletion>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<Print3DTaskDetail>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestImpl: Sized {
    fn CreateTask(&self, title: &::windows::core::HSTRING, printerid: &::windows::core::HSTRING, handler: &::core::option::Option<Print3DTaskSourceRequestedHandler>) -> ::windows::core::Result<Print3DTask>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<Print3DTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceChangedEventArgsImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DTaskSourceRequestedArgsImpl: Sized {
    fn SetSource(&self, source: &::core::option::Option<Printing3D3MFPackage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackageImpl: Sized {
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn PrintTicket(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetPrintTicket(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn ModelPart(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn SetModelPart(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>>;
    fn LoadModelFromPackageAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>>;
    fn SaveModelToPackageAsync(&self, value: &::core::option::Option<Printing3DModel>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackage2Impl: Sized {
    fn Compression(&self) -> ::windows::core::Result<Printing3DPackageCompression>;
    fn SetCompression(&self, value: Printing3DPackageCompression) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3D3MFPackageStaticsImpl: Sized {
    fn LoadAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<Printing3DColorMaterial>;
    fn SetColor(&self, value: &::core::option::Option<Printing3DColorMaterial>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialGroupImpl: Sized {
    fn Bases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DBaseMaterialStaticsImpl: Sized {
    fn Abs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pla(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValue(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterial2Impl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialGroupImpl: Sized {
    fn Colors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DColorMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DColorMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DComponentImpl: Sized {
    fn Mesh(&self) -> ::windows::core::Result<Printing3DMesh>;
    fn SetMesh(&self, value: &::core::option::Option<Printing3DMesh>) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>>;
    fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetThumbnail(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<Printing3DObjectType>;
    fn SetType(&self, value: Printing3DObjectType) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PartNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPartNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DComponentWithMatrixImpl: Sized {
    fn Component(&self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetComponent(&self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Matrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialImpl: Sized {
    fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroupImpl: Sized {
    fn Composites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
    fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroup2Impl: Sized {
    fn BaseMaterialGroup(&self) -> ::windows::core::Result<Printing3DBaseMaterialGroup>;
    fn SetBaseMaterialGroup(&self, value: &::core::option::Option<Printing3DBaseMaterialGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DCompositeMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DCompositeMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DFaceReductionOptionsImpl: Sized {
    fn MaxReductionArea(&self) -> ::windows::core::Result<f64>;
    fn SetMaxReductionArea(&self, value: f64) -> ::windows::core::Result<()>;
    fn TargetTriangleCount(&self) -> ::windows::core::Result<u32>;
    fn SetTargetTriangleCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxEdgeLength(&self) -> ::windows::core::Result<f64>;
    fn SetMaxEdgeLength(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMaterialImpl: Sized {
    fn BaseGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>>;
    fn ColorGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>>;
    fn Texture2CoordGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>>;
    fn CompositeGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>>;
    fn MultiplePropertyGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMeshImpl: Sized {
    fn VertexCount(&self) -> ::windows::core::Result<u32>;
    fn SetVertexCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn IndexCount(&self) -> ::windows::core::Result<u32>;
    fn SetIndexCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn VertexPositionsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexPositionsDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn VertexNormalsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetVertexNormalsDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleIndicesDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn TriangleMaterialIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription>;
    fn SetTriangleMaterialIndicesDescription(&self, value: &Printing3DBufferDescription) -> ::windows::core::Result<()>;
    fn GetVertexPositions(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexPositions(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetVertexNormals(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateVertexNormals(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleIndices(&self, value: u32) -> ::windows::core::Result<()>;
    fn GetTriangleMaterialIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CreateTriangleMaterialIndices(&self, value: u32) -> ::windows::core::Result<()>;
    fn BufferDescriptionSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn BufferSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn VerifyAsync(&self, value: Printing3DMeshVerificationMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMeshVerificationResultImpl: Sized {
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn NonmanifoldTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn ReversedNormalTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModelImpl: Sized {
    fn Unit(&self) -> ::windows::core::Result<Printing3DModelUnit>;
    fn SetUnit(&self, value: Printing3DModelUnit) -> ::windows::core::Result<()>;
    fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>>;
    fn Meshes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>>;
    fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>>;
    fn Material(&self) -> ::windows::core::Result<Printing3DMaterial>;
    fn SetMaterial(&self, value: &::core::option::Option<Printing3DMaterial>) -> ::windows::core::Result<()>;
    fn Build(&self) -> ::windows::core::Result<Printing3DComponent>;
    fn SetBuild(&self, value: &::core::option::Option<Printing3DComponent>) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RequiredExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn RepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Clone(&self) -> ::windows::core::Result<Printing3DModel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModel2Impl: Sized {
    fn TryPartialRepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPartialRepairWithTimeAsync(&self, maxwaittime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryReduceFacesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAsync(&self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn TryReduceFacesWithOptionsAndTimeAsync(&self, printing3dfacereductionoptions: &::core::option::Option<Printing3DFaceReductionOptions>, maxwait: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
    fn RepairWithProgressAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DModelTextureImpl: Sized {
    fn TextureResource(&self) -> ::windows::core::Result<Printing3DTextureResource>;
    fn SetTextureResource(&self, value: &::core::option::Option<Printing3DTextureResource>) -> ::windows::core::Result<()>;
    fn TileStyleU(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
    fn TileStyleV(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior>;
    fn SetTileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialImpl: Sized {
    fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialGroupImpl: Sized {
    fn MultipleProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>>;
    fn MaterialGroupIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DMultiplePropertyMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DMultiplePropertyMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialImpl: Sized {
    fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
    fn U(&self) -> ::windows::core::Result<f64>;
    fn SetU(&self, value: f64) -> ::windows::core::Result<()>;
    fn V(&self) -> ::windows::core::Result<f64>;
    fn SetV(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroupImpl: Sized {
    fn Texture2Coords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>>;
    fn MaterialGroupId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroup2Impl: Sized {
    fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture>;
    fn SetTexture(&self, value: &::core::option::Option<Printing3DModelTexture>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTexture2CoordMaterialGroupFactoryImpl: Sized {
    fn Create(&self, materialgroupid: u32) -> ::windows::core::Result<Printing3DTexture2CoordMaterialGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrinting3DTextureResourceImpl: Sized {
    fn TextureData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn SetTextureData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
