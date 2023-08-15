#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneBoundingBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
}
impl ::core::clone::Clone for ISceneBoundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneBoundingBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8ffc70_c618_4083_8251_9962593114aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneBoundingBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Center: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Extents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Extents: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Max: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Min: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponent {
    type Vtable = ISceneComponent_Vtbl;
}
impl ::core::clone::Clone for ISceneComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneComponent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae20fc96_226c_44bd_95cb_dd5ed9ebe9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ComponentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneComponentType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponentCollection {
    type Vtable = ISceneComponentCollection_Vtbl;
}
impl ::core::clone::Clone for ISceneComponentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneComponentCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc483791c_5f46_45e4_b666_a3d2259f9b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponentFactory {
    type Vtable = ISceneComponentFactory_Vtbl;
}
impl ::core::clone::Clone for ISceneComponentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneComponentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fbc5574_ddd8_5889_ab5b_d8fa716e7c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
}
impl ::core::clone::Clone for ISceneMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMaterial {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ca74b7c_30df_4e07_9490_37875af1a123);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterial_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialFactory {
    type Vtable = ISceneMaterialFactory_Vtbl;
}
impl ::core::clone::Clone for ISceneMaterialFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMaterialFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67536c19_a707_5254_a495_7fdc799893b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
}
impl ::core::clone::Clone for ISceneMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMaterialInput {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x422a1642_1ef1_485c_97e9_ae6f95ad812f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInputFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialInputFactory {
    type Vtable = ISceneMaterialInputFactory_Vtbl;
}
impl ::core::clone::Clone for ISceneMaterialInputFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMaterialInputFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa88feb74_7d0a_5e4c_a748_1015af9ca74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMesh(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMesh {
    type Vtable = ISceneMesh_Vtbl;
}
impl ::core::clone::Clone for ISceneMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMesh {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee9a1530_1155_4c0c_92bd_40020cf78347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMesh_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub PrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPrimitiveTopology: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub FillMeshAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    FillMeshAttribute: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshMaterialAttributeMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
}
impl ::core::clone::Clone for ISceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMeshMaterialAttributeMap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce843171_3d43_4855_aa69_31ff988d049d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
}
impl ::core::clone::Clone for ISceneMeshRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMeshRendererComponent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9929f7e3_6364_477e_98fe_74ed9fd4c2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Material: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Mesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UVMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshRendererComponentStatics {
    type Vtable = ISceneMeshRendererComponentStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneMeshRendererComponentStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMeshRendererComponentStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4954f37a_4459_4521_bd6e_2b38b8d711ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshStatics {
    type Vtable = ISceneMeshStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneMeshStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMeshStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8412316c_7b57_473f_966b_81dc277b1751);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
}
impl ::core::clone::Clone for ISceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMetallicRoughnessMaterial {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1d91446_799c_429e_a4e4_5da645f18e61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BaseColorInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBaseColorInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub BaseColorFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BaseColorFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetBaseColorFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetBaseColorFactor: usize,
    pub MetallicFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMetallicFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub MetallicRoughnessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMetallicRoughnessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RoughnessFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRoughnessFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterialStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMetallicRoughnessMaterialStatics {
    type Vtable = ISceneMetallicRoughnessMaterialStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneMetallicRoughnessMaterialStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneMetallicRoughnessMaterialStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bddca50_6d9d_4531_8dc4_b27e3e49b7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneModelTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
}
impl ::core::clone::Clone for ISceneModelTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneModelTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc05576c2_32b1_4269_980d_b98537100ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModelTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub RotationAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRotationAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTranslation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNode {
    type Vtable = ISceneNode_Vtbl;
}
impl ::core::clone::Clone for ISceneNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneNode {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacf2c247_f307_4581_9c41_af2e29c3b016);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindFirstComponentOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneComponentType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNodeCollection {
    type Vtable = ISceneNodeCollection_Vtbl;
}
impl ::core::clone::Clone for ISceneNodeCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneNodeCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29ada101_2dd9_4332_be63_60d2cf4269f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNodeStatics {
    type Vtable = ISceneNodeStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneNodeStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneNodeStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x579a0faa_be9d_4210_908c_93d15feed0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneObject {
    type Vtable = ISceneObject_Vtbl;
}
impl ::core::clone::Clone for ISceneObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e94249b_0f1b_49eb_a819_877d8450005b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObject_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneObjectFactory {
    type Vtable = ISceneObjectFactory_Vtbl;
}
impl ::core::clone::Clone for ISceneObjectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneObjectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14fe799a_33e4_52ef_956c_44229d21f2c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
}
impl ::core::clone::Clone for IScenePbrMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScenePbrMaterial {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaab6ebbe_d680_46df_8294_b6800a9f95e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterial_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlphaCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetAlphaCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneAlphaMode) -> ::windows_core::HRESULT,
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneAlphaMode) -> ::windows_core::HRESULT,
    pub EmissiveInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEmissiveInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub EmissiveFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EmissiveFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEmissiveFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEmissiveFactor: usize,
    pub IsDoubleSided: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDoubleSided: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub NormalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNormalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NormalScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetNormalScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub OcclusionInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOcclusionInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OcclusionStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetOcclusionStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScenePbrMaterialFactory {
    type Vtable = IScenePbrMaterialFactory_Vtbl;
}
impl ::core::clone::Clone for IScenePbrMaterialFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScenePbrMaterialFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e3f3dfe_0b85_5727_b5be_b7d3cbac37fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
}
impl ::core::clone::Clone for ISceneRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneRendererComponent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1acb857_cf4f_4025_9b25_a2d1944cf507);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneRendererComponentFactory {
    type Vtable = ISceneRendererComponentFactory_Vtbl;
}
impl ::core::clone::Clone for ISceneRendererComponentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneRendererComponentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1db6ed6c_aa2c_5967_9035_56352dc69658);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
}
impl ::core::clone::Clone for ISceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneSurfaceMaterialInput {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9937da5c_a9ca_4cfc_b3aa_088356518742);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::CompositionBitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::CompositionBitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WrappingUMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows_core::HRESULT,
    pub SetWrappingUMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows_core::HRESULT,
    pub WrappingVMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows_core::HRESULT,
    pub SetWrappingVMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInputStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneSurfaceMaterialInputStatics {
    type Vtable = ISceneSurfaceMaterialInputStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneSurfaceMaterialInputStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneSurfaceMaterialInputStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a2394d3_6429_4589_bbcf_b84f4f3cfbfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneVisual {
    type Vtable = ISceneVisual_Vtbl;
}
impl ::core::clone::Clone for ISceneVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneVisual {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e672c1e_d734_47b1_be14_3d694ffa4301);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisualStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneVisualStatics {
    type Vtable = ISceneVisualStatics_Vtbl;
}
impl ::core::clone::Clone for ISceneVisualStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneVisualStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8347e9a_50aa_4527_8d34_de4cb8ea88b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisualStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneBoundingBox(::windows_core::IUnknown);
impl SceneBoundingBox {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Center(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Extents(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extents)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Max(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Min(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Size(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneBoundingBox {}
impl ::core::fmt::Debug for SceneBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneBoundingBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneBoundingBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneBoundingBox;{5d8ffc70-c618-4083-8251-9962593114aa})");
}
impl ::core::clone::Clone for SceneBoundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneBoundingBox {
    const IID: ::windows_core::GUID = <ISceneBoundingBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneBoundingBox";
}
::windows_core::imp::interface_hierarchy!(SceneBoundingBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneBoundingBox {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneBoundingBox {}
impl ::windows_core::CanTryInto<SceneObject> for SceneBoundingBox {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneBoundingBox {}
unsafe impl ::core::marker::Send for SceneBoundingBox {}
unsafe impl ::core::marker::Sync for SceneBoundingBox {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneComponent(::windows_core::IUnknown);
impl SceneComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneComponent {}
impl ::core::fmt::Debug for SceneComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneComponent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})");
}
impl ::core::clone::Clone for SceneComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneComponent {
    type Vtable = ISceneComponent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneComponent {
    const IID: ::windows_core::GUID = <ISceneComponent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponent";
}
::windows_core::imp::interface_hierarchy!(SceneComponent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneComponent {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneComponent {}
impl ::windows_core::CanTryInto<SceneObject> for SceneComponent {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneComponent {}
unsafe impl ::core::marker::Send for SceneComponent {}
unsafe impl ::core::marker::Sync for SceneComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SceneComponentCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SceneComponentCollection {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<SceneComponent>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<SceneComponent>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SceneComponent>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<SceneComponent>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneComponent>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneComponent>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneComponent>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<SceneComponent>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<SceneComponent>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SceneComponentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for SceneComponentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for SceneComponentCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SceneComponentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for SceneComponentCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<SceneComponent>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for SceneComponentCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<SceneComponent> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponentCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(SceneComponentCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<SceneComponent>> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVector<SceneComponent>> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<SceneObject> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for SceneComponentCollection {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMaterial(::windows_core::IUnknown);
impl SceneMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterial {}
impl ::core::fmt::Debug for SceneMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterial").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMaterial {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterial;{8ca74b7c-30df-4e07-9490-37875af1a123})");
}
impl ::core::clone::Clone for SceneMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMaterial {
    const IID: ::windows_core::GUID = <ISceneMaterial as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterial";
}
::windows_core::imp::interface_hierarchy!(SceneMaterial, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMaterial {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMaterial {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMaterial {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMaterial {}
unsafe impl ::core::marker::Send for SceneMaterial {}
unsafe impl ::core::marker::Sync for SceneMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMaterialInput(::windows_core::IUnknown);
impl SceneMaterialInput {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterialInput {}
impl ::core::fmt::Debug for SceneMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterialInput").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMaterialInput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterialInput;{422a1642-1ef1-485c-97e9-ae6f95ad812f})");
}
impl ::core::clone::Clone for SceneMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMaterialInput {
    const IID: ::windows_core::GUID = <ISceneMaterialInput as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterialInput";
}
::windows_core::imp::interface_hierarchy!(SceneMaterialInput, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMaterialInput {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMaterialInput {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMaterialInput {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMaterialInput {}
unsafe impl ::core::marker::Send for SceneMaterialInput {}
unsafe impl ::core::marker::Sync for SceneMaterialInput {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMesh(::windows_core::IUnknown);
impl SceneMesh {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Bounds(&self) -> ::windows_core::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PrimitiveTopology(&self) -> ::windows_core::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrimitiveTopology)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPrimitiveTopology(&self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrimitiveTopology)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub fn FillMeshAttribute<P0>(&self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::MemoryBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).FillMeshAttribute)(::windows_core::Interface::as_raw(this), semantic, format, memory.into_param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneMesh>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneMesh, ISceneMeshStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMesh {}
impl ::core::fmt::Debug for SceneMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMesh").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMesh {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMesh;{ee9a1530-1155-4c0c-92bd-40020cf78347})");
}
impl ::core::clone::Clone for SceneMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMesh {
    type Vtable = ISceneMesh_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMesh {
    const IID: ::windows_core::GUID = <ISceneMesh as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMesh {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMesh";
}
::windows_core::imp::interface_hierarchy!(SceneMesh, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMesh {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMesh {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMesh {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMesh {}
unsafe impl ::core::marker::Send for SceneMesh {}
unsafe impl ::core::marker::Sync for SceneMesh {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMeshMaterialAttributeMap(::windows_core::IUnknown);
impl SceneMeshMaterialAttributeMap {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<SceneAttributeSemantic> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, SceneAttributeSemantic>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows_core::HSTRING, value: SceneAttributeSemantic) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneMeshMaterialAttributeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshMaterialAttributeMap {}
impl ::core::fmt::Debug for SceneMeshMaterialAttributeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshMaterialAttributeMap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap;{ce843171-3d43-4855-aa69-31ff988d049d})");
}
impl ::core::clone::Clone for SceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMeshMaterialAttributeMap {
    const IID: ::windows_core::GUID = <ISceneMeshMaterialAttributeMap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(SceneMeshMaterialAttributeMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMeshMaterialAttributeMap {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMeshMaterialAttributeMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> for SceneMeshMaterialAttributeMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>> for SceneMeshMaterialAttributeMap {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMeshMaterialAttributeMap {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMeshMaterialAttributeMap {}
unsafe impl ::core::marker::Send for SceneMeshMaterialAttributeMap {}
unsafe impl ::core::marker::Sync for SceneMeshMaterialAttributeMap {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMeshRendererComponent(::windows_core::IUnknown);
impl SceneMeshRendererComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = &::windows_core::ComInterface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Material(&self) -> ::windows_core::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Material)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaterial<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterial>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaterial)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Mesh(&self) -> ::windows_core::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mesh)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMesh<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SceneMesh>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMesh)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UVMappings(&self) -> ::windows_core::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UVMappings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneMeshRendererComponent>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshRendererComponentStatics<R, F: FnOnce(&ISceneMeshRendererComponentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneMeshRendererComponent, ISceneMeshRendererComponentStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneMeshRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshRendererComponent {}
impl ::core::fmt::Debug for SceneMeshRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshRendererComponent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshRendererComponent;{9929f7e3-6364-477e-98fe-74ed9fd4c2de})");
}
impl ::core::clone::Clone for SceneMeshRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMeshRendererComponent {
    const IID: ::windows_core::GUID = <ISceneMeshRendererComponent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
}
::windows_core::imp::interface_hierarchy!(SceneMeshRendererComponent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMeshRendererComponent {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMeshRendererComponent {}
impl ::windows_core::CanTryInto<SceneRendererComponent> for SceneMeshRendererComponent {}
impl ::windows_core::CanTryInto<SceneComponent> for SceneMeshRendererComponent {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMeshRendererComponent {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMeshRendererComponent {}
unsafe impl ::core::marker::Send for SceneMeshRendererComponent {}
unsafe impl ::core::marker::Sync for SceneMeshRendererComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMetallicRoughnessMaterial(::windows_core::IUnknown);
impl SceneMetallicRoughnessMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn BaseColorInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseColorInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBaseColorInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseColorInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BaseColorFactor(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseColorFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetBaseColorFactor(&self, value: super::super::super::Foundation::Numerics::Vector4) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseColorFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MetallicFactor(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MetallicFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMetallicFactor(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMetallicFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MetallicRoughnessInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MetallicRoughnessInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMetallicRoughnessInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMetallicRoughnessInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn RoughnessFactor(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoughnessFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoughnessFactor(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoughnessFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneMetallicRoughnessMaterial>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn AlphaCutoff(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlphaCutoff)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> ::windows_core::Result<SceneAlphaMode> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmissiveInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn EmissiveFactor(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetEmissiveFactor(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleSided(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleSided)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDoubleSided)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn NormalScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOcclusionInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionStrength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionStrength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ISceneMetallicRoughnessMaterialStatics<R, F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneMetallicRoughnessMaterial, ISceneMetallicRoughnessMaterialStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneMetallicRoughnessMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMetallicRoughnessMaterial {}
impl ::core::fmt::Debug for SceneMetallicRoughnessMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMetallicRoughnessMaterial").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial;{c1d91446-799c-429e-a4e4-5da645f18e61})");
}
impl ::core::clone::Clone for SceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneMetallicRoughnessMaterial {
    const IID: ::windows_core::GUID = <ISceneMetallicRoughnessMaterial as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
::windows_core::imp::interface_hierarchy!(SceneMetallicRoughnessMaterial, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneMetallicRoughnessMaterial {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneMetallicRoughnessMaterial {}
impl ::windows_core::CanTryInto<ScenePbrMaterial> for SceneMetallicRoughnessMaterial {}
impl ::windows_core::CanTryInto<SceneMaterial> for SceneMetallicRoughnessMaterial {}
impl ::windows_core::CanTryInto<SceneObject> for SceneMetallicRoughnessMaterial {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneMetallicRoughnessMaterial {}
unsafe impl ::core::marker::Send for SceneMetallicRoughnessMaterial {}
unsafe impl ::core::marker::Sync for SceneMetallicRoughnessMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneModelTransform(::windows_core::IUnknown);
impl SceneModelTransform {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegrees)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RotationAxis(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAxis)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRotationAxis(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAxis)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Scale(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetScale(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Translation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Translation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTranslation(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneModelTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneModelTransform {}
impl ::core::fmt::Debug for SceneModelTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneModelTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneModelTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneModelTransform;{c05576c2-32b1-4269-980d-b98537100ae4})");
}
impl ::core::clone::Clone for SceneModelTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneModelTransform {
    const IID: ::windows_core::GUID = <ISceneModelTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneModelTransform";
}
::windows_core::imp::interface_hierarchy!(SceneModelTransform, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneModelTransform {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneModelTransform {}
impl ::windows_core::CanTryInto<super::CompositionTransform> for SceneModelTransform {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneModelTransform {}
unsafe impl ::core::marker::Send for SceneModelTransform {}
unsafe impl ::core::marker::Sync for SceneModelTransform {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneNode(::windows_core::IUnknown);
impl SceneNode {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows_core::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> ::windows_core::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Components)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FindFirstComponentOfType(&self, value: SceneComponentType) -> ::windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindFirstComponentOfType)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneNode>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneNode, ISceneNodeStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneNode {}
impl ::core::fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneNode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})");
}
impl ::core::clone::Clone for SceneNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneNode {
    type Vtable = ISceneNode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneNode {
    const IID: ::windows_core::GUID = <ISceneNode as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneNode {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNode";
}
::windows_core::imp::interface_hierarchy!(SceneNode, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneNode {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneNode {}
impl ::windows_core::CanTryInto<SceneObject> for SceneNode {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneNode {}
unsafe impl ::core::marker::Send for SceneNode {}
unsafe impl ::core::marker::Sync for SceneNode {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SceneNodeCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SceneNodeCollection {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<SceneNode>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<SceneNode>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SceneNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<SceneNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SceneNode>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SceneNode>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SceneNode>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<SceneNode>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<SceneNode>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SceneNodeCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for SceneNodeCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNodeCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for SceneNodeCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNodeCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SceneNodeCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for SceneNodeCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_Vtbl<SceneNode>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for SceneNodeCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVector<SceneNode> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNodeCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(SceneNodeCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<SceneNode>> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVector<SceneNode>> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<SceneObject> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for SceneNodeCollection {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneObject(::windows_core::IUnknown);
impl SceneObject {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneObject {}
impl ::core::fmt::Debug for SceneObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneObject").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneObject {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneObject;{1e94249b-0f1b-49eb-a819-877d8450005b})");
}
impl ::core::clone::Clone for SceneObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneObject {
    type Vtable = ISceneObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneObject {
    const IID: ::windows_core::GUID = <ISceneObject as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneObject {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneObject";
}
::windows_core::imp::interface_hierarchy!(SceneObject, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneObject {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneObject {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneObject {}
unsafe impl ::core::marker::Send for SceneObject {}
unsafe impl ::core::marker::Sync for SceneObject {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct ScenePbrMaterial(::windows_core::IUnknown);
impl ScenePbrMaterial {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn AlphaCutoff(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlphaCutoff)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> ::windows_core::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmissiveInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn EmissiveFactor(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetEmissiveFactor(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleSided(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleSided)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDoubleSided)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn NormalScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOcclusionInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<SceneMaterialInput>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionStrength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionStrength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ScenePbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScenePbrMaterial {}
impl ::core::fmt::Debug for ScenePbrMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScenePbrMaterial").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScenePbrMaterial {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.ScenePbrMaterial;{aab6ebbe-d680-46df-8294-b6800a9f95e7})");
}
impl ::core::clone::Clone for ScenePbrMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScenePbrMaterial {
    const IID: ::windows_core::GUID = <IScenePbrMaterial as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ScenePbrMaterial";
}
::windows_core::imp::interface_hierarchy!(ScenePbrMaterial, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for ScenePbrMaterial {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for ScenePbrMaterial {}
impl ::windows_core::CanTryInto<SceneMaterial> for ScenePbrMaterial {}
impl ::windows_core::CanTryInto<SceneObject> for ScenePbrMaterial {}
impl ::windows_core::CanTryInto<super::CompositionObject> for ScenePbrMaterial {}
unsafe impl ::core::marker::Send for ScenePbrMaterial {}
unsafe impl ::core::marker::Sync for ScenePbrMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneRendererComponent(::windows_core::IUnknown);
impl SceneRendererComponent {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = &::windows_core::ComInterface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneRendererComponent {}
impl ::core::fmt::Debug for SceneRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneRendererComponent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneRendererComponent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneRendererComponent;{f1acb857-cf4f-4025-9b25-a2d1944cf507})");
}
impl ::core::clone::Clone for SceneRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneRendererComponent {
    const IID: ::windows_core::GUID = <ISceneRendererComponent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneRendererComponent";
}
::windows_core::imp::interface_hierarchy!(SceneRendererComponent, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneRendererComponent {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneRendererComponent {}
impl ::windows_core::CanTryInto<SceneComponent> for SceneRendererComponent {}
impl ::windows_core::CanTryInto<SceneObject> for SceneRendererComponent {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneRendererComponent {}
unsafe impl ::core::marker::Send for SceneRendererComponent {}
unsafe impl ::core::marker::Sync for SceneRendererComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneSurfaceMaterialInput(::windows_core::IUnknown);
impl SceneSurfaceMaterialInput {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn BitmapInterpolationMode(&self) -> ::windows_core::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapInterpolationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Surface(&self) -> ::windows_core::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Surface)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSurface<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSurface>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSurface)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn WrappingUMode(&self) -> ::windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WrappingUMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWrappingUMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WrappingVMode(&self) -> ::windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WrappingVMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWrappingVMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneSurfaceMaterialInput>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneSurfaceMaterialInputStatics<R, F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneSurfaceMaterialInput, ISceneSurfaceMaterialInputStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneSurfaceMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneSurfaceMaterialInput {}
impl ::core::fmt::Debug for SceneSurfaceMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneSurfaceMaterialInput").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput;{9937da5c-a9ca-4cfc-b3aa-088356518742})");
}
impl ::core::clone::Clone for SceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneSurfaceMaterialInput {
    const IID: ::windows_core::GUID = <ISceneSurfaceMaterialInput as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
::windows_core::imp::interface_hierarchy!(SceneSurfaceMaterialInput, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneSurfaceMaterialInput {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneSurfaceMaterialInput {}
impl ::windows_core::CanTryInto<SceneMaterialInput> for SceneSurfaceMaterialInput {}
impl ::windows_core::CanTryInto<SceneObject> for SceneSurfaceMaterialInput {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneSurfaceMaterialInput {}
unsafe impl ::core::marker::Send for SceneSurfaceMaterialInput {}
unsafe impl ::core::marker::Sync for SceneSurfaceMaterialInput {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneVisual(::windows_core::IUnknown);
impl SceneVisual {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationWithController)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi()).ok() }
    }
    pub fn Children(&self) -> ::windows_core::Result<super::VisualCollection> {
        let this = &::windows_core::ComInterface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Root(&self) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Root)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SceneNode>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> ::windows_core::Result<SceneVisual>
    where
        P0: ::windows_core::IntoParam<super::Compositor>,
    {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AnchorPoint(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnchorPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetAnchorPoint(&self, value: super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAnchorPoint)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackfaceVisibility(&self) -> ::windows_core::Result<super::CompositionBackfaceVisibility> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackfaceVisibility)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBackfaceVisibility(&self, value: super::CompositionBackfaceVisibility) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBackfaceVisibility)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderMode(&self) -> ::windows_core::Result<super::CompositionBorderMode> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BorderMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBorderMode(&self, value: super::CompositionBorderMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBorderMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CenterPoint(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetCenterPoint(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterPoint)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Clip(&self) -> ::windows_core::Result<super::CompositionClip> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clip)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetClip<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionClip>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetClip)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn CompositeMode(&self) -> ::windows_core::Result<super::CompositionCompositeMode> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositeMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompositeMode(&self, value: super::CompositionCompositeMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompositeMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Offset(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOffset(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opacity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Parent(&self) -> ::windows_core::Result<super::ContainerVisual> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegrees)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RotationAxis(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAxis)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRotationAxis(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAxis)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Scale(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetScale(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Size(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetSize(&self, value: super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TransformMatrix(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformMatrix)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransformMatrix(&self, value: super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransformMatrix)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ParentForTransform(&self) -> ::windows_core::Result<super::Visual> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentForTransform)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetParentForTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Visual>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParentForTransform)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOffsetAdjustment(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeOffsetAdjustment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOffsetAdjustment(&self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeOffsetAdjustment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeSizeAdjustment(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeSizeAdjustment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeSizeAdjustment(&self, value: super::super::super::Foundation::Numerics::Vector2) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeSizeAdjustment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHitTestVisible)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHitTestVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPixelSnappingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPixelSnappingEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IVisual4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPixelSnappingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ISceneVisualStatics<R, F: FnOnce(&ISceneVisualStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneVisual, ISceneVisualStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SceneVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneVisual {}
impl ::core::fmt::Debug for SceneVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneVisual").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneVisual {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneVisual;{8e672c1e-d734-47b1-be14-3d694ffa4301})");
}
impl ::core::clone::Clone for SceneVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneVisual {
    type Vtable = ISceneVisual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneVisual {
    const IID: ::windows_core::GUID = <ISceneVisual as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneVisual {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneVisual";
}
::windows_core::imp::interface_hierarchy!(SceneVisual, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for SceneVisual {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for SceneVisual {}
impl ::windows_core::CanTryInto<super::ContainerVisual> for SceneVisual {}
impl ::windows_core::CanTryInto<super::Visual> for SceneVisual {}
impl ::windows_core::CanTryInto<super::CompositionObject> for SceneVisual {}
unsafe impl ::core::marker::Send for SceneVisual {}
unsafe impl ::core::marker::Sync for SceneVisual {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SceneAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneAlphaMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAlphaMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAlphaMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAlphaMode;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SceneAttributeSemantic {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneAttributeSemantic {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneAttributeSemantic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAttributeSemantic").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAttributeSemantic;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SceneComponentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneComponentType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneComponentType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneComponentType;i4)");
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SceneWrappingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneWrappingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneWrappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneWrappingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneWrappingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneWrappingMode;i4)");
}
