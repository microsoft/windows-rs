#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IImageScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScanner {}
impl ::core::clone::Clone for IImageScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerFeederConfiguration {}
impl ::core::clone::Clone for IImageScannerFeederConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerFormatConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerFormatConfiguration {}
impl ::core::clone::Clone for IImageScannerFormatConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerPreviewResult {}
impl ::core::clone::Clone for IImageScannerPreviewResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerScanResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerScanResult {}
impl ::core::clone::Clone for IImageScannerScanResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerSourceConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerSourceConfiguration {}
impl ::core::clone::Clone for IImageScannerSourceConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageScannerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageScannerStatics {}
impl ::core::clone::Clone for IImageScannerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScanner {}
impl ::core::clone::Clone for ImageScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerAutoConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScannerAutoConfiguration {}
impl ::core::clone::Clone for ImageScannerAutoConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerAutoCroppingMode(pub i32);
impl ImageScannerAutoCroppingMode {
    pub const Disabled: Self = Self(0i32);
    pub const SingleRegion: Self = Self(1i32);
    pub const MultipleRegion: Self = Self(2i32);
}
impl ::core::marker::Copy for ImageScannerAutoCroppingMode {}
impl ::core::clone::Clone for ImageScannerAutoCroppingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerColorMode(pub i32);
impl ImageScannerColorMode {
    pub const Color: Self = Self(0i32);
    pub const Grayscale: Self = Self(1i32);
    pub const Monochrome: Self = Self(2i32);
    pub const AutoColor: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerColorMode {}
impl ::core::clone::Clone for ImageScannerColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerFeederConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScannerFeederConfiguration {}
impl ::core::clone::Clone for ImageScannerFeederConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerFlatbedConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScannerFlatbedConfiguration {}
impl ::core::clone::Clone for ImageScannerFlatbedConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerFormat(pub i32);
impl ImageScannerFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const DeviceIndependentBitmap: Self = Self(2i32);
    pub const Tiff: Self = Self(3i32);
    pub const Xps: Self = Self(4i32);
    pub const OpenXps: Self = Self(5i32);
    pub const Pdf: Self = Self(6i32);
}
impl ::core::marker::Copy for ImageScannerFormat {}
impl ::core::clone::Clone for ImageScannerFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerPreviewResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScannerPreviewResult {}
impl ::core::clone::Clone for ImageScannerPreviewResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ImageScannerResolution {
    pub DpiX: f32,
    pub DpiY: f32,
}
impl ::core::marker::Copy for ImageScannerResolution {}
impl ::core::clone::Clone for ImageScannerResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerScanResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageScannerScanResult {}
impl ::core::clone::Clone for ImageScannerScanResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageScannerScanSource(pub i32);
impl ImageScannerScanSource {
    pub const Default: Self = Self(0i32);
    pub const Flatbed: Self = Self(1i32);
    pub const Feeder: Self = Self(2i32);
    pub const AutoConfigured: Self = Self(3i32);
}
impl ::core::marker::Copy for ImageScannerScanSource {}
impl ::core::clone::Clone for ImageScannerScanSource {
    fn clone(&self) -> Self {
        *self
    }
}
