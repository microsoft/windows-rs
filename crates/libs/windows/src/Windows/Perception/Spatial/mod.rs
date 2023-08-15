#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0529e5ce_1d34_3702_bcec_eabff578a869);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RawCoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawCoordinateSystemAdjusted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawCoordinateSystemAdjusted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchor2 {
    type Vtable = ISpatialAnchor2_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed17c908_a695_4cf6_92fd_97263ba71047);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExportSufficiency(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorExportSufficiency {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsMinimallySufficient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub RecommendedSufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorExporter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorExporter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a2a4338_24fb_4269_89c5_88304aeef20f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetAnchorExportSufficiencyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: *mut ::core::ffi::c_void, purpose: SpatialAnchorExportPurpose, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnchorExportSufficiencyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryExportAnchorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: *mut ::core::ffi::c_void, purpose: SpatialAnchorExportPurpose, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryExportAnchorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExporterStatics {
    type Vtable = ISpatialAnchorExporterStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorExporterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorExporterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed2507b8_2475_439c_85ff_7fed341fdc88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorManagerStatics {
    type Vtable = ISpatialAnchorManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88e30eab_f3b7_420b_b086_8a80c07d910d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub OldRawCoordinateSystemToNewRawCoordinateSystemTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OldRawCoordinateSystemToNewRawCoordinateSystemTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorStatics {
    type Vtable = ISpatialAnchorStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9928642_0174_311c_ae79_0e5107669f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryCreateRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionRelativeTo: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionAndOrientationRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionAndOrientationRelativeTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
}
impl ::core::clone::Clone for ISpatialAnchorStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialAnchorStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSavedAnchors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSavedAnchors: usize,
    pub TrySave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, anchor: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISpatialAnchorTransferManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISpatialAnchorTransferManagerStatics {
    type Vtable = ISpatialAnchorTransferManagerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISpatialAnchorTransferManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISpatialAnchorTransferManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03bbf9b9_12d8_4bce_8835_c5df3ac0adab);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryImportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryImportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryExportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchors: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryExportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolume(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
}
impl ::core::clone::Clone for ISpatialBoundingVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialBoundingVolume {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2065da_68c3_33df_b7af_4c787207999c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolumeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialBoundingVolumeStatics {
    type Vtable = ISpatialBoundingVolumeStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialBoundingVolumeStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialBoundingVolumeStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05889117_b3e1_36d8_b017_566181a5b196);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, r#box: SpatialBoundingBox, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromOrientedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, r#box: SpatialBoundingOrientedBox, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromOrientedBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromSphere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, sphere: SpatialBoundingSphere, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromSphere: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, frustum: SpatialBoundingFrustum, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromFrustum: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialCoordinateSystem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
}
impl ::core::clone::Clone for ISpatialCoordinateSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialCoordinateSystem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69ebca4b_60a3_3586_a653_59a7bd676d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetTransformTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x166de955_e1eb_454c_ba08_e6c0668ddc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntity_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Anchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityAddedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa397f49b_156a_4707_ac2c_d31d570ed399);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityFactory {
    type Vtable = ISpatialEntityFactory_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1f1e325_349f_4225_a2f3_4b01c15fe056);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithSpatialAnchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithSpatialAnchorAndProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithSpatialAnchorAndProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityRemovedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91741800_536d_4e9f_abf6_415b5444d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x329788ba_e513_4f06_889d_1be30ecf43e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAsync: usize,
    pub CreateEntityWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStoreStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityStoreStatics {
    type Vtable = ISpatialEntityStoreStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityStoreStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityStoreStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b4b389e_7c50_4e92_8a62_4d1d4b7ccd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub TryGetForRemoteSystemSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    TryGetForRemoteSystemSession: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityUpdatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5671766_627b_43cb_a49f_b3be6d47deed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
}
impl ::core::clone::Clone for ISpatialEntityWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialEntityWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialEntityWatcherStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearAcceleration: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularVelocity: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularAcceleration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocation2 {
    type Vtable = ISpatialLocation2_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x117f2416_38a7_4a18_b404_ab8fabe1d78b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularVelocityAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularVelocityAxisAngle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularAccelerationAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularAccelerationAxisAngle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Locatability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLocatability) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionalTrackingDeactivating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionalTrackingDeactivating: usize,
    pub TryLocateAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAttachedFrameOfReferenceAtCurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: usize,
    pub CreateStationaryFrameOfReferenceAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorAttachedFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocatorAttachedFrameOfReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOrientation: usize,
    pub AdjustHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headingoffsetinradians: f64) -> ::windows_core::HRESULT,
    pub GetStationaryCoordinateSystemAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetRelativeHeadingAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetRelativeHeadingAtTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorStatics {
    type Vtable = ISpatialLocatorStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialLocatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialLocatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb76e3340_a7c2_361b_bb82_56e93b89b1bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
}
impl ::core::clone::Clone for ISpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialStageFrameOfReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a8a3464_ad0d_4590_ab86_33062b674926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MovementRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialMovementRange) -> ::windows_core::HRESULT,
    pub LookDirectionRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLookDirectionRange) -> ::windows_core::HRESULT,
    pub GetCoordinateSystemAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locator: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMovementBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMovementBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReferenceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStageFrameOfReferenceStatics {
    type Vtable = ISpatialStageFrameOfReferenceStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialStageFrameOfReferenceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialStageFrameOfReferenceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf78d5c4d_a0a4_499c_8d91_a8c965d40654);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewStageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewStageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStationaryFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
}
impl ::core::clone::Clone for ISpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpatialStationaryFrameOfReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchor(::windows_core::IUnknown);
impl SpatialAnchor {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RawCoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawCoordinateSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RawCoordinateSystemAdjusted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawCoordinateSystemAdjusted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawCoordinateSystemAdjusted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRawCoordinateSystemAdjusted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn RemovedByUser(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemovedByUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryCreateRelativeTo<P0>(coordinatesystem: P0) -> ::windows_core::Result<SpatialAnchor>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<P0>(coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<SpatialAnchor>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateWithPositionRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<P0>(coordinatesystem: P0, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<SpatialAnchor>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateWithPositionAndOrientationRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position, orientation, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorStatics<R, F: FnOnce(&ISpatialAnchorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialAnchor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchor;{0529e5ce-1d34-3702-bcec-eabff578a869})");
}
impl ::core::clone::Clone for SpatialAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialAnchor {
    const IID: ::windows_core::GUID = <ISpatialAnchor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
::windows_core::imp::interface_hierarchy!(SpatialAnchor, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAnchor {}
unsafe impl ::core::marker::Sync for SpatialAnchor {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorExportSufficiency(::windows_core::IUnknown);
impl SpatialAnchorExportSufficiency {
    pub fn IsMinimallySufficient(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMinimallySufficient)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SufficiencyLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SufficiencyLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecommendedSufficiencyLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecommendedSufficiencyLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExportSufficiency;{77c25b2b-3409-4088-b91b-fdfd05d1648f})");
}
impl ::core::clone::Clone for SpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialAnchorExportSufficiency {
    const IID: ::windows_core::GUID = <ISpatialAnchorExportSufficiency as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
::windows_core::imp::interface_hierarchy!(SpatialAnchorExportSufficiency, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAnchorExportSufficiency {}
unsafe impl ::core::marker::Sync for SpatialAnchorExportSufficiency {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorExporter(::windows_core::IUnknown);
impl SpatialAnchorExporter {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAnchorExportSufficiencyAsync<P0>(&self, anchor: P0, purpose: SpatialAnchorExportPurpose) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>
    where
        P0: ::windows_core::IntoParam<SpatialAnchor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnchorExportSufficiencyAsync)(::windows_core::Interface::as_raw(this), anchor.into_param().abi(), purpose, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryExportAnchorAsync<P0, P1>(&self, anchor: P0, purpose: SpatialAnchorExportPurpose, stream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<SpatialAnchor>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryExportAnchorAsync)(::windows_core::Interface::as_raw(this), anchor.into_param().abi(), purpose, stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorExporterStatics<R, F: FnOnce(&ISpatialAnchorExporterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialAnchorExporter, ISpatialAnchorExporterStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExporter;{9a2a4338-24fb-4269-89c5-88304aeef20f})");
}
impl ::core::clone::Clone for SpatialAnchorExporter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialAnchorExporter {
    const IID: ::windows_core::GUID = <ISpatialAnchorExporter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
::windows_core::imp::interface_hierarchy!(SpatialAnchorExporter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAnchorExporter {}
unsafe impl ::core::marker::Sync for SpatialAnchorExporter {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
pub struct SpatialAnchorManager;
impl SpatialAnchorManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>> {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAnchorManagerStatics<R, F: FnOnce(&ISpatialAnchorManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialAnchorManager, ISpatialAnchorManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows_core::IUnknown);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs;{a1e81eb8-56c7-3117-a2e4-81e0fcf28e00})");
}
impl ::core::clone::Clone for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const IID: ::windows_core::GUID = <ISpatialAnchorRawCoordinateSystemAdjustedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpatialAnchorRawCoordinateSystemAdjustedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorStore(::windows_core::IUnknown);
impl SpatialAnchorStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllSavedAnchors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySave<P0>(&self, id: &::windows_core::HSTRING, anchor: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<SpatialAnchor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySave)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), anchor.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, id: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id)).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorStore;{b0bc3636-486a-3cb0-9e6f-1245165c4db6})");
}
impl ::core::clone::Clone for SpatialAnchorStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialAnchorStore {
    const IID: ::windows_core::GUID = <ISpatialAnchorStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
::windows_core::imp::interface_hierarchy!(SpatialAnchorStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAnchorStore {}
unsafe impl ::core::marker::Sync for SpatialAnchorStore {}
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct SpatialAnchorTransferManager;
#[cfg(feature = "deprecated")]
impl SpatialAnchorTransferManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryImportAnchorsAsync<P0>(stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryImportAnchorsAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryExportAnchorsAsync<P0, P1>(anchors: P0, stream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SpatialAnchor>>>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryExportAnchorsAsync)(::windows_core::Interface::as_raw(this), anchors.try_into_param()?.abi(), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISpatialAnchorTransferManagerStatics<R, F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialAnchorTransferManager, ISpatialAnchorTransferManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialBoundingVolume(::windows_core::IUnknown);
impl SpatialBoundingVolume {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<P0>(coordinatesystem: P0, r#box: SpatialBoundingBox) -> ::windows_core::Result<SpatialBoundingVolume>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromBox)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), r#box, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<P0>(coordinatesystem: P0, r#box: SpatialBoundingOrientedBox) -> ::windows_core::Result<SpatialBoundingVolume>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromOrientedBox)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), r#box, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<P0>(coordinatesystem: P0, sphere: SpatialBoundingSphere) -> ::windows_core::Result<SpatialBoundingVolume>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromSphere)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), sphere, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<P0>(coordinatesystem: P0, frustum: SpatialBoundingFrustum) -> ::windows_core::Result<SpatialBoundingVolume>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromFrustum)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), frustum, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialBoundingVolumeStatics<R, F: FnOnce(&ISpatialBoundingVolumeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialBoundingVolume, ISpatialBoundingVolumeStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialBoundingVolume;{fb2065da-68c3-33df-b7af-4c787207999c})");
}
impl ::core::clone::Clone for SpatialBoundingVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialBoundingVolume {
    const IID: ::windows_core::GUID = <ISpatialBoundingVolume as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
::windows_core::imp::interface_hierarchy!(SpatialBoundingVolume, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialBoundingVolume {}
unsafe impl ::core::marker::Sync for SpatialBoundingVolume {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialCoordinateSystem(::windows_core::IUnknown);
impl SpatialCoordinateSystem {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetTransformTo<P0>(&self, target: P0) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetTransformTo)(::windows_core::Interface::as_raw(this), target.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialCoordinateSystem;{69ebca4b-60a3-3586-a653-59a7bd676d07})");
}
impl ::core::clone::Clone for SpatialCoordinateSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialCoordinateSystem {
    const IID: ::windows_core::GUID = <ISpatialCoordinateSystem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
::windows_core::imp::interface_hierarchy!(SpatialCoordinateSystem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialCoordinateSystem {}
unsafe impl ::core::marker::Sync for SpatialCoordinateSystem {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntity(::windows_core::IUnknown);
impl SpatialEntity {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Anchor(&self) -> ::windows_core::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Anchor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithSpatialAnchor<P0>(spatialanchor: P0) -> ::windows_core::Result<SpatialEntity>
    where
        P0: ::windows_core::IntoParam<SpatialAnchor>,
    {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSpatialAnchor)(::windows_core::Interface::as_raw(this), spatialanchor.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<P0, P1>(spatialanchor: P0, propertyset: P1) -> ::windows_core::Result<SpatialEntity>
    where
        P0: ::windows_core::IntoParam<SpatialAnchor>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSpatialAnchorAndProperties)(::windows_core::Interface::as_raw(this), spatialanchor.into_param().abi(), propertyset.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityFactory<R, F: FnOnce(&ISpatialEntityFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialEntity, ISpatialEntityFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialEntity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntity;{166de955-e1eb-454c-ba08-e6c0668ddc65})");
}
impl ::core::clone::Clone for SpatialEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntity {
    const IID: ::windows_core::GUID = <ISpatialEntity as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
::windows_core::imp::interface_hierarchy!(SpatialEntity, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntity {}
unsafe impl ::core::marker::Sync for SpatialEntity {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityAddedEventArgs(::windows_core::IUnknown);
impl SpatialEntityAddedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityAddedEventArgs;{a397f49b-156a-4707-ac2c-d31d570ed399})");
}
impl ::core::clone::Clone for SpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntityAddedEventArgs {
    const IID: ::windows_core::GUID = <ISpatialEntityAddedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpatialEntityAddedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntityAddedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityAddedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityRemovedEventArgs(::windows_core::IUnknown);
impl SpatialEntityRemovedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityRemovedEventArgs;{91741800-536d-4e9f-abf6-415b5444d651})");
}
impl ::core::clone::Clone for SpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntityRemovedEventArgs {
    const IID: ::windows_core::GUID = <ISpatialEntityRemovedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpatialEntityRemovedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntityRemovedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityRemovedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityStore(::windows_core::IUnknown);
impl SpatialEntityStore {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync<P0>(&self, entity: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<SpatialEntity>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), entity.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAsync<P0>(&self, entity: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<SpatialEntity>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAsync)(::windows_core::Interface::as_raw(this), entity.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateEntityWatcher(&self) -> ::windows_core::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateEntityWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System_RemoteSystems\"`*"]
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<P0>(session: P0) -> ::windows_core::Result<SpatialEntityStore>
    where
        P0: ::windows_core::IntoParam<super::super::System::RemoteSystems::RemoteSystemSession>,
    {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetForRemoteSystemSession)(::windows_core::Interface::as_raw(this), session.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialEntityStoreStatics<R, F: FnOnce(&ISpatialEntityStoreStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialEntityStore, ISpatialEntityStoreStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialEntityStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityStore;{329788ba-e513-4f06-889d-1be30ecf43e6})");
}
impl ::core::clone::Clone for SpatialEntityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntityStore {
    const IID: ::windows_core::GUID = <ISpatialEntityStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
::windows_core::imp::interface_hierarchy!(SpatialEntityStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntityStore {}
unsafe impl ::core::marker::Sync for SpatialEntityStore {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityUpdatedEventArgs(::windows_core::IUnknown);
impl SpatialEntityUpdatedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs;{e5671766-627b-43cb-a49f-b3be6d47deed})");
}
impl ::core::clone::Clone for SpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntityUpdatedEventArgs {
    const IID: ::windows_core::GUID = <ISpatialEntityUpdatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpatialEntityUpdatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntityUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityUpdatedEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityWatcher(::windows_core::IUnknown);
impl SpatialEntityWatcher {
    pub fn Status(&self) -> ::windows_core::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityWatcher;{b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959})");
}
impl ::core::clone::Clone for SpatialEntityWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialEntityWatcher {
    const IID: ::windows_core::GUID = <ISpatialEntityWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
::windows_core::imp::interface_hierarchy!(SpatialEntityWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialEntityWatcher {}
unsafe impl ::core::marker::Sync for SpatialEntityWatcher {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocation(::windows_core::IUnknown);
impl SpatialLocation {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteLinearVelocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteLinearAcceleration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularVelocity(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularVelocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularAcceleration(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularAcceleration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularVelocityAxisAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularAccelerationAxisAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocation;{1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2})");
}
impl ::core::clone::Clone for SpatialLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialLocation {
    const IID: ::windows_core::GUID = <ISpatialLocation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
::windows_core::imp::interface_hierarchy!(SpatialLocation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialLocation {}
unsafe impl ::core::marker::Sync for SpatialLocation {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocator(::windows_core::IUnknown);
impl SpatialLocator {
    pub fn Locatability(&self) -> ::windows_core::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Locatability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocatabilityChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocatabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLocatabilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLocatabilityChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionalTrackingDeactivating<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionalTrackingDeactivating)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionalTrackingDeactivating(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionalTrackingDeactivating)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn TryLocateAtTimestamp<P0, P1>(&self, timestamp: P0, coordinatesystem: P1) -> ::windows_core::Result<SpatialLocation>
    where
        P0: ::windows_core::IntoParam<super::PerceptionTimestamp>,
        P1: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryLocateAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), coordinatesystem.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativeposition: super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(::windows_core::Interface::as_raw(this), relativeposition, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), relativeposition, relativeorientation, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(::windows_core::Interface::as_raw(this), relativeposition, relativeorientation, relativeheadinginradians, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativeposition: super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(::windows_core::Interface::as_raw(this), relativeposition, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), relativeposition, relativeorientation, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(::windows_core::Interface::as_raw(this), relativeposition, relativeorientation, relativeheadinginradians, &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialLocatorStatics<R, F: FnOnce(&ISpatialLocatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialLocator, ISpatialLocatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialLocator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocator;{f6478925-9e0c-3bb6-997e-b64ecca24cf4})");
}
impl ::core::clone::Clone for SpatialLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialLocator {
    const IID: ::windows_core::GUID = <ISpatialLocator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
::windows_core::imp::interface_hierarchy!(SpatialLocator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialLocator {}
unsafe impl ::core::marker::Sync for SpatialLocator {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(::windows_core::IUnknown);
impl SpatialLocatorAttachedFrameOfReference {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativePosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativePosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(&self) -> ::windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeOrientation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustHeading)(::windows_core::Interface::as_raw(this), headingoffsetinradians).ok() }
    }
    pub fn GetStationaryCoordinateSystemAtTimestamp<P0>(&self, timestamp: P0) -> ::windows_core::Result<SpatialCoordinateSystem>
    where
        P0: ::windows_core::IntoParam<super::PerceptionTimestamp>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStationaryCoordinateSystemAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetRelativeHeadingAtTimestamp<P0>(&self, timestamp: P0) -> ::windows_core::Result<super::super::Foundation::IReference<f64>>
    where
        P0: ::windows_core::IntoParam<super::PerceptionTimestamp>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetRelativeHeadingAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference;{e1774ef6-1f4f-499c-9625-ef5e6ed7a048})");
}
impl ::core::clone::Clone for SpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialLocatorAttachedFrameOfReference {
    const IID: ::windows_core::GUID = <ISpatialLocatorAttachedFrameOfReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
::windows_core::imp::interface_hierarchy!(SpatialLocatorAttachedFrameOfReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialLocatorAttachedFrameOfReference {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows_core::IUnknown);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub fn Canceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanceled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanceled)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs;{b8a84063-e3f4-368b-9061-9ea9d1d6cc16})");
}
impl ::core::clone::Clone for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const IID: ::windows_core::GUID = <ISpatialLocatorPositionalTrackingDeactivatingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
::windows_core::imp::interface_hierarchy!(SpatialLocatorPositionalTrackingDeactivatingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl ::core::marker::Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(::windows_core::IUnknown);
impl SpatialStageFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MovementRange(&self) -> ::windows_core::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MovementRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LookDirectionRange(&self) -> ::windows_core::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LookDirectionRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCoordinateSystemAtCurrentLocation<P0>(&self, locator: P0) -> ::windows_core::Result<SpatialCoordinateSystem>
    where
        P0: ::windows_core::IntoParam<SpatialLocator>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCoordinateSystemAtCurrentLocation)(::windows_core::Interface::as_raw(this), locator.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<P0>(&self, coordinatesystem: P0) -> ::windows_core::Result<::windows_core::Array<super::super::Foundation::Numerics::Vector3>>
    where
        P0: ::windows_core::IntoParam<SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).TryGetMovementBounds)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), ::windows_core::Array::<super::super::Foundation::Numerics::Vector3>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn Current() -> ::windows_core::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentChanged)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestNewStageAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewStageAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialStageFrameOfReferenceStatics<R, F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpatialStageFrameOfReference, ISpatialStageFrameOfReferenceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStageFrameOfReference;{7a8a3464-ad0d-4590-ab86-33062b674926})");
}
impl ::core::clone::Clone for SpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialStageFrameOfReference {
    const IID: ::windows_core::GUID = <ISpatialStageFrameOfReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
::windows_core::imp::interface_hierarchy!(SpatialStageFrameOfReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialStageFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStageFrameOfReference {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(::windows_core::IUnknown);
impl SpatialStationaryFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStationaryFrameOfReference;{09dbccb9-bcf8-3e7f-be7e-7edccbb178a8})");
}
impl ::core::clone::Clone for SpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpatialStationaryFrameOfReference {
    const IID: ::windows_core::GUID = <ISpatialStationaryFrameOfReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
::windows_core::imp::interface_hierarchy!(SpatialStationaryFrameOfReference, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SpatialStationaryFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStationaryFrameOfReference {}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialAnchorExportPurpose {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialAnchorExportPurpose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportPurpose").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)");
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialEntityWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialEntityWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)");
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialLocatability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialLocatability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialLocatability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)");
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialLookDirectionRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialLookDirectionRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLookDirectionRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)");
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialMovementRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialMovementRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialMovementRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialMovementRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)");
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for SpatialPerceptionAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SpatialPerceptionAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPerceptionAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)");
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
impl ::windows_core::TypeKind for SpatialBoundingBox {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeType for SpatialBoundingBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
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
impl ::windows_core::TypeKind for SpatialBoundingFrustum {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))");
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
impl ::windows_core::TypeKind for SpatialBoundingOrientedBox {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))");
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
impl ::windows_core::TypeKind for SpatialBoundingSphere {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
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
impl ::core::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for SpatialRay {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::RuntimeType for SpatialRay {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
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
impl ::core::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
