#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
#[repr(transparent)]
pub struct DeliveryOptimizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeliveryOptimizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeliveryOptimizationSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreConfigurationStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStoreHardwareManufacturerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewProductInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreHardwareManufacturerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
#[repr(transparent)]
pub struct StorePreviewProductInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
#[repr(transparent)]
pub struct StorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
