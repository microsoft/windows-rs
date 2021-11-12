#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(0i32);
    pub const HttpOnly: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(1i32);
    pub const Lan: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(2i32);
    pub const Group: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(3i32);
    pub const Internet: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(4i32);
    pub const Bypass: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(5i32);
}
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: DeliveryOptimizationDownloadModeSource = DeliveryOptimizationDownloadModeSource(0i32);
    pub const Policy: DeliveryOptimizationDownloadModeSource = DeliveryOptimizationDownloadModeSource(1i32);
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
    pub const None: StoreLogOptions = StoreLogOptions(0u32);
    pub const TryElevate: StoreLogOptions = StoreLogOptions(1u32);
}
#[repr(transparent)]
pub struct StorePreviewProductInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(0i32);
    pub const AlreadyPurchased: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(1i32);
    pub const NotFulfilled: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(2i32);
    pub const NotPurchased: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(3i32);
}
#[repr(transparent)]
pub struct StorePreviewPurchaseResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorePreviewSkuInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: StoreSystemFeature = StoreSystemFeature(0i32);
    pub const ArchitectureX64: StoreSystemFeature = StoreSystemFeature(1i32);
    pub const ArchitectureArm: StoreSystemFeature = StoreSystemFeature(2i32);
    pub const DirectX9: StoreSystemFeature = StoreSystemFeature(3i32);
    pub const DirectX10: StoreSystemFeature = StoreSystemFeature(4i32);
    pub const DirectX11: StoreSystemFeature = StoreSystemFeature(5i32);
    pub const D3D12HardwareFL11: StoreSystemFeature = StoreSystemFeature(6i32);
    pub const D3D12HardwareFL12: StoreSystemFeature = StoreSystemFeature(7i32);
    pub const Memory300MB: StoreSystemFeature = StoreSystemFeature(8i32);
    pub const Memory750MB: StoreSystemFeature = StoreSystemFeature(9i32);
    pub const Memory1GB: StoreSystemFeature = StoreSystemFeature(10i32);
    pub const Memory2GB: StoreSystemFeature = StoreSystemFeature(11i32);
    pub const CameraFront: StoreSystemFeature = StoreSystemFeature(12i32);
    pub const CameraRear: StoreSystemFeature = StoreSystemFeature(13i32);
    pub const Gyroscope: StoreSystemFeature = StoreSystemFeature(14i32);
    pub const Hover: StoreSystemFeature = StoreSystemFeature(15i32);
    pub const Magnetometer: StoreSystemFeature = StoreSystemFeature(16i32);
    pub const Nfc: StoreSystemFeature = StoreSystemFeature(17i32);
    pub const Resolution720P: StoreSystemFeature = StoreSystemFeature(18i32);
    pub const ResolutionWvga: StoreSystemFeature = StoreSystemFeature(19i32);
    pub const ResolutionWvgaOr720P: StoreSystemFeature = StoreSystemFeature(20i32);
    pub const ResolutionWxga: StoreSystemFeature = StoreSystemFeature(21i32);
    pub const ResolutionWvgaOrWxga: StoreSystemFeature = StoreSystemFeature(22i32);
    pub const ResolutionWxgaOr720P: StoreSystemFeature = StoreSystemFeature(23i32);
    pub const Memory4GB: StoreSystemFeature = StoreSystemFeature(24i32);
    pub const Memory6GB: StoreSystemFeature = StoreSystemFeature(25i32);
    pub const Memory8GB: StoreSystemFeature = StoreSystemFeature(26i32);
    pub const Memory12GB: StoreSystemFeature = StoreSystemFeature(27i32);
    pub const Memory16GB: StoreSystemFeature = StoreSystemFeature(28i32);
    pub const Memory20GB: StoreSystemFeature = StoreSystemFeature(29i32);
    pub const VideoMemory2GB: StoreSystemFeature = StoreSystemFeature(30i32);
    pub const VideoMemory4GB: StoreSystemFeature = StoreSystemFeature(31i32);
    pub const VideoMemory6GB: StoreSystemFeature = StoreSystemFeature(32i32);
    pub const VideoMemory1GB: StoreSystemFeature = StoreSystemFeature(33i32);
    pub const ArchitectureArm64: StoreSystemFeature = StoreSystemFeature(34i32);
}
