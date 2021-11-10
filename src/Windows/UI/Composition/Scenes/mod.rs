#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneBoundingBox(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneBoundingBox {
    type Vtable = ISceneBoundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d8ffc70_c618_4083_8251_9962593114aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneBoundingBox_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponent {
    type Vtable = ISceneComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xae20fc96_226c_44bd_95cb_dd5ed9ebe9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SceneComponentType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneComponentCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponentCollection {
    type Vtable = ISceneComponentCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc483791c_5f46_45e4_b666_a3d2259f9b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneComponentFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponentFactory {
    type Vtable = ISceneComponentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5fbc5574_ddd8_5889_ab5b_d8fa716e7c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterial {
    type Vtable = ISceneMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ca74b7c_30df_4e07_9490_37875af1a123);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMaterialFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialFactory {
    type Vtable = ISceneMaterialFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x67536c19_a707_5254_a495_7fdc799893b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMaterialInput(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialInput {
    type Vtable = ISceneMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x422a1642_1ef1_485c_97e9_ae6f95ad812f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInput_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialInputFactory {
    type Vtable = ISceneMaterialInputFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa88feb74_7d0a_5e4c_a748_1015af9ca74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMesh(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMesh {
    type Vtable = ISceneMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xee9a1530_1155_4c0c_92bd_40020cf78347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMesh_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce843171_3d43_4855_aa69_31ff988d049d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9929f7e3_6364_477e_98fe_74ed9fd4c2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshRendererComponentStatics {
    type Vtable = ISceneMeshRendererComponentStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4954f37a_4459_4521_bd6e_2b38b8d711ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshStatics {
    type Vtable = ISceneMeshStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8412316c_7b57_473f_966b_81dc277b1751);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1d91446_799c_429e_a4e4_5da645f18e61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector4) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector4) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMetallicRoughnessMaterialStatics {
    type Vtable = ISceneMetallicRoughnessMaterialStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3bddca50_6d9d_4531_8dc4_b27e3e49b7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneModelTransform(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneModelTransform {
    type Vtable = ISceneModelTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc05576c2_32b1_4269_980d_b98537100ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModelTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneNode(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNode {
    type Vtable = ISceneNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xacf2c247_f307_4581_9c41_af2e29c3b016);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SceneComponentType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneNodeCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNodeCollection {
    type Vtable = ISceneNodeCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29ada101_2dd9_4332_be63_60d2cf4269f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneNodeStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNodeStatics {
    type Vtable = ISceneNodeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x579a0faa_be9d_4210_908c_93d15feed0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneObject(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneObject {
    type Vtable = ISceneObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e94249b_0f1b_49eb_a819_877d8450005b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneObjectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneObjectFactory {
    type Vtable = ISceneObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x14fe799a_33e4_52ef_956c_44229d21f2c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObjectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScenePbrMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScenePbrMaterial {
    type Vtable = IScenePbrMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaab6ebbe_d680_46df_8294_b6800a9f95e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SceneAlphaMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SceneAlphaMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScenePbrMaterialFactory {
    type Vtable = IScenePbrMaterialFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e3f3dfe_0b85_5727_b5be_b7d3cbac37fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneRendererComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneRendererComponent {
    type Vtable = ISceneRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1acb857_cf4f_4025_9b25_a2d1944cf507);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneRendererComponentFactory {
    type Vtable = ISceneRendererComponentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1db6ed6c_aa2c_5967_9035_56352dc69658);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9937da5c_a9ca_4cfc_b3aa_088356518742);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::CompositionBitmapInterpolationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::CompositionBitmapInterpolationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SceneWrappingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SceneWrappingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SceneWrappingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SceneWrappingMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneSurfaceMaterialInputStatics {
    type Vtable = ISceneSurfaceMaterialInputStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5a2394d3_6429_4589_bbcf_b84f4f3cfbfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneVisual(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneVisual {
    type Vtable = ISceneVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e672c1e_d734_47b1_be14_3d694ffa4301);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisual_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneVisualStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneVisualStatics {
    type Vtable = ISceneVisualStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb8347e9a_50aa_4527_8d34_de4cb8ea88b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisualStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: SceneAlphaMode = SceneAlphaMode(0i32);
    pub const AlphaTest: SceneAlphaMode = SceneAlphaMode(1i32);
    pub const Blend: SceneAlphaMode = SceneAlphaMode(2i32);
}
impl ::core::convert::From<i32> for SceneAlphaMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneAlphaMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneAlphaMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAlphaMode;i4)");
}
impl ::windows::runtime::DefaultType for SceneAlphaMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for SceneAttributeSemantic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneAttributeSemantic {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAttributeSemantic;i4)");
}
impl ::windows::runtime::DefaultType for SceneAttributeSemantic {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneBoundingBox(pub ::windows::runtime::IInspectable);
impl SceneBoundingBox {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Center(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Extents(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Max(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Min(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneBoundingBox {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneBoundingBox;{5d8ffc70-c618-4083-8251-9962593114aa})");
}
unsafe impl ::windows::runtime::Interface for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d8ffc70_c618_4083_8251_9962593114aa);
}
impl ::windows::runtime::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneBoundingBox";
}
impl ::core::convert::From<SceneBoundingBox> for ::windows::runtime::IUnknown {
    fn from(value: SceneBoundingBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneBoundingBox> for ::windows::runtime::IUnknown {
    fn from(value: &SceneBoundingBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneBoundingBox> for ::windows::runtime::IInspectable {
    fn from(value: SceneBoundingBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneBoundingBox> for ::windows::runtime::IInspectable {
    fn from(value: &SceneBoundingBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneBoundingBox> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneBoundingBox> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneBoundingBox> for SceneObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for SceneObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneBoundingBox> for super::CompositionObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for super::CompositionObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneBoundingBox {}
unsafe impl ::core::marker::Sync for SceneBoundingBox {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneComponent(pub ::windows::runtime::IInspectable);
impl SceneComponent {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__: SceneComponentType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneComponentType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})");
}
unsafe impl ::windows::runtime::Interface for SceneComponent {
    type Vtable = ISceneComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xae20fc96_226c_44bd_95cb_dd5ed9ebe9a5);
}
impl ::windows::runtime::RuntimeName for SceneComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponent";
}
impl ::core::convert::From<SceneComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneComponent> for SceneObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for SceneObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneComponent> for super::CompositionObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for super::CompositionObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneComponent {}
unsafe impl ::core::marker::Sync for SceneComponent {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneComponentCollection(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl SceneComponentCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<SceneComponent>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SceneComponent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SceneComponent>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, SceneComponent>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneComponent>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneComponent>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, SceneComponent>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<SceneComponent as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<SceneComponent as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<SceneComponent>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<SceneComponent>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<SceneComponent>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for SceneComponentCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for SceneComponentCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<SceneComponent>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<SceneComponent> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponentCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneComponentCollection> for ::windows::runtime::IUnknown {
    fn from(value: SceneComponentCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneComponentCollection> for ::windows::runtime::IUnknown {
    fn from(value: &SceneComponentCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneComponentCollection> for ::windows::runtime::IInspectable {
    fn from(value: SceneComponentCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneComponentCollection> for ::windows::runtime::IInspectable {
    fn from(value: &SceneComponentCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneComponentCollection> for super::super::super::Foundation::Collections::IVector<SceneComponent> {
    fn from(value: SceneComponentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneComponentCollection> for super::super::super::Foundation::Collections::IVector<SceneComponent> {
    fn from(value: &SceneComponentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<SceneComponent>> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<SceneComponent>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<SceneComponent>> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<SceneComponent>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneComponentCollection> for super::super::super::Foundation::Collections::IIterable<SceneComponent> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for super::super::super::Foundation::Collections::IIterable<SceneComponent> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<SceneComponent>> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<SceneComponent>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<SceneComponent>> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<SceneComponent>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<SceneComponent>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::core::convert::TryFrom<SceneComponentCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::core::convert::TryFrom<&SceneComponentCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneComponentCollection> for SceneObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneComponentCollection> for SceneObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneComponentCollection> for super::CompositionObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneComponentCollection> for super::CompositionObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for SceneComponentCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for SceneComponentCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: SceneComponentType = SceneComponentType(0i32);
}
impl ::core::convert::From<i32> for SceneComponentType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneComponentType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneComponentType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneComponentType;i4)");
}
impl ::windows::runtime::DefaultType for SceneComponentType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMaterial(pub ::windows::runtime::IInspectable);
impl SceneMaterial {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterial;{8ca74b7c-30df-4e07-9490-37875af1a123})");
}
unsafe impl ::windows::runtime::Interface for SceneMaterial {
    type Vtable = ISceneMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ca74b7c_30df_4e07_9490_37875af1a123);
}
impl ::windows::runtime::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterial";
}
impl ::core::convert::From<SceneMaterial> for ::windows::runtime::IUnknown {
    fn from(value: SceneMaterial) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMaterial> for ::windows::runtime::IInspectable {
    fn from(value: SceneMaterial) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMaterial> for SceneObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for SceneObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMaterial> for super::CompositionObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for super::CompositionObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMaterial {}
unsafe impl ::core::marker::Sync for SceneMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMaterialInput(pub ::windows::runtime::IInspectable);
impl SceneMaterialInput {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMaterialInput {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterialInput;{422a1642-1ef1-485c-97e9-ae6f95ad812f})");
}
unsafe impl ::windows::runtime::Interface for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x422a1642_1ef1_485c_97e9_ae6f95ad812f);
}
impl ::windows::runtime::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterialInput";
}
impl ::core::convert::From<SceneMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: SceneMaterialInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMaterialInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: SceneMaterialInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMaterialInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMaterialInput> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMaterialInput> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMaterialInput> for SceneObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for SceneObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMaterialInput> for super::CompositionObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for super::CompositionObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMaterialInput {}
unsafe impl ::core::marker::Sync for SceneMaterialInput {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMesh(pub ::windows::runtime::IInspectable);
impl SceneMesh {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Bounds(&self) -> ::windows::runtime::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneBoundingBox>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Graphics_DirectX`*"]
    pub fn PrimitiveTopology(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Graphics_DirectX`*"]
    pub fn SetPrimitiveTopology(&self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`, `Graphics_DirectX`*"]
    pub fn FillMeshAttribute<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::MemoryBuffer>>(&self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), semantic, format, memory.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneMesh> {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneMesh>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneMesh, ISceneMeshStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMesh {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMesh;{ee9a1530-1155-4c0c-92bd-40020cf78347})");
}
unsafe impl ::windows::runtime::Interface for SceneMesh {
    type Vtable = ISceneMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xee9a1530_1155_4c0c_92bd_40020cf78347);
}
impl ::windows::runtime::RuntimeName for SceneMesh {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMesh";
}
impl ::core::convert::From<SceneMesh> for ::windows::runtime::IUnknown {
    fn from(value: SceneMesh) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMesh> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMesh) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMesh> for ::windows::runtime::IInspectable {
    fn from(value: SceneMesh) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMesh> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMesh) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMesh> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMesh) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMesh> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMesh) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMesh> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMesh) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMesh> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMesh) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMesh> for SceneObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for SceneObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMesh> for super::CompositionObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for super::CompositionObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMesh {}
unsafe impl ::core::marker::Sync for SceneMesh {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMeshMaterialAttributeMap(pub ::windows::runtime::IInspectable);
impl SceneMeshMaterialAttributeMap {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<SceneAttributeSemantic> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__: SceneAttributeSemantic = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<SceneAttributeSemantic>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, SceneAttributeSemantic>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, SceneAttributeSemantic>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, value: SceneAttributeSemantic) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap;{ce843171-3d43-4855-aa69-31ff988d049d})");
}
unsafe impl ::windows::runtime::Interface for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xce843171_3d43_4855_aa69_31ff988d049d);
}
impl ::windows::runtime::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for ::windows::runtime::IUnknown {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for ::windows::runtime::IInspectable {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, SceneAttributeSemantic>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMeshMaterialAttributeMap {}
unsafe impl ::core::marker::Sync for SceneMeshMaterialAttributeMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, SceneAttributeSemantic>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMeshRendererComponent(pub ::windows::runtime::IInspectable);
impl SceneMeshRendererComponent {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Material(&self) -> ::windows::runtime::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterial>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMaterial<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterial>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Mesh(&self) -> ::windows::runtime::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMesh>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMesh<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMesh>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn UVMappings(&self) -> ::windows::runtime::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMeshMaterialAttributeMap>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneMeshRendererComponent> {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneMeshRendererComponent>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = &::windows::runtime::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__: SceneComponentType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneComponentType>(result__)
        }
    }
    pub fn ISceneMeshRendererComponentStatics<R, F: FnOnce(&ISceneMeshRendererComponentStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneMeshRendererComponent, ISceneMeshRendererComponentStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshRendererComponent;{9929f7e3-6364-477e-98fe-74ed9fd4c2de})");
}
unsafe impl ::windows::runtime::Interface for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9929f7e3_6364_477e_98fe_74ed9fd4c2de);
}
impl ::windows::runtime::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
}
impl ::core::convert::From<SceneMeshRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneMeshRendererComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneMeshRendererComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::Into::<SceneRendererComponent>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneRendererComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneRendererComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneRendererComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneRendererComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneRendererComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneRendererComponent>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::Into::<SceneComponent>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneComponent>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMeshRendererComponent {}
unsafe impl ::core::marker::Sync for SceneMeshRendererComponent {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneMetallicRoughnessMaterial(pub ::windows::runtime::IInspectable);
impl SceneMetallicRoughnessMaterial {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BaseColorInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBaseColorInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn BaseColorFactor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector4 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector4>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetBaseColorFactor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector4>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn MetallicFactor(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMetallicFactor(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn MetallicRoughnessInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMetallicRoughnessInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RoughnessFactor(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRoughnessFactor(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneMetallicRoughnessMaterial> {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneMetallicRoughnessMaterial>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaCutoff(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaMode(&self) -> ::windows::runtime::Result<SceneAlphaMode> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: SceneAlphaMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn EmissiveFactor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetEmissiveFactor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsDoubleSided(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalScale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionStrength(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ISceneMetallicRoughnessMaterialStatics<R, F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneMetallicRoughnessMaterial, ISceneMetallicRoughnessMaterialStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial;{c1d91446-799c-429e-a4e4-5da645f18e61})");
}
unsafe impl ::windows::runtime::Interface for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc1d91446_799c_429e_a4e4_5da645f18e61);
}
impl ::windows::runtime::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ::windows::runtime::IUnknown {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ::windows::runtime::IInspectable {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::Into::<ScenePbrMaterial>::into(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ScenePbrMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ScenePbrMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<ScenePbrMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ScenePbrMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ScenePbrMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<ScenePbrMaterial>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::Into::<SceneMaterial>::into(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterial>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneMetallicRoughnessMaterial {}
unsafe impl ::core::marker::Sync for SceneMetallicRoughnessMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneModelTransform(pub ::windows::runtime::IInspectable);
impl SceneModelTransform {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetOrientation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngleInDegrees(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn RotationAxis(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetRotationAxis<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetScale<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Translation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetTranslation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneModelTransform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneModelTransform;{c05576c2-32b1-4269-980d-b98537100ae4})");
}
unsafe impl ::windows::runtime::Interface for SceneModelTransform {
    type Vtable = ISceneModelTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc05576c2_32b1_4269_980d_b98537100ae4);
}
impl ::windows::runtime::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneModelTransform";
}
impl ::core::convert::From<SceneModelTransform> for ::windows::runtime::IUnknown {
    fn from(value: SceneModelTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneModelTransform> for ::windows::runtime::IUnknown {
    fn from(value: &SceneModelTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneModelTransform> for ::windows::runtime::IInspectable {
    fn from(value: SceneModelTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneModelTransform> for ::windows::runtime::IInspectable {
    fn from(value: &SceneModelTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneModelTransform> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneModelTransform> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionTransform {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::Into::<super::CompositionTransform>::into(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionTransform {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionTransform> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionTransform> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionTransform> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionTransform> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionTransform>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionObject {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionObject {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneModelTransform {}
unsafe impl ::core::marker::Sync for SceneModelTransform {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneNode(pub ::windows::runtime::IInspectable);
impl SceneNode {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Children(&self) -> ::windows::runtime::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneNodeCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Components(&self) -> ::windows::runtime::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneComponentCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneNode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Transform(&self) -> ::windows::runtime::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneModelTransform>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn FindFirstComponentOfType(&self, value: SceneComponentType) -> ::windows::runtime::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<SceneComponent>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneNode> {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneNode>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneNode, ISceneNodeStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})");
}
unsafe impl ::windows::runtime::Interface for SceneNode {
    type Vtable = ISceneNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xacf2c247_f307_4581_9c41_af2e29c3b016);
}
impl ::windows::runtime::RuntimeName for SceneNode {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNode";
}
impl ::core::convert::From<SceneNode> for ::windows::runtime::IUnknown {
    fn from(value: SceneNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneNode> for ::windows::runtime::IUnknown {
    fn from(value: &SceneNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneNode> for ::windows::runtime::IInspectable {
    fn from(value: SceneNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneNode> for ::windows::runtime::IInspectable {
    fn from(value: &SceneNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneNode> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNode) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneNode> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneNode> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNode) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNode> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneNode> for SceneObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneNode> for SceneObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneNode> for super::CompositionObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneNode> for super::CompositionObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneNode {}
unsafe impl ::core::marker::Sync for SceneNode {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneNodeCollection(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl SceneNodeCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<SceneNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<SceneNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<SceneNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneNode>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneNode>>(&self, index: u32, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<SceneNode as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn ReplaceAll(&self, items: &[<SceneNode as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<SceneNode>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<SceneNode>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<SceneNode>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for SceneNodeCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNodeCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for SceneNodeCollection {
    type Vtable = super::super::super::Foundation::Collections::IVector_abi<SceneNode>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVector<SceneNode> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNodeCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneNodeCollection> for ::windows::runtime::IUnknown {
    fn from(value: SceneNodeCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneNodeCollection> for ::windows::runtime::IUnknown {
    fn from(value: &SceneNodeCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneNodeCollection> for ::windows::runtime::IInspectable {
    fn from(value: SceneNodeCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneNodeCollection> for ::windows::runtime::IInspectable {
    fn from(value: &SceneNodeCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneNodeCollection> for super::super::super::Foundation::Collections::IVector<SceneNode> {
    fn from(value: SceneNodeCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneNodeCollection> for super::super::super::Foundation::Collections::IVector<SceneNode> {
    fn from(value: &SceneNodeCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<SceneNode>> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<SceneNode>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<SceneNode>> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVector<SceneNode>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneNodeCollection> for super::super::super::Foundation::Collections::IIterable<SceneNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for super::super::super::Foundation::Collections::IIterable<SceneNode> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<SceneNode>> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<SceneNode>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<SceneNode>> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<SceneNode>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<SceneNode>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::core::convert::TryFrom<SceneNodeCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::core::convert::TryFrom<&SceneNodeCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneNodeCollection> for SceneObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneNodeCollection> for SceneObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SceneNodeCollection> for super::CompositionObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SceneNodeCollection> for super::CompositionObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for SceneNodeCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for SceneNodeCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneObject(pub ::windows::runtime::IInspectable);
impl SceneObject {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneObject;{1e94249b-0f1b-49eb-a819-877d8450005b})");
}
unsafe impl ::windows::runtime::Interface for SceneObject {
    type Vtable = ISceneObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e94249b_0f1b_49eb_a819_877d8450005b);
}
impl ::windows::runtime::RuntimeName for SceneObject {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneObject";
}
impl ::core::convert::From<SceneObject> for ::windows::runtime::IUnknown {
    fn from(value: SceneObject) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneObject> for ::windows::runtime::IUnknown {
    fn from(value: &SceneObject) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneObject> for ::windows::runtime::IInspectable {
    fn from(value: SceneObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneObject> for ::windows::runtime::IInspectable {
    fn from(value: &SceneObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneObject> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneObject) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneObject> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneObject) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneObject> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneObject) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneObject> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneObject) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneObject> for super::CompositionObject {
    fn from(value: SceneObject) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneObject> for super::CompositionObject {
    fn from(value: &SceneObject) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneObject {}
unsafe impl ::core::marker::Sync for SceneObject {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScenePbrMaterial(pub ::windows::runtime::IInspectable);
impl ScenePbrMaterial {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaCutoff(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaMode(&self) -> ::windows::runtime::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: SceneAlphaMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn EmissiveFactor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetEmissiveFactor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsDoubleSided(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionStrength(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScenePbrMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.ScenePbrMaterial;{aab6ebbe-d680-46df-8294-b6800a9f95e7})");
}
unsafe impl ::windows::runtime::Interface for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaab6ebbe_d680_46df_8294_b6800a9f95e7);
}
impl ::windows::runtime::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ScenePbrMaterial";
}
impl ::core::convert::From<ScenePbrMaterial> for ::windows::runtime::IUnknown {
    fn from(value: ScenePbrMaterial) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &ScenePbrMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScenePbrMaterial> for ::windows::runtime::IInspectable {
    fn from(value: ScenePbrMaterial) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &ScenePbrMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ScenePbrMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ScenePbrMaterial> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneMaterial {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::Into::<SceneMaterial>::into(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneMaterial {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterial>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<ScenePbrMaterial> for super::CompositionObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for super::CompositionObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ScenePbrMaterial {}
unsafe impl ::core::marker::Sync for ScenePbrMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneRendererComponent(pub ::windows::runtime::IInspectable);
impl SceneRendererComponent {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = &::windows::runtime::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__: SceneComponentType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneComponentType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneRendererComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneRendererComponent;{f1acb857-cf4f-4025-9b25-a2d1944cf507})");
}
unsafe impl ::windows::runtime::Interface for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1acb857_cf4f_4025_9b25_a2d1944cf507);
}
impl ::windows::runtime::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneRendererComponent";
}
impl ::core::convert::From<SceneRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneRendererComponent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneRendererComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneRendererComponent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneRendererComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneRendererComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneRendererComponent> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneComponent {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::Into::<SceneComponent>::into(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneComponent {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneComponent>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneRendererComponent> for super::CompositionObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for super::CompositionObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneRendererComponent {}
unsafe impl ::core::marker::Sync for SceneRendererComponent {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneSurfaceMaterialInput(pub ::windows::runtime::IInspectable);
impl SceneSurfaceMaterialInput {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BitmapInterpolationMode(&self) -> ::windows::runtime::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__: super::CompositionBitmapInterpolationMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionBitmapInterpolationMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Surface(&self) -> ::windows::runtime::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ICompositionSurface>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionSurface>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn WrappingUMode(&self) -> ::windows::runtime::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__: SceneWrappingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneWrappingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn WrappingVMode(&self) -> ::windows::runtime::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__: SceneWrappingMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneWrappingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneSurfaceMaterialInput> {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneSurfaceMaterialInput>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn ISceneSurfaceMaterialInputStatics<R, F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneSurfaceMaterialInput, ISceneSurfaceMaterialInputStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput;{9937da5c-a9ca-4cfc-b3aa-088356518742})");
}
unsafe impl ::windows::runtime::Interface for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9937da5c_a9ca_4cfc_b3aa_088356518742);
}
impl ::windows::runtime::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::Into::<SceneMaterialInput>::into(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterialInput> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterialInput> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterialInput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterialInput> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterialInput> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneMaterialInput>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<SceneObject>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneSurfaceMaterialInput {}
unsafe impl ::core::marker::Sync for SceneSurfaceMaterialInput {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SceneVisual(pub ::windows::runtime::IInspectable);
impl SceneVisual {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Root(&self) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneNode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRoot<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<SceneVisual> {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<SceneVisual>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Children(&self) -> ::windows::runtime::Result<super::VisualCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VisualCollection>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn AnchorPoint(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetAnchorPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BackfaceVisibility(&self) -> ::windows::runtime::Result<super::CompositionBackfaceVisibility> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionBackfaceVisibility = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionBackfaceVisibility>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBackfaceVisibility(&self, value: super::CompositionBackfaceVisibility) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BorderMode(&self) -> ::windows::runtime::Result<super::CompositionBorderMode> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionBorderMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionBorderMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBorderMode(&self, value: super::CompositionBorderMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn CenterPoint(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetCenterPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Clip(&self) -> ::windows::runtime::Result<super::CompositionClip> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionClip>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, super::CompositionClip>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn CompositeMode(&self) -> ::windows::runtime::Result<super::CompositionCompositeMode> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionCompositeMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionCompositeMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetCompositeMode(&self, value: super::CompositionCompositeMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Offset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetOffset<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetOrientation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<super::ContainerVisual> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ContainerVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngleInDegrees(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn RotationAxis(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetRotationAxis<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetScale<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn TransformMatrix(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetTransformMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix4x4>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ParentForTransform(&self) -> ::windows::runtime::Result<super::Visual> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Visual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetParentForTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn RelativeOffsetAdjustment(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetRelativeOffsetAdjustment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn RelativeSizeAdjustment(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Foundation_Numerics`*"]
    pub fn SetRelativeSizeAdjustment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `System`*"]
    pub fn DispatcherQueue(&self) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertyname: Param0) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsHitTestVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsPixelSnappingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ISceneVisualStatics<R, F: FnOnce(&ISceneVisualStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneVisual, ISceneVisualStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneVisual {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneVisual;{8e672c1e-d734-47b1-be14-3d694ffa4301})");
}
unsafe impl ::windows::runtime::Interface for SceneVisual {
    type Vtable = ISceneVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e672c1e_d734_47b1_be14_3d694ffa4301);
}
impl ::windows::runtime::RuntimeName for SceneVisual {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneVisual";
}
impl ::core::convert::From<SceneVisual> for ::windows::runtime::IUnknown {
    fn from(value: SceneVisual) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneVisual> for ::windows::runtime::IUnknown {
    fn from(value: &SceneVisual) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneVisual> for ::windows::runtime::IInspectable {
    fn from(value: SceneVisual) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneVisual> for ::windows::runtime::IInspectable {
    fn from(value: &SceneVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SceneVisual> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneVisual) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SceneVisual> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneVisual) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneVisual> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneVisual) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneVisual> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneVisual) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<SceneVisual> for super::ContainerVisual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::Into::<super::ContainerVisual>::into(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::ContainerVisual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ContainerVisual> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ContainerVisual> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ContainerVisual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ContainerVisual> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ContainerVisual> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::ContainerVisual>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneVisual> for super::Visual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::Into::<super::Visual>::into(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::Visual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::Visual> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Visual> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::Visual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::Visual> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Visual> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::Visual>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<SceneVisual> for super::CompositionObject {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::CompositionObject {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for SceneVisual {}
unsafe impl ::core::marker::Sync for SceneVisual {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: SceneWrappingMode = SceneWrappingMode(0i32);
    pub const MirroredRepeat: SceneWrappingMode = SceneWrappingMode(1i32);
    pub const Repeat: SceneWrappingMode = SceneWrappingMode(2i32);
}
impl ::core::convert::From<i32> for SceneWrappingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneWrappingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneWrappingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneWrappingMode;i4)");
}
impl ::windows::runtime::DefaultType for SceneWrappingMode {
    type DefaultType = Self;
}
