#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchor {
    type Vtable = ISpatialAnchor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        86631886,
        7476,
        14082,
        [188, 236, 234, 191, 245, 120, 168, 105],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchor2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchor2 {
    type Vtable = ISpatialAnchor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3977758984,
        42645,
        19702,
        [146, 253, 151, 38, 59, 167, 16, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2009226027,
        13321,
        16520,
        [185, 27, 253, 253, 5, 209, 100, 143],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2586460984,
        9467,
        17001,
        [137, 197, 136, 48, 74, 238, 242, 15],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        anchor: ::windows::runtime::RawPtr,
        purpose: SpatialAnchorExportPurpose,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        anchor: ::windows::runtime::RawPtr,
        purpose: SpatialAnchorExportPurpose,
        stream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorExporterStatics {
    type Vtable = ISpatialAnchorExporterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3978627000,
        9333,
        17308,
        [133, 255, 127, 237, 52, 31, 220, 136],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorManagerStatics {
    type Vtable = ISpatialAnchorManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2296581803,
        62391,
        16907,
        [176, 134, 138, 128, 192, 125, 145, 13],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2716343992,
        22215,
        12567,
        [162, 228, 129, 224, 252, 242, 142, 0],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorStatics {
    type Vtable = ISpatialAnchorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2844952130,
        372,
        12572,
        [174, 121, 14, 81, 7, 102, 159, 22],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        position: super::super::Foundation::Numerics::Vector3,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        position: super::super::Foundation::Numerics::Vector3,
        orientation: super::super::Foundation::Numerics::Quaternion,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2965124662,
        18538,
        15536,
        [158, 111, 18, 69, 22, 92, 77, 182],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStore_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        anchor: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAnchorTransferManagerStatics {
    type Vtable = ISpatialAnchorTransferManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        62650809,
        4824,
        19406,
        [136, 53, 197, 223, 58, 192, 173, 171],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        stream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    )))]
    usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        anchors: ::windows::runtime::RawPtr,
        stream: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    )))]
    usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4213204442,
        26819,
        13279,
        [183, 175, 76, 120, 114, 7, 153, 156],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialBoundingVolumeStatics {
    type Vtable = ISpatialBoundingVolumeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        92836119,
        46049,
        14040,
        [176, 23, 86, 97, 129, 165, 177, 150],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        r#box: SpatialBoundingBox,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        r#box: SpatialBoundingOrientedBox,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        sphere: SpatialBoundingSphere,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        frustum: SpatialBoundingFrustum,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1777060427,
        24739,
        13702,
        [166, 83, 89, 167, 189, 103, 109, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntity(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntity {
    type Vtable = ISpatialEntity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        376301909,
        57835,
        17740,
        [186, 8, 230, 192, 102, 141, 220, 101],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntity_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2744644763,
        5482,
        18183,
        [172, 44, 211, 29, 87, 14, 211, 153],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityFactory {
    type Vtable = ISpatialEntityFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3790725925,
        13471,
        16933,
        [162, 243, 75, 1, 193, 95, 224, 86],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        spatialanchor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        spatialanchor: ::windows::runtime::RawPtr,
        propertyset: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2440304640,
        21357,
        20127,
        [171, 246, 65, 91, 84, 68, 214, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityStore {
    type Vtable = ISpatialEntityStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        848791738,
        58643,
        20230,
        [136, 157, 27, 227, 14, 207, 67, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStore_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        entity: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        entity: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityStoreStatics {
    type Vtable = ISpatialEntityStoreStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1800091806,
        31824,
        20114,
        [138, 98, 77, 29, 75, 124, 205, 62],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        session: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3848738662,
        25211,
        17355,
        [164, 159, 179, 190, 109, 71, 222, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3015204768,
        27998,
        19388,
        [128, 93, 95, 229, 185, 186, 25, 89],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SpatialEntityWatcherStatus,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialLocation {
    type Vtable = ISpatialLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        495047325,
        9377,
        14293,
        [143, 161, 57, 180, 249, 173, 103, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialLocation2 {
    type Vtable = ISpatialLocation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        293544982,
        14503,
        18968,
        [180, 4, 171, 143, 171, 225, 215, 139],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialLocator {
    type Vtable = ISpatialLocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4131883301,
        40460,
        15286,
        [153, 126, 182, 78, 204, 162, 76, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SpatialLocatability,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        relativeorientation: super::super::Foundation::Numerics::Quaternion,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        relativeorientation: super::super::Foundation::Numerics::Quaternion,
        relativeheadinginradians: f64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        relativeorientation: super::super::Foundation::Numerics::Quaternion,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        relativeposition: super::super::Foundation::Numerics::Vector3,
        relativeorientation: super::super::Foundation::Numerics::Quaternion,
        relativeheadinginradians: f64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3782692598,
        8015,
        18844,
        [150, 37, 239, 94, 110, 215, 160, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        headingoffsetinradians: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timestamp: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface
    for ISpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3098034275,
        58356,
        13963,
        [144, 97, 158, 169, 209, 214, 204, 22],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialLocatorStatics {
    type Vtable = ISpatialLocatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3077452608,
        42946,
        13851,
        [187, 130, 86, 233, 59, 137, 177, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2055877732,
        44301,
        17808,
        [171, 134, 51, 6, 43, 103, 73, 38],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SpatialMovementRange,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SpatialLookDirectionRange,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        locator: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coordinatesystem: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialStageFrameOfReferenceStatics {
    type Vtable = ISpatialStageFrameOfReferenceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4153236557,
        41124,
        18844,
        [141, 145, 168, 201, 101, 212, 6, 84],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cookie: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        165399737,
        48376,
        15999,
        [190, 126, 126, 220, 203, 177, 120, 168],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAnchor(::windows::runtime::IInspectable);
impl SpatialAnchor {
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn RawCoordinateSystem(&self) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RawCoordinateSystemAdjusted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialAnchor,
                SpatialAnchorRawCoordinateSystemAdjustedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRawCoordinateSystemAdjusted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TryCreateRelativeTo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
    >(
        coordinatesystem: Param0,
    ) -> ::windows::runtime::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        coordinatesystem: Param0,
        position: Param1,
    ) -> ::windows::runtime::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                position.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        coordinatesystem: Param0,
        position: Param1,
        orientation: Param2,
    ) -> ::windows::runtime::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                position.into_param().abi(),
                orientation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialAnchor>(result__)
        })
    }
    pub fn RemovedByUser(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ISpatialAnchorStatics<
        R,
        F: FnOnce(&ISpatialAnchorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialAnchor;{0529e5ce-1d34-3702-bcec-eabff578a869})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        86631886,
        7476,
        14082,
        [188, 236, 234, 191, 245, 120, 168, 105],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
impl ::std::convert::From<SpatialAnchor> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAnchor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAnchor> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAnchor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAnchor> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAnchor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAnchor> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAnchor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAnchor {}
unsafe impl ::std::marker::Sync for SpatialAnchor {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(0i32);
    pub const Sharing: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(1i32);
}
impl ::std::convert::From<i32> for SpatialAnchorExportPurpose {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialAnchorExportPurpose {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAnchorExportSufficiency(::windows::runtime::IInspectable);
impl SpatialAnchorExportSufficiency {
    pub fn IsMinimallySufficient(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SufficiencyLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn RecommendedSufficiencyLevel(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialAnchorExportSufficiency;{77c25b2b-3409-4088-b91b-fdfd05d1648f})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2009226027,
        13321,
        16520,
        [185, 27, 253, 253, 5, 209, 100, 143],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
impl ::std::convert::From<SpatialAnchorExportSufficiency> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAnchorExportSufficiency> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialAnchorExportSufficiency
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialAnchorExportSufficiency
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAnchorExportSufficiency> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAnchorExportSufficiency> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAnchorExportSufficiency
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAnchorExportSufficiency
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAnchorExportSufficiency {}
unsafe impl ::std::marker::Sync for SpatialAnchorExportSufficiency {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAnchorExporter(::windows::runtime::IInspectable);
impl SpatialAnchorExporter {
    #[cfg(feature = "Foundation")]
    pub fn GetAnchorExportSufficiencyAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialAnchor>,
    >(
        &self,
        anchor: Param0,
        purpose: SpatialAnchorExportPurpose,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                anchor.into_param().abi(),
                purpose,
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>(
                result__,
            )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryExportAnchorAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialAnchor>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>,
    >(
        &self,
        anchor: Param0,
        purpose: SpatialAnchorExportPurpose,
        stream: Param2,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                anchor.into_param().abi(),
                purpose,
                stream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::runtime::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialAnchorExporter>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>,
    > {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(
                result__,
            )
        })
    }
    pub fn ISpatialAnchorExporterStatics<
        R,
        F: FnOnce(&ISpatialAnchorExporterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAnchorExporter,
            ISpatialAnchorExporterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialAnchorExporter;{9a2a4338-24fb-4269-89c5-88304aeef20f})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2586460984,
        9467,
        17001,
        [137, 197, 136, 48, 74, 238, 242, 15],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
impl ::std::convert::From<SpatialAnchorExporter> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAnchorExporter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAnchorExporter> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAnchorExporter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialAnchorExporter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAnchorExporter> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAnchorExporter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAnchorExporter> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAnchorExporter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAnchorExporter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAnchorExporter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAnchorExporter {}
unsafe impl ::std::marker::Sync for SpatialAnchorExporter {}
pub struct SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>
    {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>(result__)
        })
    }
    pub fn ISpatialAnchorManagerStatics<
        R,
        F: FnOnce(&ISpatialAnchorManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAnchorManager,
            ISpatialAnchorManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows::runtime::IInspectable);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Matrix4x4 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs;{a1e81eb8-56c7-3117-a2e4-81e0fcf28e00})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2716343992,
        22215,
        12567,
        [162, 228, 129, 224, 252, 242, 142, 0],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str =
        "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
impl ::std::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAnchorStore(::windows::runtime::IInspectable);
impl SpatialAnchorStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, SpatialAnchor>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                SpatialAnchor,
            >>(result__)
        }
    }
    pub fn TrySave<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialAnchor>,
    >(
        &self,
        id: Param0,
        anchor: Param1,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                anchor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        id: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialAnchorStore;{b0bc3636-486a-3cb0-9e6f-1245165c4db6})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2965124662,
        18538,
        15536,
        [158, 111, 18, 69, 22, 92, 77, 182],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
impl ::std::convert::From<SpatialAnchorStore> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAnchorStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAnchorStore> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAnchorStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialAnchorStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialAnchorStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAnchorStore> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAnchorStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAnchorStore> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAnchorStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAnchorStore
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAnchorStore
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAnchorStore {}
unsafe impl ::std::marker::Sync for SpatialAnchorStore {}
pub struct SpatialAnchorTransferManager {}
impl SpatialAnchorTransferManager {
    #[cfg(feature = "deprecated")]
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    ))]
    pub fn TryImportAnchorsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>,
    >(
        stream: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                SpatialAnchor,
            >,
        >,
    > {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                stream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::Foundation::Collections::IMapView<
                    ::windows::runtime::HSTRING,
                    SpatialAnchor,
                >,
            >>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Storage_Streams"
    ))]
    pub fn TryExportAnchorsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    SpatialAnchor,
                >,
            >,
        >,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream>,
    >(
        anchors: Param0,
        stream: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                anchors.into_param().abi(),
                stream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>,
    > {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(
                result__,
            )
        })
    }
    pub fn ISpatialAnchorTransferManagerStatics<
        R,
        F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAnchorTransferManager,
            ISpatialAnchorTransferManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::default::Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::fmt::Debug for SpatialBoundingBox {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SpatialBoundingBox")
            .field("Center", &self.Center)
            .field("Extents", &self.Extents)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::PartialEq for SpatialBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Extents == other.Extents
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::Eq for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for SpatialBoundingBox {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for SpatialBoundingBox {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))" ) ;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::fmt::Debug for SpatialBoundingFrustum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SpatialBoundingFrustum")
            .field("Near", &self.Near)
            .field("Far", &self.Far)
            .field("Right", &self.Right)
            .field("Left", &self.Left)
            .field("Top", &self.Top)
            .field("Bottom", &self.Bottom)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::PartialEq for SpatialBoundingFrustum {
    fn eq(&self, other: &Self) -> bool {
        self.Near == other.Near
            && self.Far == other.Far
            && self.Right == other.Right
            && self.Left == other.Left
            && self.Top == other.Top
            && self.Bottom == other.Bottom
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::Eq for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for SpatialBoundingFrustum {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))" ) ;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::fmt::Debug for SpatialBoundingOrientedBox {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SpatialBoundingOrientedBox")
            .field("Center", &self.Center)
            .field("Extents", &self.Extents)
            .field("Orientation", &self.Orientation)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::PartialEq for SpatialBoundingOrientedBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center
            && self.Extents == other.Extents
            && self.Orientation == other.Orientation
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::Eq for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for SpatialBoundingOrientedBox {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))" ) ;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::fmt::Debug for SpatialBoundingSphere {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SpatialBoundingSphere")
            .field("Center", &self.Center)
            .field("Radius", &self.Radius)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::PartialEq for SpatialBoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Radius == other.Radius
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::Eq for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for SpatialBoundingSphere {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)" ) ;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialBoundingVolume(::windows::runtime::IInspectable);
impl SpatialBoundingVolume {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialBoundingBox>,
    >(
        coordinatesystem: Param0,
        r#box: Param1,
    ) -> ::windows::runtime::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                r#box.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialBoundingOrientedBox>,
    >(
        coordinatesystem: Param0,
        r#box: Param1,
    ) -> ::windows::runtime::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                r#box.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialBoundingSphere>,
    >(
        coordinatesystem: Param0,
        sphere: Param1,
    ) -> ::windows::runtime::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                sphere.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialBoundingFrustum>,
    >(
        coordinatesystem: Param0,
        frustum: Param1,
    ) -> ::windows::runtime::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                coordinatesystem.into_param().abi(),
                frustum.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    pub fn ISpatialBoundingVolumeStatics<
        R,
        F: FnOnce(&ISpatialBoundingVolumeStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialBoundingVolume,
            ISpatialBoundingVolumeStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialBoundingVolume;{fb2065da-68c3-33df-b7af-4c787207999c})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4213204442,
        26819,
        13279,
        [183, 175, 76, 120, 114, 7, 153, 156],
    );
}
impl ::windows::runtime::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
impl ::std::convert::From<SpatialBoundingVolume> for ::windows::runtime::IUnknown {
    fn from(value: SpatialBoundingVolume) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialBoundingVolume> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialBoundingVolume) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialBoundingVolume
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialBoundingVolume> for ::windows::runtime::IInspectable {
    fn from(value: SpatialBoundingVolume) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialBoundingVolume> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialBoundingVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialBoundingVolume
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialBoundingVolume
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialBoundingVolume {}
unsafe impl ::std::marker::Sync for SpatialBoundingVolume {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialCoordinateSystem(::windows::runtime::IInspectable);
impl SpatialCoordinateSystem {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn TryGetTransformTo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
    >(
        &self,
        target: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<
                super::super::Foundation::Numerics::Matrix4x4,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialCoordinateSystem;{69ebca4b-60a3-3586-a653-59a7bd676d07})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1777060427,
        24739,
        13702,
        [166, 83, 89, 167, 189, 103, 109, 7],
    );
}
impl ::windows::runtime::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
impl ::std::convert::From<SpatialCoordinateSystem> for ::windows::runtime::IUnknown {
    fn from(value: SpatialCoordinateSystem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialCoordinateSystem> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialCoordinateSystem
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialCoordinateSystem
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialCoordinateSystem> for ::windows::runtime::IInspectable {
    fn from(value: SpatialCoordinateSystem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialCoordinateSystem> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialCoordinateSystem
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialCoordinateSystem
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialCoordinateSystem {}
unsafe impl ::std::marker::Sync for SpatialCoordinateSystem {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntity(::windows::runtime::IInspectable);
impl SpatialEntity {
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Anchor(&self) -> ::windows::runtime::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialAnchor>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn CreateWithSpatialAnchor<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialAnchor>>(
        spatialanchor: Param0,
    ) -> ::windows::runtime::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                spatialanchor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialEntity>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialAnchor>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
    >(
        spatialanchor: Param0,
        propertyset: Param1,
    ) -> ::windows::runtime::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                spatialanchor.into_param().abi(),
                propertyset.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialEntity>(result__)
        })
    }
    pub fn ISpatialEntityFactory<
        R,
        F: FnOnce(&ISpatialEntityFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialEntity, ISpatialEntityFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialEntity;{166de955-e1eb-454c-ba08-e6c0668ddc65})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        376301909,
        57835,
        17740,
        [186, 8, 230, 192, 102, 141, 220, 101],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
impl ::std::convert::From<SpatialEntity> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntity> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntity> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntity) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntity> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SpatialEntity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntity {}
unsafe impl ::std::marker::Sync for SpatialEntity {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntityAddedEventArgs(::windows::runtime::IInspectable);
impl SpatialEntityAddedEventArgs {
    pub fn Entity(&self) -> ::windows::runtime::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialEntityAddedEventArgs;{a397f49b-156a-4707-ac2c-d31d570ed399})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2744644763,
        5482,
        18183,
        [172, 44, 211, 29, 87, 14, 211, 153],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
impl ::std::convert::From<SpatialEntityAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntityAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialEntityAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialEntityAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntityAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntityAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialEntityAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialEntityAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntityAddedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialEntityAddedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntityRemovedEventArgs(::windows::runtime::IInspectable);
impl SpatialEntityRemovedEventArgs {
    pub fn Entity(&self) -> ::windows::runtime::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialEntityRemovedEventArgs;{91741800-536d-4e9f-abf6-415b5444d651})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2440304640,
        21357,
        20127,
        [171, 246, 65, 91, 84, 68, 214, 81],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
impl ::std::convert::From<SpatialEntityRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialEntityRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialEntityRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntityRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntityRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialEntityRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialEntityRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntityRemovedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialEntityRemovedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntityStore(::windows::runtime::IInspectable);
impl SpatialEntityStore {
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialEntity>>(
        &self,
        entity: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                entity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, SpatialEntity>>(
        &self,
        entity: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                entity.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateEntityWatcher(&self) -> ::windows::runtime::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialEntityWatcher>(result__)
        }
    }
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::RemoteSystems::RemoteSystemSession>,
    >(
        session: Param0,
    ) -> ::windows::runtime::Result<SpatialEntityStore> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                session.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialEntityStore>(result__)
        })
    }
    pub fn ISpatialEntityStoreStatics<
        R,
        F: FnOnce(&ISpatialEntityStoreStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialEntityStore,
            ISpatialEntityStoreStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialEntityStore;{329788ba-e513-4f06-889d-1be30ecf43e6})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        848791738,
        58643,
        20230,
        [136, 157, 27, 227, 14, 207, 67, 230],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
impl ::std::convert::From<SpatialEntityStore> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntityStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntityStore> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntityStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialEntityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialEntityStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntityStore> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntityStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntityStore> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntityStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialEntityStore
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialEntityStore
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntityStore {}
unsafe impl ::std::marker::Sync for SpatialEntityStore {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntityUpdatedEventArgs(::windows::runtime::IInspectable);
impl SpatialEntityUpdatedEventArgs {
    pub fn Entity(&self) -> ::windows::runtime::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialEntity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs;{e5671766-627b-43cb-a49f-b3be6d47deed})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3848738662,
        25211,
        17355,
        [164, 159, 179, 190, 109, 71, 222, 237],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
impl ::std::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialEntityUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialEntityUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntityUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialEntityUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialEntityUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntityUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for SpatialEntityUpdatedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialEntityWatcher(::windows::runtime::IInspectable);
impl SpatialEntityWatcher {
    pub fn Status(&self) -> ::windows::runtime::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: SpatialEntityWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialEntityWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialEntityWatcher,
                SpatialEntityAddedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialEntityWatcher,
                SpatialEntityUpdatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialEntityWatcher,
                SpatialEntityRemovedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialEntityWatcher,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialEntityWatcher;{b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3015204768,
        27998,
        19388,
        [128, 93, 95, 229, 185, 186, 25, 89],
    );
}
impl ::windows::runtime::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
impl ::std::convert::From<SpatialEntityWatcher> for ::windows::runtime::IUnknown {
    fn from(value: SpatialEntityWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialEntityWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialEntityWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialEntityWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialEntityWatcher> for ::windows::runtime::IInspectable {
    fn from(value: SpatialEntityWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialEntityWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialEntityWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialEntityWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialEntityWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialEntityWatcher {}
unsafe impl ::std::marker::Sync for SpatialEntityWatcher {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
impl ::std::convert::From<i32> for SpatialEntityWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialEntityWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: SpatialLocatability = SpatialLocatability(0i32);
    pub const OrientationOnly: SpatialLocatability = SpatialLocatability(1i32);
    pub const PositionalTrackingActivating: SpatialLocatability = SpatialLocatability(2i32);
    pub const PositionalTrackingActive: SpatialLocatability = SpatialLocatability(3i32);
    pub const PositionalTrackingInhibited: SpatialLocatability = SpatialLocatability(4i32);
}
impl ::std::convert::From<i32> for SpatialLocatability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialLocatability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialLocatability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialLocation(::windows::runtime::IInspectable);
impl SpatialLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocity(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAcceleration(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialLocation;{1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        495047325,
        9377,
        14293,
        [143, 161, 57, 180, 249, 173, 103, 226],
    );
}
impl ::windows::runtime::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
impl ::std::convert::From<SpatialLocation> for ::windows::runtime::IUnknown {
    fn from(value: SpatialLocation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialLocation> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialLocation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialLocation> for ::windows::runtime::IInspectable {
    fn from(value: SpatialLocation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialLocation> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialLocation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialLocation {}
unsafe impl ::std::marker::Sync for SpatialLocation {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialLocator(::windows::runtime::IInspectable);
impl SpatialLocator {
    pub fn Locatability(&self) -> ::windows::runtime::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__: SpatialLocatability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialLocatability>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LocatabilityChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialLocator,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLocatabilityChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionalTrackingDeactivating<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialLocator,
                SpatialLocatorPositionalTrackingDeactivatingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionalTrackingDeactivating<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TryLocateAtTimestamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::PerceptionTimestamp>,
        Param1: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
    >(
        &self,
        timestamp: Param0,
        coordinatesystem: Param1,
    ) -> ::windows::runtime::Result<SpatialLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                timestamp.into_param().abi(),
                coordinatesystem.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialLocation>(result__)
        }
    }
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(
        &self,
    ) -> ::windows::runtime::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        relativeposition: Param0,
    ) -> ::windows::runtime::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        relativeposition: Param0,
        relativeorientation: Param1,
    ) -> ::windows::runtime::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                relativeorientation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        relativeposition: Param0,
        relativeorientation: Param1,
        relativeheadinginradians: f64,
    ) -> ::windows::runtime::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                relativeorientation.into_param().abi(),
                relativeheadinginradians,
                &mut result__,
            )
            .from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(
        &self,
    ) -> ::windows::runtime::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        relativeposition: Param0,
    ) -> ::windows::runtime::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        relativeposition: Param0,
        relativeorientation: Param1,
    ) -> ::windows::runtime::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                relativeorientation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        relativeposition: Param0,
        relativeorientation: Param1,
        relativeheadinginradians: f64,
    ) -> ::windows::runtime::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                relativeposition.into_param().abi(),
                relativeorientation.into_param().abi(),
                relativeheadinginradians,
                &mut result__,
            )
            .from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::runtime::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialLocator>(result__)
        })
    }
    pub fn ISpatialLocatorStatics<
        R,
        F: FnOnce(&ISpatialLocatorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialLocator,
            ISpatialLocatorStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialLocator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Perception.Spatial.SpatialLocator;{f6478925-9e0c-3bb6-997e-b64ecca24cf4})",
    );
}
unsafe impl ::windows::runtime::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4131883301,
        40460,
        15286,
        [153, 126, 182, 78, 204, 162, 76, 244],
    );
}
impl ::windows::runtime::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
impl ::std::convert::From<SpatialLocator> for ::windows::runtime::IUnknown {
    fn from(value: SpatialLocator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialLocator> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialLocator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SpatialLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SpatialLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialLocator> for ::windows::runtime::IInspectable {
    fn from(value: SpatialLocator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialLocator> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialLocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SpatialLocator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialLocator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialLocator {}
unsafe impl ::std::marker::Sync for SpatialLocator {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialLocatorAttachedFrameOfReference(::windows::runtime::IInspectable);
impl SpatialLocatorAttachedFrameOfReference {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                headingoffsetinradians,
            )
            .ok()
        }
    }
    pub fn GetStationaryCoordinateSystemAtTimestamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::PerceptionTimestamp>,
    >(
        &self,
        timestamp: Param0,
    ) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                timestamp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryGetRelativeHeadingAtTimestamp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::PerceptionTimestamp>,
    >(
        &self,
        timestamp: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                timestamp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference;{e1774ef6-1f4f-499c-9625-ef5e6ed7a048})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3782692598,
        8015,
        18844,
        [150, 37, 239, 94, 110, 215, 160, 72],
    );
}
impl ::windows::runtime::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
impl ::std::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows::runtime::IUnknown {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialLocatorAttachedFrameOfReference>
    for ::windows::runtime::IUnknown
{
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialLocatorAttachedFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialLocatorAttachedFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialLocatorAttachedFrameOfReference>
    for ::windows::runtime::IInspectable
{
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialLocatorAttachedFrameOfReference>
    for ::windows::runtime::IInspectable
{
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialLocatorAttachedFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialLocatorAttachedFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl ::std::marker::Sync for SpatialLocatorAttachedFrameOfReference {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows::runtime::IInspectable);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub fn Canceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanceled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType
    for SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs;{b8a84063-e3f4-368b-9061-9ea9d1d6cc16})" ) ;
}
unsafe impl ::windows::runtime::Interface
    for SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3098034275,
        58356,
        13963,
        [144, 97, 158, 169, 209, 214, 204, 22],
    );
}
impl ::windows::runtime::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str =
        "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
impl ::std::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl ::std::marker::Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: SpatialLookDirectionRange = SpatialLookDirectionRange(0i32);
    pub const Omnidirectional: SpatialLookDirectionRange = SpatialLookDirectionRange(1i32);
}
impl ::std::convert::From<i32> for SpatialLookDirectionRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialLookDirectionRange {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: SpatialMovementRange = SpatialMovementRange(0i32);
    pub const Bounded: SpatialMovementRange = SpatialMovementRange(1i32);
}
impl ::std::convert::From<i32> for SpatialMovementRange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialMovementRange {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialMovementRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(0i32);
    pub const Allowed: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(1i32);
    pub const DeniedByUser: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(2i32);
    pub const DeniedBySystem: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(3i32);
}
impl ::std::convert::From<i32> for SpatialPerceptionAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialPerceptionAccessStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::fmt::Debug for SpatialRay {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SpatialRay")
            .field("Origin", &self.Origin)
            .field("Direction", &self.Direction)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::PartialEq for SpatialRay {
    fn eq(&self, other: &Self) -> bool {
        self.Origin == other.Origin && self.Direction == other.Direction
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::std::cmp::Eq for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for SpatialRay {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for SpatialRay {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))" ) ;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialStageFrameOfReference(::windows::runtime::IInspectable);
impl SpatialStageFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn MovementRange(&self) -> ::windows::runtime::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialMovementRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialMovementRange>(result__)
        }
    }
    pub fn LookDirectionRange(&self) -> ::windows::runtime::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__: SpatialLookDirectionRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialLookDirectionRange>(result__)
        }
    }
    pub fn GetCoordinateSystemAtCurrentLocation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialLocator>,
    >(
        &self,
        locator: Param0,
    ) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                locator.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SpatialCoordinateSystem>,
    >(
        &self,
        coordinatesystem: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::runtime::Array<super::super::Foundation::Numerics::Vector3>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<
                super::super::Foundation::Numerics::Vector3,
            > = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .10 ) ( :: std :: mem :: transmute_copy ( this ) , coordinatesystem . into_param ( ) . abi ( ) , :: windows :: runtime :: Array :: < super::super::Foundation::Numerics:: Vector3 > :: set_abi_len ( & mut result__ ) , & mut result__ as * mut _ as _ ) . and_then ( || result__ )
        }
    }
    pub fn Current() -> ::windows::runtime::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialStageFrameOfReference>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestNewStageAsync() -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>,
    > {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>(
                result__,
            )
        })
    }
    pub fn ISpatialStageFrameOfReferenceStatics<
        R,
        F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialStageFrameOfReference,
            ISpatialStageFrameOfReferenceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialStageFrameOfReference;{7a8a3464-ad0d-4590-ab86-33062b674926})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2055877732,
        44301,
        17808,
        [171, 134, 51, 6, 43, 103, 73, 38],
    );
}
impl ::windows::runtime::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
impl ::std::convert::From<SpatialStageFrameOfReference> for ::windows::runtime::IUnknown {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialStageFrameOfReference> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialStageFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialStageFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialStageFrameOfReference> for ::windows::runtime::IInspectable {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialStageFrameOfReference> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialStageFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialStageFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialStageFrameOfReference {}
unsafe impl ::std::marker::Sync for SpatialStageFrameOfReference {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialStationaryFrameOfReference(::windows::runtime::IInspectable);
impl SpatialStationaryFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Perception.Spatial.SpatialStationaryFrameOfReference;{09dbccb9-bcf8-3e7f-be7e-7edccbb178a8})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        165399737,
        48376,
        15999,
        [190, 126, 126, 220, 203, 177, 120, 168],
    );
}
impl ::windows::runtime::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
impl ::std::convert::From<SpatialStationaryFrameOfReference> for ::windows::runtime::IUnknown {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialStationaryFrameOfReference> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialStationaryFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialStationaryFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialStationaryFrameOfReference> for ::windows::runtime::IInspectable {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialStationaryFrameOfReference> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialStationaryFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialStationaryFrameOfReference
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialStationaryFrameOfReference {}
unsafe impl ::std::marker::Sync for SpatialStationaryFrameOfReference {}
