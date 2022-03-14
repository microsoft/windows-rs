#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0529e5ce_1d34_3702_bcec_eabff578a869);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RawCoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawCoordinateSystemAdjusted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawCoordinateSystemAdjusted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchor2 {
    type Vtable = ISpatialAnchor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed17c908_a695_4cf6_92fd_97263ba71047);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExportSufficiency(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsMinimallySufficient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub RecommendedSufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2a4338_24fb_4269_89c5_88304aeef20f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetAnchorExportSufficiencyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnchorExportSufficiencyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryExportAnchorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: ::windows::core::RawPtr, purpose: SpatialAnchorExportPurpose, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryExportAnchorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorExporterStatics {
    type Vtable = ISpatialAnchorExporterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2507b8_2475_439c_85ff_7fed341fdc88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorManagerStatics {
    type Vtable = ISpatialAnchorManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e30eab_f3b7_420b_b086_8a80c07d910d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub OldRawCoordinateSystemToNewRawCoordinateSystemTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OldRawCoordinateSystemToNewRawCoordinateSystemTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorStatics {
    type Vtable = ISpatialAnchorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9928642_0174_311c_ae79_0e5107669f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCreateRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionRelativeTo: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionAndOrientationRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionAndOrientationRelativeTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSavedAnchors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSavedAnchors: usize,
    pub TrySave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, anchor: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISpatialAnchorTransferManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISpatialAnchorTransferManagerStatics {
    type Vtable = ISpatialAnchorTransferManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03bbf9b9_12d8_4bce_8835_c5df3ac0adab);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryImportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryImportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryExportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchors: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryExportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolume(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2065da_68c3_33df_b7af_4c787207999c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolumeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialBoundingVolumeStatics {
    type Vtable = ISpatialBoundingVolumeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05889117_b3e1_36d8_b017_566181a5b196);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromOrientedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, r#box: SpatialBoundingOrientedBox, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromOrientedBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromSphere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, sphere: SpatialBoundingSphere, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromSphere: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, frustum: SpatialBoundingFrustum, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromFrustum: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialCoordinateSystem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69ebca4b_60a3_3586_a653_59a7bd676d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetTransformTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x166de955_e1eb_454c_ba08_e6c0668ddc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntity_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Anchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa397f49b_156a_4707_ac2c_d31d570ed399);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityFactory {
    type Vtable = ISpatialEntityFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1f1e325_349f_4225_a2f3_4b01c15fe056);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWithSpatialAnchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithSpatialAnchorAndProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: ::windows::core::RawPtr, propertyset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithSpatialAnchorAndProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91741800_536d_4e9f_abf6_415b5444d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x329788ba_e513_4f06_889d_1be30ecf43e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAsync: usize,
    pub CreateEntityWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStoreStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityStoreStatics {
    type Vtable = ISpatialEntityStoreStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b4b389e_7c50_4e92_8a62_4d1d4b7ccd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub TryGetForRemoteSystemSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    TryGetForRemoteSystemSession: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5671766_627b_43cb_a49f_b3be6d47deed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialEntityWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearAcceleration: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularVelocity: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularAcceleration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocation2 {
    type Vtable = ISpatialLocation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x117f2416_38a7_4a18_b404_ab8fabe1d78b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularVelocityAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularVelocityAxisAngle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularAccelerationAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularAccelerationAxisAngle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocator_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Locatability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLocatability) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionalTrackingDeactivating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionalTrackingDeactivating: usize,
    pub TryLocateAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAttachedFrameOfReferenceAtCurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: usize,
    pub CreateStationaryFrameOfReferenceAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorAttachedFrameOfReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOrientation: usize,
    pub AdjustHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headingoffsetinradians: f64) -> ::windows::core::HRESULT,
    pub GetStationaryCoordinateSystemAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetRelativeHeadingAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetRelativeHeadingAtTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialLocatorStatics {
    type Vtable = ISpatialLocatorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb76e3340_a7c2_361b_bb82_56e93b89b1bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a8a3464_ad0d_4590_ab86_33062b674926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MovementRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialMovementRange) -> ::windows::core::HRESULT,
    pub LookDirectionRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLookDirectionRange) -> ::windows::core::HRESULT,
    pub GetCoordinateSystemAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locator: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMovementBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMovementBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReferenceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialStageFrameOfReferenceStatics {
    type Vtable = ISpatialStageFrameOfReferenceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf78d5c4d_a0a4_499c_8d91_a8c965d40654);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewStageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewStageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStationaryFrameOfReference(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchor(::windows::core::IUnknown);
impl SpatialAnchor {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn RawCoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawCoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RawCoordinateSystemAdjusted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawCoordinateSystemAdjusted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawCoordinateSystemAdjusted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRawCoordinateSystemAdjusted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn RemovedByUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemovedByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn TryCreateRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateRelativeTo)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(coordinatesystem: Param0, position: Param1) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateWithPositionRelativeTo)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(coordinatesystem: Param0, position: Param1, orientation: Param2) -> ::windows::core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateWithPositionAndOrientationRelativeTo)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), &mut result__).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorStatics<R, F: FnOnce(&ISpatialAnchorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchor {}
impl ::core::fmt::Debug for SpatialAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchor;{0529e5ce-1d34-3702-bcec-eabff578a869})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAnchor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
impl ::core::convert::From<SpatialAnchor> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchor> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchor {}
unsafe impl ::core::marker::Sync for SpatialAnchor {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: Self = Self(0i32);
    pub const Sharing: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAnchorExportPurpose {}
impl ::core::clone::Clone for SpatialAnchorExportPurpose {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAnchorExportPurpose {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAnchorExportPurpose {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAnchorExportPurpose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportPurpose").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorExportSufficiency(::windows::core::IUnknown);
impl SpatialAnchorExportSufficiency {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn IsMinimallySufficient(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMinimallySufficient)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn SufficiencyLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SufficiencyLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn RecommendedSufficiencyLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecommendedSufficiencyLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExportSufficiency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExportSufficiency {}
impl ::core::fmt::Debug for SpatialAnchorExportSufficiency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportSufficiency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExportSufficiency;{77c25b2b-3409-4088-b91b-fdfd05d1648f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAnchorExportSufficiency as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExportSufficiency {}
unsafe impl ::core::marker::Sync for SpatialAnchorExportSufficiency {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorExporter(::windows::core::IUnknown);
impl SpatialAnchorExporter {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnchorExportSufficiencyAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAnchorExportSufficiencyAsync)(::core::mem::transmute_copy(this), anchor.into_param().abi(), purpose, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryExportAnchorAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose, stream: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryExportAnchorAsync)(::core::mem::transmute_copy(this), anchor.into_param().abi(), purpose, stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAnchorExporter>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorExporterStatics<R, F: FnOnce(&ISpatialAnchorExporterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorExporter, ISpatialAnchorExporterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAnchorExporter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExporter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExporter {}
impl ::core::fmt::Debug for SpatialAnchorExporter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExporter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExporter;{9a2a4338-24fb-4269-89c5-88304aeef20f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAnchorExporter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorExporter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorExporter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorExporter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorExporter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExporter {}
unsafe impl ::core::marker::Sync for SpatialAnchorExporter {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
pub struct SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>> {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorManagerStatics<R, F: FnOnce(&ISpatialAnchorManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorManager, ISpatialAnchorManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows::core::IUnknown);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
impl ::core::fmt::Debug for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorRawCoordinateSystemAdjustedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs;{a1e81eb8-56c7-3117-a2e4-81e0fcf28e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAnchorRawCoordinateSystemAdjustedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorStore(::windows::core::IUnknown);
impl SpatialAnchorStore {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllSavedAnchors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn TrySave<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SpatialAnchor>>(&self, id: Param0, anchor: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySave)(::core::mem::transmute_copy(this), id.into_param().abi(), anchor.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SpatialAnchorStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorStore {}
impl ::core::fmt::Debug for SpatialAnchorStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorStore;{b0bc3636-486a-3cb0-9e6f-1245165c4db6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAnchorStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows::core::IUnknown {
    fn from(value: SpatialAnchorStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows::core::IUnknown {
    fn from(value: &SpatialAnchorStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows::core::IInspectable {
    fn from(value: SpatialAnchorStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows::core::IInspectable {
    fn from(value: &SpatialAnchorStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorStore {}
unsafe impl ::core::marker::Sync for SpatialAnchorStore {}
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct SpatialAnchorTransferManager {}
#[cfg(feature = "deprecated")]
impl SpatialAnchorTransferManager {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryImportAnchorsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(stream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryImportAnchorsAsync)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryExportAnchorsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>>>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream>>(anchors: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryExportAnchorsAsync)(::core::mem::transmute_copy(this), anchors.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISpatialAnchorTransferManagerStatics<R, F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAnchorTransferManager, ISpatialAnchorTransferManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingBox").field("Center", &self.Center).field("Extents", &self.Extents).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingBox>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
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
impl ::core::marker::Copy for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingFrustum {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingFrustum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingFrustum").field("Near", &self.Near).field("Far", &self.Far).field("Right", &self.Right).field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingFrustum {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingFrustum {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingFrustum>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingOrientedBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingOrientedBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingOrientedBox").field("Center", &self.Center).field("Extents", &self.Extents).field("Orientation", &self.Orientation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingOrientedBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingOrientedBox {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingOrientedBox>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingSphere {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingSphere {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingSphere").field("Center", &self.Center).field("Radius", &self.Radius).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialBoundingSphere {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingSphere>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialBoundingVolume(::windows::core::IUnknown);
impl SpatialBoundingVolume {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromBox)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingOrientedBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromOrientedBox)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingSphere>>(coordinatesystem: Param0, sphere: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromSphere)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), sphere.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, SpatialBoundingFrustum>>(coordinatesystem: Param0, frustum: Param1) -> ::windows::core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromFrustum)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), frustum.into_param().abi(), &mut result__).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialBoundingVolumeStatics<R, F: FnOnce(&ISpatialBoundingVolumeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialBoundingVolume, ISpatialBoundingVolumeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialBoundingVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialBoundingVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialBoundingVolume {}
impl ::core::fmt::Debug for SpatialBoundingVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialBoundingVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialBoundingVolume;{fb2065da-68c3-33df-b7af-4c787207999c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialBoundingVolume as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows::core::IUnknown {
    fn from(value: SpatialBoundingVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows::core::IUnknown {
    fn from(value: &SpatialBoundingVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows::core::IInspectable {
    fn from(value: SpatialBoundingVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows::core::IInspectable {
    fn from(value: &SpatialBoundingVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialBoundingVolume {}
unsafe impl ::core::marker::Sync for SpatialBoundingVolume {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialCoordinateSystem(::windows::core::IUnknown);
impl SpatialCoordinateSystem {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetTransformTo<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, target: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetTransformTo)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialCoordinateSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialCoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialCoordinateSystem {}
impl ::core::fmt::Debug for SpatialCoordinateSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialCoordinateSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialCoordinateSystem;{69ebca4b-60a3-3586-a653-59a7bd676d07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialCoordinateSystem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows::core::IUnknown {
    fn from(value: SpatialCoordinateSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows::core::IUnknown {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows::core::IInspectable {
    fn from(value: SpatialCoordinateSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows::core::IInspectable {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialCoordinateSystem {}
unsafe impl ::core::marker::Sync for SpatialCoordinateSystem {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntity(::windows::core::IUnknown);
impl SpatialEntity {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Anchor(&self) -> ::windows::core::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Anchor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAnchor>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CreateWithSpatialAnchor<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>>(spatialanchor: Param0) -> ::windows::core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSpatialAnchor)(::core::mem::transmute_copy(this), spatialanchor.into_param().abi(), &mut result__).from_abi::<SpatialEntity>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<'a, Param0: ::windows::core::IntoParam<'a, SpatialAnchor>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::ValueSet>>(spatialanchor: Param0, propertyset: Param1) -> ::windows::core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSpatialAnchorAndProperties)(::core::mem::transmute_copy(this), spatialanchor.into_param().abi(), propertyset.into_param().abi(), &mut result__).from_abi::<SpatialEntity>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityFactory<R, F: FnOnce(&ISpatialEntityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialEntity, ISpatialEntityFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntity {}
impl ::core::fmt::Debug for SpatialEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntity;{166de955-e1eb-454c-ba08-e6c0668ddc65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntity as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
impl ::core::convert::From<SpatialEntity> for ::windows::core::IUnknown {
    fn from(value: SpatialEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntity> for ::windows::core::IInspectable {
    fn from(value: SpatialEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntity {}
unsafe impl ::core::marker::Sync for SpatialEntity {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityAddedEventArgs(::windows::core::IUnknown);
impl SpatialEntityAddedEventArgs {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Entity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityAddedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityAddedEventArgs;{a397f49b-156a-4707-ac2c-d31d570ed399})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntityAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityAddedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityAddedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityRemovedEventArgs(::windows::core::IUnknown);
impl SpatialEntityRemovedEventArgs {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Entity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityRemovedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityRemovedEventArgs;{91741800-536d-4e9f-abf6-415b5444d651})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntityRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityRemovedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityRemovedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityStore(::windows::core::IUnknown);
impl SpatialEntityStore {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsync)(::core::mem::transmute_copy(this), entity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAsync<'a, Param0: ::windows::core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAsync)(::core::mem::transmute_copy(this), entity.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CreateEntityWatcher(&self) -> ::windows::core::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateEntityWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntityWatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"System_RemoteSystems\"`*"]
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemSession>>(session: Param0) -> ::windows::core::Result<SpatialEntityStore> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetForRemoteSystemSession)(::core::mem::transmute_copy(this), session.into_param().abi(), &mut result__).from_abi::<SpatialEntityStore>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityStoreStatics<R, F: FnOnce(&ISpatialEntityStoreStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialEntityStore, ISpatialEntityStoreStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialEntityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityStore {}
impl ::core::fmt::Debug for SpatialEntityStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityStore;{329788ba-e513-4f06-889d-1be30ecf43e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntityStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
impl ::core::convert::From<SpatialEntityStore> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityStore> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityStore {}
unsafe impl ::core::marker::Sync for SpatialEntityStore {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityUpdatedEventArgs(::windows::core::IUnknown);
impl SpatialEntityUpdatedEventArgs {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Entity(&self) -> ::windows::core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Entity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityUpdatedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs;{e5671766-627b-43cb-a49f-b3be6d47deed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntityUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityUpdatedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityWatcher(::windows::core::IUnknown);
impl SpatialEntityWatcher {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: SpatialEntityWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialEntityWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SpatialEntityWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityWatcher {}
impl ::core::fmt::Debug for SpatialEntityWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityWatcher;{b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialEntityWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows::core::IUnknown {
    fn from(value: SpatialEntityWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows::core::IUnknown {
    fn from(value: &SpatialEntityWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows::core::IInspectable {
    fn from(value: SpatialEntityWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows::core::IInspectable {
    fn from(value: &SpatialEntityWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityWatcher {}
unsafe impl ::core::marker::Sync for SpatialEntityWatcher {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialEntityWatcherStatus {}
impl ::core::clone::Clone for SpatialEntityWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialEntityWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialEntityWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialEntityWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const PositionalTrackingActivating: Self = Self(2i32);
    pub const PositionalTrackingActive: Self = Self(3i32);
    pub const PositionalTrackingInhibited: Self = Self(4i32);
}
impl ::core::marker::Copy for SpatialLocatability {}
impl ::core::clone::Clone for SpatialLocatability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialLocatability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialLocatability {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialLocatability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocation(::windows::core::IUnknown);
impl SpatialLocation {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteLinearVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteLinearAcceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteAngularVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteAngularAcceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteAngularVelocityAxisAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteAngularAccelerationAxisAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocation {}
impl ::core::fmt::Debug for SpatialLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocation;{1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
impl ::core::convert::From<SpatialLocation> for ::windows::core::IUnknown {
    fn from(value: SpatialLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocation> for ::windows::core::IInspectable {
    fn from(value: SpatialLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocation {}
unsafe impl ::core::marker::Sync for SpatialLocation {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocator(::windows::core::IUnknown);
impl SpatialLocator {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Locatability(&self) -> ::windows::core::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__: SpatialLocatability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Locatability)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocatability>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocatabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocatabilityChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLocatabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLocatabilityChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionalTrackingDeactivating<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionalTrackingDeactivating)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionalTrackingDeactivating<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePositionalTrackingDeactivating)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn TryLocateAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>, Param1: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, timestamp: Param0, coordinatesystem: Param1) -> ::windows::core::Result<SpatialLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryLocateAtTimestamp)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialLocation>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeading)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, &mut result__).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(::core::mem::transmute_copy(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, &mut result__).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLocator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialLocatorStatics<R, F: FnOnce(&ISpatialLocatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialLocator, ISpatialLocatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocator {}
impl ::core::fmt::Debug for SpatialLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocator;{f6478925-9e0c-3bb6-997e-b64ecca24cf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialLocator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
impl ::core::convert::From<SpatialLocator> for ::windows::core::IUnknown {
    fn from(value: SpatialLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocator> for ::windows::core::IInspectable {
    fn from(value: SpatialLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocator {}
unsafe impl ::core::marker::Sync for SpatialLocator {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(::windows::core::IUnknown);
impl SpatialLocatorAttachedFrameOfReference {
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativePosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativePosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeOrientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativeOrientation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AdjustHeading)(::core::mem::transmute_copy(this), headingoffsetinradians).ok() }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn GetStationaryCoordinateSystemAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStationaryCoordinateSystemAtTimestamp)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetRelativeHeadingAtTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetRelativeHeadingAtTimestamp)(::core::mem::transmute_copy(this), timestamp.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorAttachedFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorAttachedFrameOfReference {}
impl ::core::fmt::Debug for SpatialLocatorAttachedFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorAttachedFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference;{e1774ef6-1f4f-499c-9625-ef5e6ed7a048})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialLocatorAttachedFrameOfReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialLocatorAttachedFrameOfReference {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows::core::IUnknown);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Canceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Canceled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn SetCanceled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanceled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
impl ::core::fmt::Debug for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorPositionalTrackingDeactivatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs;{b8a84063-e3f4-368b-9061-9ea9d1d6cc16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialLocatorPositionalTrackingDeactivatingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl ::core::marker::Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: Self = Self(0i32);
    pub const Omnidirectional: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialLookDirectionRange {}
impl ::core::clone::Clone for SpatialLookDirectionRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialLookDirectionRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialLookDirectionRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialLookDirectionRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLookDirectionRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: Self = Self(0i32);
    pub const Bounded: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialMovementRange {}
impl ::core::clone::Clone for SpatialMovementRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialMovementRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialMovementRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialMovementRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialMovementRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialMovementRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialPerceptionAccessStatus {}
impl ::core::clone::Clone for SpatialPerceptionAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialPerceptionAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialPerceptionAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialPerceptionAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPerceptionAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialRay {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialRay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialRay").field("Origin", &self.Origin).field("Direction", &self.Direction).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for SpatialRay {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for SpatialRay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialRay {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialRay>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(::windows::core::IUnknown);
impl SpatialStageFrameOfReference {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn MovementRange(&self) -> ::windows::core::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialMovementRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MovementRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialMovementRange>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn LookDirectionRange(&self) -> ::windows::core::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialLookDirectionRange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LookDirectionRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialLookDirectionRange>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn GetCoordinateSystemAtCurrentLocation<'a, Param0: ::windows::core::IntoParam<'a, SpatialLocator>>(&self, locator: Param0) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCoordinateSystemAtCurrentLocation)(::core::mem::transmute_copy(this), locator.into_param().abi(), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<'a, Param0: ::windows::core::IntoParam<'a, SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<super::super::Foundation::Numerics::Vector3> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetMovementBounds)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), ::windows::core::Array::<super::super::Foundation::Numerics::Vector3>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn Current() -> ::windows::core::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialStageFrameOfReference>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCurrentChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewStageAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestNewStageAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialStageFrameOfReferenceStatics<R, F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialStageFrameOfReference, ISpatialStageFrameOfReferenceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialStageFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStageFrameOfReference {}
impl ::core::fmt::Debug for SpatialStageFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStageFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStageFrameOfReference;{7a8a3464-ad0d-4590-ab86-33062b674926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialStageFrameOfReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialStageFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStageFrameOfReference {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(::windows::core::IUnknown);
impl SpatialStationaryFrameOfReference {
    #[doc = "*Required features: `\"Perception_Spatial\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialStationaryFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStationaryFrameOfReference {}
impl ::core::fmt::Debug for SpatialStationaryFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStationaryFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStationaryFrameOfReference;{09dbccb9-bcf8-3e7f-be7e-7edccbb178a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialStationaryFrameOfReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows::core::IUnknown {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows::core::IInspectable {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialStationaryFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStationaryFrameOfReference {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
