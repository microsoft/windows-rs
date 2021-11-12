#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: BitmapAlphaMode = BitmapAlphaMode(0i32);
    pub const Straight: BitmapAlphaMode = BitmapAlphaMode(1i32);
    pub const Ignore: BitmapAlphaMode = BitmapAlphaMode(2i32);
}
#[repr(C)]
pub struct BitmapBounds(i32);
#[repr(transparent)]
pub struct BitmapBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: BitmapBufferAccessMode = BitmapBufferAccessMode(0i32);
    pub const ReadWrite: BitmapBufferAccessMode = BitmapBufferAccessMode(1i32);
    pub const Write: BitmapBufferAccessMode = BitmapBufferAccessMode(2i32);
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
    pub const None: BitmapFlip = BitmapFlip(0i32);
    pub const Horizontal: BitmapFlip = BitmapFlip(1i32);
    pub const Vertical: BitmapFlip = BitmapFlip(2i32);
}
#[repr(transparent)]
pub struct BitmapFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: BitmapInterpolationMode = BitmapInterpolationMode(0i32);
    pub const Linear: BitmapInterpolationMode = BitmapInterpolationMode(1i32);
    pub const Cubic: BitmapInterpolationMode = BitmapInterpolationMode(2i32);
    pub const Fant: BitmapInterpolationMode = BitmapInterpolationMode(3i32);
}
#[repr(transparent)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: BitmapPixelFormat = BitmapPixelFormat(0i32);
    pub const Rgba16: BitmapPixelFormat = BitmapPixelFormat(12i32);
    pub const Rgba8: BitmapPixelFormat = BitmapPixelFormat(30i32);
    pub const Gray16: BitmapPixelFormat = BitmapPixelFormat(57i32);
    pub const Gray8: BitmapPixelFormat = BitmapPixelFormat(62i32);
    pub const Bgra8: BitmapPixelFormat = BitmapPixelFormat(87i32);
    pub const Nv12: BitmapPixelFormat = BitmapPixelFormat(103i32);
    pub const P010: BitmapPixelFormat = BitmapPixelFormat(104i32);
    pub const Yuy2: BitmapPixelFormat = BitmapPixelFormat(107i32);
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
    pub const None: BitmapRotation = BitmapRotation(0i32);
    pub const Clockwise90Degrees: BitmapRotation = BitmapRotation(1i32);
    pub const Clockwise180Degrees: BitmapRotation = BitmapRotation(2i32);
    pub const Clockwise270Degrees: BitmapRotation = BitmapRotation(3i32);
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
    pub const DoNotColorManage: ColorManagementMode = ColorManagementMode(0i32);
    pub const ColorManageToSRgb: ColorManagementMode = ColorManagementMode(1i32);
}
#[repr(transparent)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: ExifOrientationMode = ExifOrientationMode(0i32);
    pub const RespectExifOrientation: ExifOrientationMode = ExifOrientationMode(1i32);
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
    pub const Default: JpegSubsamplingMode = JpegSubsamplingMode(0i32);
    pub const Y4Cb2Cr0: JpegSubsamplingMode = JpegSubsamplingMode(1i32);
    pub const Y4Cb2Cr2: JpegSubsamplingMode = JpegSubsamplingMode(2i32);
    pub const Y4Cb4Cr4: JpegSubsamplingMode = JpegSubsamplingMode(3i32);
}
#[repr(transparent)]
pub struct PixelDataProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: PngFilterMode = PngFilterMode(0i32);
    pub const None: PngFilterMode = PngFilterMode(1i32);
    pub const Sub: PngFilterMode = PngFilterMode(2i32);
    pub const Up: PngFilterMode = PngFilterMode(3i32);
    pub const Average: PngFilterMode = PngFilterMode(4i32);
    pub const Paeth: PngFilterMode = PngFilterMode(5i32);
    pub const Adaptive: PngFilterMode = PngFilterMode(6i32);
}
#[repr(transparent)]
pub struct SoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: TiffCompressionMode = TiffCompressionMode(0i32);
    pub const None: TiffCompressionMode = TiffCompressionMode(1i32);
    pub const Ccitt3: TiffCompressionMode = TiffCompressionMode(2i32);
    pub const Ccitt4: TiffCompressionMode = TiffCompressionMode(3i32);
    pub const Lzw: TiffCompressionMode = TiffCompressionMode(4i32);
    pub const Rle: TiffCompressionMode = TiffCompressionMode(5i32);
    pub const Zip: TiffCompressionMode = TiffCompressionMode(6i32);
    pub const LzwhDifferencing: TiffCompressionMode = TiffCompressionMode(7i32);
}
