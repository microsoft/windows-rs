#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
#[repr(C)]
pub struct BitmapBounds(i32);
#[repr(transparent)]
pub struct BitmapBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
#[repr(transparent)]
pub struct BitmapCodecInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
#[repr(transparent)]
pub struct BitmapFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const Fant: Self = Self(3i32);
}
#[repr(transparent)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Rgba16: Self = Self(12i32);
    pub const Rgba8: Self = Self(30i32);
    pub const Gray16: Self = Self(57i32);
    pub const Gray8: Self = Self(62i32);
    pub const Bgra8: Self = Self(87i32);
    pub const Nv12: Self = Self(103i32);
    pub const P010: Self = Self(104i32);
    pub const Yuy2: Self = Self(107i32);
}
#[repr(C)]
pub struct BitmapPlaneDescription(i32);
#[repr(transparent)]
pub struct BitmapProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapPropertiesView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapPropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
#[repr(C)]
pub struct BitmapSize(i32);
#[repr(transparent)]
pub struct BitmapTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapTypedValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: Self = Self(0i32);
    pub const ColorManageToSRgb: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: Self = Self(0i32);
    pub const RespectExifOrientation: Self = Self(1i32);
}
#[repr(transparent)]
pub struct IBitmapBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapCodecInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapDecoderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapDecoderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapEncoderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapEncoderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapEncoderWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapFrameWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapPropertiesView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapTypedValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapTypedValueFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPixelDataProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISoftwareBitmapFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISoftwareBitmapStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: Self = Self(0i32);
    pub const Y4Cb2Cr0: Self = Self(1i32);
    pub const Y4Cb2Cr2: Self = Self(2i32);
    pub const Y4Cb4Cr4: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PixelDataProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Sub: Self = Self(2i32);
    pub const Up: Self = Self(3i32);
    pub const Average: Self = Self(4i32);
    pub const Paeth: Self = Self(5i32);
    pub const Adaptive: Self = Self(6i32);
}
#[repr(transparent)]
pub struct SoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Ccitt3: Self = Self(2i32);
    pub const Ccitt4: Self = Self(3i32);
    pub const Lzw: Self = Self(4i32);
    pub const Rle: Self = Self(5i32);
    pub const Zip: Self = Self(6i32);
    pub const LzwhDifferencing: Self = Self(7i32);
}
