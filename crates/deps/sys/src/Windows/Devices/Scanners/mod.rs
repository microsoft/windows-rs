#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IImageScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerFormatConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerScanResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerSourceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageScannerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScannerAutoConfiguration(pub *mut ::core::ffi::c_void);
pub struct ImageScannerAutoCroppingMode(i32);
pub struct ImageScannerColorMode(i32);
#[repr(transparent)]
pub struct ImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScannerFlatbedConfiguration(pub *mut ::core::ffi::c_void);
pub struct ImageScannerFormat(i32);
#[repr(transparent)]
pub struct ImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
pub struct ImageScannerResolution(i32);
#[repr(transparent)]
pub struct ImageScannerScanResult(pub *mut ::core::ffi::c_void);
pub struct ImageScannerScanSource(i32);
pub struct ScannerDeviceContract(i32);
