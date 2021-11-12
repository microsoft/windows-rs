#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISceneBoundingBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneBoundingBox {}
impl ::core::clone::Clone for ISceneBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneComponent {}
impl ::core::clone::Clone for ISceneComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneComponentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneComponentCollection {}
impl ::core::clone::Clone for ISceneComponentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneComponentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneComponentFactory {}
impl ::core::clone::Clone for ISceneComponentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMaterial {}
impl ::core::clone::Clone for ISceneMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMaterialFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMaterialFactory {}
impl ::core::clone::Clone for ISceneMaterialFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMaterialInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMaterialInput {}
impl ::core::clone::Clone for ISceneMaterialInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMaterialInputFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMaterialInputFactory {}
impl ::core::clone::Clone for ISceneMaterialInputFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMesh(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMesh {}
impl ::core::clone::Clone for ISceneMesh {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMeshMaterialAttributeMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMeshMaterialAttributeMap {}
impl ::core::clone::Clone for ISceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMeshRendererComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMeshRendererComponent {}
impl ::core::clone::Clone for ISceneMeshRendererComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMeshRendererComponentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMeshRendererComponentStatics {}
impl ::core::clone::Clone for ISceneMeshRendererComponentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMeshStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMeshStatics {}
impl ::core::clone::Clone for ISceneMeshStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMetallicRoughnessMaterial {}
impl ::core::clone::Clone for ISceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterialStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneMetallicRoughnessMaterialStatics {}
impl ::core::clone::Clone for ISceneMetallicRoughnessMaterialStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneModelTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneModelTransform {}
impl ::core::clone::Clone for ISceneModelTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneNode {}
impl ::core::clone::Clone for ISceneNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneNodeCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneNodeCollection {}
impl ::core::clone::Clone for ISceneNodeCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneNodeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneNodeStatics {}
impl ::core::clone::Clone for ISceneNodeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneObject {}
impl ::core::clone::Clone for ISceneObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneObjectFactory {}
impl ::core::clone::Clone for ISceneObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScenePbrMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScenePbrMaterial {}
impl ::core::clone::Clone for IScenePbrMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScenePbrMaterialFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScenePbrMaterialFactory {}
impl ::core::clone::Clone for IScenePbrMaterialFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneRendererComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneRendererComponent {}
impl ::core::clone::Clone for ISceneRendererComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneRendererComponentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneRendererComponentFactory {}
impl ::core::clone::Clone for ISceneRendererComponentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneSurfaceMaterialInput {}
impl ::core::clone::Clone for ISceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInputStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneSurfaceMaterialInputStatics {}
impl ::core::clone::Clone for ISceneSurfaceMaterialInputStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneVisual {}
impl ::core::clone::Clone for ISceneVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneVisualStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneVisualStatics {}
impl ::core::clone::Clone for ISceneVisualStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: Self = Self(0i32);
    pub const AlphaTest: Self = Self(1i32);
    pub const Blend: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAlphaMode {}
impl ::core::clone::Clone for SceneAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: Self = Self(0i32);
    pub const Vertex: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const TexCoord0: Self = Self(3i32);
    pub const TexCoord1: Self = Self(4i32);
    pub const Color: Self = Self(5i32);
    pub const Tangent: Self = Self(6i32);
}
impl ::core::marker::Copy for SceneAttributeSemantic {}
impl ::core::clone::Clone for SceneAttributeSemantic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneBoundingBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneBoundingBox {}
impl ::core::clone::Clone for SceneBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneComponent {}
impl ::core::clone::Clone for SceneComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneComponentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneComponentCollection {}
impl ::core::clone::Clone for SceneComponentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: Self = Self(0i32);
}
impl ::core::marker::Copy for SceneComponentType {}
impl ::core::clone::Clone for SceneComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMaterial {}
impl ::core::clone::Clone for SceneMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMaterialInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMaterialInput {}
impl ::core::clone::Clone for SceneMaterialInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMesh(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMesh {}
impl ::core::clone::Clone for SceneMesh {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMeshMaterialAttributeMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMeshMaterialAttributeMap {}
impl ::core::clone::Clone for SceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMeshRendererComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMeshRendererComponent {}
impl ::core::clone::Clone for SceneMeshRendererComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneMetallicRoughnessMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneMetallicRoughnessMaterial {}
impl ::core::clone::Clone for SceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneModelTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneModelTransform {}
impl ::core::clone::Clone for SceneModelTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneNode {}
impl ::core::clone::Clone for SceneNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneNodeCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneNodeCollection {}
impl ::core::clone::Clone for SceneNodeCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneObject {}
impl ::core::clone::Clone for SceneObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScenePbrMaterial(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScenePbrMaterial {}
impl ::core::clone::Clone for ScenePbrMaterial {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneRendererComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneRendererComponent {}
impl ::core::clone::Clone for SceneRendererComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneSurfaceMaterialInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneSurfaceMaterialInput {}
impl ::core::clone::Clone for SceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneVisual(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneVisual {}
impl ::core::clone::Clone for SceneVisual {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: Self = Self(0i32);
    pub const MirroredRepeat: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneWrappingMode {}
impl ::core::clone::Clone for SceneWrappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
