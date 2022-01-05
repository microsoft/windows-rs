#[cfg(feature = "implement_exclusive")]
pub trait ISceneBoundingBoxImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Extents(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentImpl: Sized {
    fn ComponentType(&self) -> ::windows::core::Result<SceneComponentType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneComponentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialInputImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMaterialInputFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<SceneBoundingBox>;
    fn PrimitiveTopology(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>;
    fn SetPrimitiveTopology(&self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::core::Result<()>;
    fn FillMeshAttribute(&self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: &::core::option::Option<super::super::super::Foundation::MemoryBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshMaterialAttributeMapImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshRendererComponentImpl: Sized {
    fn Material(&self) -> ::windows::core::Result<SceneMaterial>;
    fn SetMaterial(&self, value: &::core::option::Option<SceneMaterial>) -> ::windows::core::Result<()>;
    fn Mesh(&self) -> ::windows::core::Result<SceneMesh>;
    fn SetMesh(&self, value: &::core::option::Option<SceneMesh>) -> ::windows::core::Result<()>;
    fn UVMappings(&self) -> ::windows::core::Result<SceneMeshMaterialAttributeMap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshRendererComponentStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMeshRendererComponent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMeshStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMesh>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMetallicRoughnessMaterialImpl: Sized {
    fn BaseColorInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetBaseColorInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn BaseColorFactor(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector4>;
    fn SetBaseColorFactor(&self, value: &super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn MetallicFactor(&self) -> ::windows::core::Result<f32>;
    fn SetMetallicFactor(&self, value: f32) -> ::windows::core::Result<()>;
    fn MetallicRoughnessInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetMetallicRoughnessInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn RoughnessFactor(&self) -> ::windows::core::Result<f32>;
    fn SetRoughnessFactor(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneMetallicRoughnessMaterialStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneMetallicRoughnessMaterial>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneModelTransformImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn RotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Translation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetTranslation(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeImpl: Sized {
    fn Children(&self) -> ::windows::core::Result<SceneNodeCollection>;
    fn Components(&self) -> ::windows::core::Result<SceneComponentCollection>;
    fn Parent(&self) -> ::windows::core::Result<SceneNode>;
    fn Transform(&self) -> ::windows::core::Result<SceneModelTransform>;
    fn FindFirstComponentOfType(&self, value: SceneComponentType) -> ::windows::core::Result<SceneComponent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneNodeStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IScenePbrMaterialImpl: Sized {
    fn AlphaCutoff(&self) -> ::windows::core::Result<f32>;
    fn SetAlphaCutoff(&self, value: f32) -> ::windows::core::Result<()>;
    fn AlphaMode(&self) -> ::windows::core::Result<SceneAlphaMode>;
    fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::core::Result<()>;
    fn EmissiveInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetEmissiveInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn EmissiveFactor(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn SetEmissiveFactor(&self, value: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn IsDoubleSided(&self) -> ::windows::core::Result<bool>;
    fn SetIsDoubleSided(&self, value: bool) -> ::windows::core::Result<()>;
    fn NormalInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetNormalInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn NormalScale(&self) -> ::windows::core::Result<f32>;
    fn SetNormalScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn OcclusionInput(&self) -> ::windows::core::Result<SceneMaterialInput>;
    fn SetOcclusionInput(&self, value: &::core::option::Option<SceneMaterialInput>) -> ::windows::core::Result<()>;
    fn OcclusionStrength(&self) -> ::windows::core::Result<f32>;
    fn SetOcclusionStrength(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScenePbrMaterialFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneRendererComponentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneRendererComponentFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneSurfaceMaterialInputImpl: Sized {
    fn BitmapInterpolationMode(&self) -> ::windows::core::Result<super::CompositionBitmapInterpolationMode>;
    fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> ::windows::core::Result<()>;
    fn Surface(&self) -> ::windows::core::Result<super::ICompositionSurface>;
    fn SetSurface(&self, value: &::core::option::Option<super::ICompositionSurface>) -> ::windows::core::Result<()>;
    fn WrappingUMode(&self) -> ::windows::core::Result<SceneWrappingMode>;
    fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()>;
    fn WrappingVMode(&self) -> ::windows::core::Result<SceneWrappingMode>;
    fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneSurfaceMaterialInputStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneSurfaceMaterialInput>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneVisualImpl: Sized {
    fn Root(&self) -> ::windows::core::Result<SceneNode>;
    fn SetRoot(&self, value: &::core::option::Option<SceneNode>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneVisualStaticsImpl: Sized {
    fn Create(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<SceneVisual>;
}
