#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(0i32);
    pub const SingleRegion: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(1i32);
    pub const MultipleRegion: ImageScannerAutoCroppingMode = ImageScannerAutoCroppingMode(2i32);
}
#[repr(transparent)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: ImageScannerColorMode = ImageScannerColorMode(0i32);
    pub const Grayscale: ImageScannerColorMode = ImageScannerColorMode(1i32);
    pub const Monochrome: ImageScannerColorMode = ImageScannerColorMode(2i32);
    pub const AutoColor: ImageScannerColorMode = ImageScannerColorMode(3i32);
}
#[repr(transparent)]
pub struct ImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScannerFlatbedConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: ImageScannerFormat = ImageScannerFormat(0i32);
    pub const Png: ImageScannerFormat = ImageScannerFormat(1i32);
    pub const DeviceIndependentBitmap: ImageScannerFormat = ImageScannerFormat(2i32);
    pub const Tiff: ImageScannerFormat = ImageScannerFormat(3i32);
    pub const Xps: ImageScannerFormat = ImageScannerFormat(4i32);
    pub const OpenXps: ImageScannerFormat = ImageScannerFormat(5i32);
    pub const Pdf: ImageScannerFormat = ImageScannerFormat(6i32);
}
#[repr(transparent)]
pub struct ImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ImageScannerResolution(i32);
#[repr(transparent)]
pub struct ImageScannerScanResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: ImageScannerScanSource = ImageScannerScanSource(0i32);
    pub const Flatbed: ImageScannerScanSource = ImageScannerScanSource(1i32);
    pub const Feeder: ImageScannerScanSource = ImageScannerScanSource(2i32);
    pub const AutoConfigured: ImageScannerScanSource = ImageScannerScanSource(3i32);
}
#[repr(C)]
pub struct ScannerDeviceContract(i32);
