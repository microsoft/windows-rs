#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISceneBoundingBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneComponentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneComponentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMaterialFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMaterialInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMaterialInputFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMeshMaterialAttributeMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMeshRendererComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMeshRendererComponentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMeshStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterialStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneModelTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneNodeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneNodeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScenePbrMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScenePbrMaterialFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneRendererComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneRendererComponentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInputStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneVisualStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: SceneAlphaMode = SceneAlphaMode(0i32);
    pub const AlphaTest: SceneAlphaMode = SceneAlphaMode(1i32);
    pub const Blend: SceneAlphaMode = SceneAlphaMode(2i32);
}
#[repr(transparent)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: SceneAttributeSemantic = SceneAttributeSemantic(0i32);
    pub const Vertex: SceneAttributeSemantic = SceneAttributeSemantic(1i32);
    pub const Normal: SceneAttributeSemantic = SceneAttributeSemantic(2i32);
    pub const TexCoord0: SceneAttributeSemantic = SceneAttributeSemantic(3i32);
    pub const TexCoord1: SceneAttributeSemantic = SceneAttributeSemantic(4i32);
    pub const Color: SceneAttributeSemantic = SceneAttributeSemantic(5i32);
    pub const Tangent: SceneAttributeSemantic = SceneAttributeSemantic(6i32);
}
#[repr(transparent)]
pub struct SceneBoundingBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneComponentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: SceneComponentType = SceneComponentType(0i32);
}
#[repr(transparent)]
pub struct SceneMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneMaterialInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneMeshMaterialAttributeMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneMeshRendererComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneMetallicRoughnessMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneModelTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneNodeCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScenePbrMaterial(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneRendererComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneSurfaceMaterialInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: SceneWrappingMode = SceneWrappingMode(0i32);
    pub const MirroredRepeat: SceneWrappingMode = SceneWrappingMode(1i32);
    pub const Repeat: SceneWrappingMode = SceneWrappingMode(2i32);
}
