#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IImageScanner(pub *mut ::core::ffi::c_void);
pub struct IImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
pub struct IImageScannerFormatConfiguration(pub *mut ::core::ffi::c_void);
pub struct IImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
pub struct IImageScannerScanResult(pub *mut ::core::ffi::c_void);
pub struct IImageScannerSourceConfiguration(pub *mut ::core::ffi::c_void);
pub struct IImageScannerStatics(pub *mut ::core::ffi::c_void);
pub struct ImageScanner(i32);
pub struct ImageScannerAutoConfiguration(i32);
pub struct ImageScannerAutoCroppingMode(i32);
pub struct ImageScannerColorMode(i32);
pub struct ImageScannerFeederConfiguration(i32);
pub struct ImageScannerFlatbedConfiguration(i32);
pub struct ImageScannerFormat(i32);
pub struct ImageScannerPreviewResult(i32);
pub struct ImageScannerResolution(i32);
pub struct ImageScannerScanResult(i32);
pub struct ImageScannerScanSource(i32);
pub struct ScannerDeviceContract(i32);
