#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchor {
    type Vtable = ISpatialAnchor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0529e5ce_1d34_3702_bcec_eabff578a869);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchor2 {
    type Vtable = ISpatialAnchor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed17c908_a695_4cf6_92fd_97263ba71047);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2a4338_24fb_4269_89c5_88304aeef20f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorExporterStatics {
    type Vtable = ISpatialAnchorExporterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2507b8_2475_439c_85ff_7fed341fdc88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorManagerStatics {
    type Vtable = ISpatialAnchorManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e30eab_f3b7_420b_b086_8a80c07d910d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorStatics {
    type Vtable = ISpatialAnchorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9928642_0174_311c_ae79_0e5107669f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, anchor: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAnchorTransferManagerStatics {
    type Vtable = ISpatialAnchorTransferManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03bbf9b9_12d8_4bce_8835_c5df3ac0adab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, anchors: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2065da_68c3_33df_b7af_4c787207999c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialBoundingVolumeStatics {
    type Vtable = ISpatialBoundingVolumeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05889117_b3e1_36d8_b017_566181a5b196);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingOrientedBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, sphere: SpatialBoundingSphere, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, frustum: SpatialBoundingFrustum, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69ebca4b_60a3_3586_a653_59a7bd676d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntity(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntity {
    type Vtable = ISpatialEntity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x166de955_e1eb_454c_ba08_e6c0668ddc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa397f49b_156a_4707_ac2c_d31d570ed399);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityFactory {
    type Vtable = ISpatialEntityFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1f1e325_349f_4225_a2f3_4b01c15fe056);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, spatialanchor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, spatialanchor: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91741800_536d_4e9f_abf6_415b5444d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityStore {
    type Vtable = ISpatialEntityStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x329788ba_e513_4f06_889d_1be30ecf43e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityStoreStatics {
    type Vtable = ISpatialEntityStoreStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b4b389e_7c50_4e92_8a62_4d1d4b7ccd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5671766_627b_43cb_a49f_b3be6d47deed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpatialEntityWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocation {
    type Vtable = ISpatialLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocation2 {
    type Vtable = ISpatialLocation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x117f2416_38a7_4a18_b404_ab8fabe1d78b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocator {
    type Vtable = ISpatialLocator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpatialLocatability) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, headingoffsetinradians: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialLocatorStatics {
    type Vtable = ISpatialLocatorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb76e3340_a7c2_361b_bb82_56e93b89b1bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a8a3464_ad0d_4590_ab86_33062b674926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpatialMovementRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpatialLookDirectionRange) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, locator: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialStageFrameOfReferenceStatics {
    type Vtable = ISpatialStageFrameOfReferenceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf78d5c4d_a0a4_499c_8d91_a8c965d40654);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAnchor(pub ::windows::core::IInspectable);
impl SpatialAnchor {
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn RawCoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RawCoordinateSystemAdjusted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawCoordinateSystemAdjusted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn TryCreateRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(coordinatesystem: Param0, position: Param1) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(coordinatesystem: Param0, position: Param1, orientation: Param2) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    pub fn RemovedByUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ISpatialAnchorStatics<R, F: FnOnce(&ISpatialAnchorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchor;{0529e5ce-1d34-3702-bcec-eabff578a869})");
}
unsafe impl ::windows::core::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0529e5ce_1d34_3702_bcec_eabff578a869);
}
impl ::windows::core::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
impl ::core::convert::From<SpatialAnchor> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAnchor> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAnchor {}
unsafe impl ::core::marker::Sync for SpatialAnchor {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(0i32);
    pub const Sharing: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(1i32);
}
impl ::core::convert::From<i32> for SpatialAnchorExportPurpose {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAnchorExportPurpose {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)");
}
impl ::windows::core::DefaultType for SpatialAnchorExportPurpose {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAnchorExportSufficiency(pub ::windows::core::IInspectable);
impl SpatialAnchorExportSufficiency {
    pub fn IsMinimallySufficient(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SufficiencyLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn RecommendedSufficiencyLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExportSufficiency;{77c25b2b-3409-4088-b91b-fdfd05d1648f})");
}
unsafe impl ::windows::core::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
}
impl ::windows::core::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExportSufficiency {}
unsafe impl ::core::marker::Sync for SpatialAnchorExportSufficiency {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAnchorExporter(pub ::windows::core::IInspectable);
impl SpatialAnchorExporter {
    #[cfg(feature = "Foundation")]
    pub fn GetAnchorExportSufficiencyAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), anchor.into_param().abi(), purpose, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryExportAnchorAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose, stream: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), anchor.into_param().abi(), purpose, stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAnchorExporter>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn ISpatialAnchorExporterStatics<R, F: FnOnce(&ISpatialAnchorExporterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorExporter, ISpatialAnchorExporterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExporter;{9a2a4338-24fb-4269-89c5-88304aeef20f})");
}
unsafe impl ::windows::core::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2a4338_24fb_4269_89c5_88304aeef20f);
}
impl ::windows::core::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorExporter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorExporter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorExporter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorExporter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExporter {}
unsafe impl ::core::marker::Sync for SpatialAnchorExporter {}
pub struct SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>> {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>(result__)
        })
    }
    pub fn ISpatialAnchorManagerStatics<R, F: FnOnce(&ISpatialAnchorManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorManager, ISpatialAnchorManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub ::windows::core::IInspectable);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs;{a1e81eb8-56c7-3117-a2e4-81e0fcf28e00})");
}
unsafe impl ::windows::core::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
}
impl ::windows::core::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAnchorStore(pub ::windows::core::IInspectable);
impl SpatialAnchorStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>(result__)
        }
    }
    pub fn TrySave<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SpatialAnchor>>(&self, id: Param0, anchor: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), anchor.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorStore;{b0bc3636-486a-3cb0-9e6f-1245165c4db6})");
}
unsafe impl ::windows::core::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
}
impl ::windows::core::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorStore {}
unsafe impl ::core::marker::Sync for SpatialAnchorStore {}
pub struct SpatialAnchorTransferManager {}
impl SpatialAnchorTransferManager {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryImportAnchorsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(stream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn TryExportAnchorsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>>>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(anchors: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), anchors.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn ISpatialAnchorTransferManagerStatics<R, F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorTransferManager, ISpatialAnchorTransferManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingBox {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialBoundingBox").field("Center", &self.Center).field("Extents", &self.Extents).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Extents == other.Extents
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for SpatialBoundingBox {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingFrustum {
    pub Near: super::super::Foundation::Numerics::Plane,
    pub Far: super::super::Foundation::Numerics::Plane,
    pub Right: super::super::Foundation::Numerics::Plane,
    pub Left: super::super::Foundation::Numerics::Plane,
    pub Top: super::super::Foundation::Numerics::Plane,
    pub Bottom: super::super::Foundation::Numerics::Plane,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingFrustum {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialBoundingFrustum").field("Near", &self.Near).field("Far", &self.Far).field("Right", &self.Right).field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingFrustum {
    fn eq(&self, other: &Self) -> bool {
        self.Near == other.Near && self.Far == other.Far && self.Right == other.Right && self.Left == other.Left && self.Top == other.Top && self.Bottom == other.Bottom
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingFrustum {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for SpatialBoundingFrustum {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingOrientedBox {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialBoundingOrientedBox").field("Center", &self.Center).field("Extents", &self.Extents).field("Orientation", &self.Orientation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingOrientedBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Extents == other.Extents && self.Orientation == other.Orientation
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingOrientedBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for SpatialBoundingOrientedBox {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingSphere {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialBoundingSphere").field("Center", &self.Center).field("Radius", &self.Radius).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Radius == other.Radius
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingSphere {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for SpatialBoundingSphere {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialBoundingVolume(pub ::windows::core::IInspectable);
impl SpatialBoundingVolume {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingOrientedBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingSphere>>(coordinatesystem: Param0, sphere: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), sphere.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingFrustum>>(coordinatesystem: Param0, frustum: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), frustum.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    pub fn ISpatialBoundingVolumeStatics<R, F: FnOnce(&ISpatialBoundingVolumeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialBoundingVolume, ISpatialBoundingVolumeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialBoundingVolume;{fb2065da-68c3-33df-b7af-4c787207999c})");
}
unsafe impl ::windows::core::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2065da_68c3_33df_b7af_4c787207999c);
}
impl ::windows::core::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows::core::IUnknown {
    fn from(value: SpatialBoundingVolume) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows::core::IUnknown {
    fn from(value: &SpatialBoundingVolume) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows::core::IInspectable {
    fn from(value: SpatialBoundingVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows::core::IInspectable {
    fn from(value: &SpatialBoundingVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialBoundingVolume {}
unsafe impl ::core::marker::Sync for SpatialBoundingVolume {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialCoordinateSystem(pub ::windows::core::IInspectable);
impl SpatialCoordinateSystem {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn TryGetTransformTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, target: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialCoordinateSystem;{69ebca4b-60a3-3586-a653-59a7bd676d07})");
}
unsafe impl ::windows::core::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69ebca4b_60a3_3586_a653_59a7bd676d07);
}
impl ::windows::core::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows::core::IUnknown {
    fn from(value: SpatialCoordinateSystem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows::core::IUnknown {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows::core::IInspectable {
    fn from(value: SpatialCoordinateSystem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows::core::IInspectable {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialCoordinateSystem {}
unsafe impl ::core::marker::Sync for SpatialCoordinateSystem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntity(pub ::windows::core::IInspectable);
impl SpatialEntity {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Anchor(&self) -> ::windows::core::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAnchor>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn CreateWithSpatialAnchor<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>>(spatialanchor: Param0) -> ::windows::core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), spatialanchor.into_param().abi(), &mut result__).from_abi::<SpatialEntity>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(spatialanchor: Param0, propertyset: Param1) -> ::windows::core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), spatialanchor.into_param().abi(), propertyset.into_param().abi(), &mut result__).from_abi::<SpatialEntity>(result__)
        })
    }
    pub fn ISpatialEntityFactory<R, F: FnOnce(&ISpatialEntityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialEntity, ISpatialEntityFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntity;{166de955-e1eb-454c-ba08-e6c0668ddc65})");
}
unsafe impl ::windows::core::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x166de955_e1eb_454c_ba08_e6c0668ddc65);
}
impl ::windows::core::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
impl ::core::convert::From<SpatialEntity> for ::windows::core::IUnknown {
    fn from(value: SpatialEntity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntity> for ::windows::core::IInspectable {
    fn from(value: SpatialEntity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntity {}
unsafe impl ::core::marker::Sync for SpatialEntity {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntityAddedEventArgs(pub ::windows::core::IInspectable);
impl SpatialEntityAddedEventArgs {
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityAddedEventArgs;{a397f49b-156a-4707-ac2c-d31d570ed399})");
}
unsafe impl ::windows::core::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa397f49b_156a_4707_ac2c_d31d570ed399);
}
impl ::windows::core::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntityAddedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityAddedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntityRemovedEventArgs(pub ::windows::core::IInspectable);
impl SpatialEntityRemovedEventArgs {
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityRemovedEventArgs;{91741800-536d-4e9f-abf6-415b5444d651})");
}
unsafe impl ::windows::core::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91741800_536d_4e9f_abf6_415b5444d651);
}
impl ::windows::core::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntityRemovedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityRemovedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntityStore(pub ::windows::core::IInspectable);
impl SpatialEntityStore {
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), entity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), entity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateEntityWatcher(&self) -> ::windows::core::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntityWatcher>(result__)
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemSession>>(session: Param0) -> ::windows::core::Result<SpatialEntityStore> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), session.into_param().abi(), &mut result__).from_abi::<SpatialEntityStore>(result__)
        })
    }
    pub fn ISpatialEntityStoreStatics<R, F: FnOnce(&ISpatialEntityStoreStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialEntityStore, ISpatialEntityStoreStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityStore;{329788ba-e513-4f06-889d-1be30ecf43e6})");
}
unsafe impl ::windows::core::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x329788ba_e513_4f06_889d_1be30ecf43e6);
}
impl ::windows::core::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
impl ::core::convert::From<SpatialEntityStore> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntityStore> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntityStore {}
unsafe impl ::core::marker::Sync for SpatialEntityStore {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntityUpdatedEventArgs(pub ::windows::core::IInspectable);
impl SpatialEntityUpdatedEventArgs {
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs;{e5671766-627b-43cb-a49f-b3be6d47deed})");
}
unsafe impl ::windows::core::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5671766_627b_43cb_a49f_b3be6d47deed);
}
impl ::windows::core::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntityUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityUpdatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialEntityWatcher(pub ::windows::core::IInspectable);
impl SpatialEntityWatcher {
    pub fn Status(&self) -> ::windows::core::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: SpatialEntityWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntityWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityWatcher;{b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959})");
}
unsafe impl ::windows::core::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
}
impl ::windows::core::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialEntityWatcher {}
unsafe impl ::core::marker::Sync for SpatialEntityWatcher {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(0i32);
    pub const Started: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(1i32);
    pub const EnumerationCompleted: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(2i32);
    pub const Stopping: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(3i32);
    pub const Stopped: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(4i32);
    pub const Aborted: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for SpatialEntityWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialEntityWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for SpatialEntityWatcherStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: SpatialLocatability = SpatialLocatability(0i32);
    pub const OrientationOnly: SpatialLocatability = SpatialLocatability(1i32);
    pub const PositionalTrackingActivating: SpatialLocatability = SpatialLocatability(2i32);
    pub const PositionalTrackingActive: SpatialLocatability = SpatialLocatability(3i32);
    pub const PositionalTrackingInhibited: SpatialLocatability = SpatialLocatability(4i32);
}
impl ::core::convert::From<i32> for SpatialLocatability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialLocatability {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)");
}
impl ::windows::core::DefaultType for SpatialLocatability {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialLocation(pub ::windows::core::IInspectable);
impl SpatialLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocation;{1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2})");
}
unsafe impl ::windows::core::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
}
impl ::windows::core::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
impl ::core::convert::From<SpatialLocation> for ::windows::core::IUnknown {
    fn from(value: SpatialLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialLocation> for ::windows::core::IInspectable {
    fn from(value: SpatialLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialLocation {}
unsafe impl ::core::marker::Sync for SpatialLocation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialLocator(pub ::windows::core::IInspectable);
impl SpatialLocator {
    pub fn Locatability(&self) -> ::windows::core::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__: SpatialLocatability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocatability>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LocatabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLocatabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionalTrackingDeactivating<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionalTrackingDeactivating<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn TryLocateAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>, Param1: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, timestamp: Param0, coordinatesystem: Param1) -> ::windows::core::Result<SpatialLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialLocation>(result__)
        }
    }
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocator>(result__)
        })
    }
    pub fn ISpatialLocatorStatics<R, F: FnOnce(&ISpatialLocatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialLocator, ISpatialLocatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocator;{f6478925-9e0c-3bb6-997e-b64ecca24cf4})");
}
unsafe impl ::windows::core::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
}
impl ::windows::core::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
impl ::core::convert::From<SpatialLocator> for ::windows::core::IUnknown {
    fn from(value: SpatialLocator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialLocator> for ::windows::core::IInspectable {
    fn from(value: SpatialLocator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialLocator {}
unsafe impl ::core::marker::Sync for SpatialLocator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialLocatorAttachedFrameOfReference(pub ::windows::core::IInspectable);
impl SpatialLocatorAttachedFrameOfReference {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), headingoffsetinradians).ok() }
    }
    pub fn GetStationaryCoordinateSystemAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryGetRelativeHeadingAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference;{e1774ef6-1f4f-499c-9625-ef5e6ed7a048})");
}
unsafe impl ::windows::core::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
}
impl ::windows::core::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialLocatorAttachedFrameOfReference {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(pub ::windows::core::IInspectable);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub fn Canceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanceled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs;{b8a84063-e3f4-368b-9061-9ea9d1d6cc16})");
}
unsafe impl ::windows::core::Interface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
}
impl ::windows::core::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl ::core::marker::Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: SpatialLookDirectionRange = SpatialLookDirectionRange(0i32);
    pub const Omnidirectional: SpatialLookDirectionRange = SpatialLookDirectionRange(1i32);
}
impl ::core::convert::From<i32> for SpatialLookDirectionRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialLookDirectionRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)");
}
impl ::windows::core::DefaultType for SpatialLookDirectionRange {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: SpatialMovementRange = SpatialMovementRange(0i32);
    pub const Bounded: SpatialMovementRange = SpatialMovementRange(1i32);
}
impl ::core::convert::From<i32> for SpatialMovementRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialMovementRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialMovementRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)");
}
impl ::windows::core::DefaultType for SpatialMovementRange {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(0i32);
    pub const Allowed: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(1i32);
    pub const DeniedByUser: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(2i32);
    pub const DeniedBySystem: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(3i32);
}
impl ::core::convert::From<i32> for SpatialPerceptionAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialPerceptionAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)");
}
impl ::windows::core::DefaultType for SpatialPerceptionAccessStatus {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialRay {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialRay").field("Origin", &self.Origin).field("Direction", &self.Direction).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialRay {
    fn eq(&self, other: &Self) -> bool {
        self.Origin == other.Origin && self.Direction == other.Direction
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialRay {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialRay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::DefaultType for SpatialRay {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialStageFrameOfReference(pub ::windows::core::IInspectable);
impl SpatialStageFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn MovementRange(&self) -> ::windows::core::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialMovementRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialMovementRange>(result__)
        }
    }
    pub fn LookDirectionRange(&self) -> ::windows::core::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialLookDirectionRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLookDirectionRange>(result__)
        }
    }
    pub fn GetCoordinateSystemAtCurrentLocation<'a, Param0: ::windows::core::IntoParam<'a, SpatialLocator>>(&self, locator: Param0) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), locator.into_param().abi(), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<super::super::Foundation::Numerics::Vector3> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), ::windows::core::Array::<super::super::Foundation::Numerics::Vector3>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialStageFrameOfReference>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestNewStageAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>(result__)
        })
    }
    pub fn ISpatialStageFrameOfReferenceStatics<R, F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialStageFrameOfReference, ISpatialStageFrameOfReferenceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStageFrameOfReference;{7a8a3464-ad0d-4590-ab86-33062b674926})");
}
unsafe impl ::windows::core::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a8a3464_ad0d_4590_ab86_33062b674926);
}
impl ::windows::core::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialStageFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStageFrameOfReference {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialStationaryFrameOfReference(pub ::windows::core::IInspectable);
impl SpatialStationaryFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStationaryFrameOfReference;{09dbccb9-bcf8-3e7f-be7e-7edccbb178a8})");
}
unsafe impl ::windows::core::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
}
impl ::windows::core::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialStationaryFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStationaryFrameOfReference {}
