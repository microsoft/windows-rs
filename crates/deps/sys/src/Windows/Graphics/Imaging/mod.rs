#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapAlphaMode {}
impl ::core::clone::Clone for BitmapAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BitmapBounds {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapBounds {}
impl ::core::clone::Clone for BitmapBounds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapBuffer {}
impl ::core::clone::Clone for BitmapBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapBufferAccessMode {}
impl ::core::clone::Clone for BitmapBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapCodecInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapCodecInformation {}
impl ::core::clone::Clone for BitmapCodecInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapDecoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapDecoder {}
impl ::core::clone::Clone for BitmapDecoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapEncoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapEncoder {}
impl ::core::clone::Clone for BitmapEncoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapFlip {}
impl ::core::clone::Clone for BitmapFlip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapFrame {}
impl ::core::clone::Clone for BitmapFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const Fant: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapInterpolationMode {}
impl ::core::clone::Clone for BitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for BitmapPixelFormat {}
impl ::core::clone::Clone for BitmapPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BitmapPlaneDescription {
    pub StartIndex: i32,
    pub Width: i32,
    pub Height: i32,
    pub Stride: i32,
}
impl ::core::marker::Copy for BitmapPlaneDescription {}
impl ::core::clone::Clone for BitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapProperties {}
impl ::core::clone::Clone for BitmapProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapPropertiesView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapPropertiesView {}
impl ::core::clone::Clone for BitmapPropertiesView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapPropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapPropertySet {}
impl ::core::clone::Clone for BitmapPropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapRotation {}
impl ::core::clone::Clone for BitmapRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BitmapSize {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapSize {}
impl ::core::clone::Clone for BitmapSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapTransform {}
impl ::core::clone::Clone for BitmapTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapTypedValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapTypedValue {}
impl ::core::clone::Clone for BitmapTypedValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: Self = Self(0i32);
    pub const ColorManageToSRgb: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorManagementMode {}
impl ::core::clone::Clone for ColorManagementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: Self = Self(0i32);
    pub const RespectExifOrientation: Self = Self(1i32);
}
impl ::core::marker::Copy for ExifOrientationMode {}
impl ::core::clone::Clone for ExifOrientationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapBuffer {}
impl ::core::clone::Clone for IBitmapBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapCodecInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapCodecInformation {}
impl ::core::clone::Clone for IBitmapCodecInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapDecoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapDecoder {}
impl ::core::clone::Clone for IBitmapDecoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapDecoderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapDecoderStatics {}
impl ::core::clone::Clone for IBitmapDecoderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapDecoderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapDecoderStatics2 {}
impl ::core::clone::Clone for IBitmapDecoderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapEncoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapEncoder {}
impl ::core::clone::Clone for IBitmapEncoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapEncoderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapEncoderStatics {}
impl ::core::clone::Clone for IBitmapEncoderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapEncoderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapEncoderStatics2 {}
impl ::core::clone::Clone for IBitmapEncoderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapEncoderWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapEncoderWithSoftwareBitmap {}
impl ::core::clone::Clone for IBitmapEncoderWithSoftwareBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapFrame {}
impl ::core::clone::Clone for IBitmapFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapFrameWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapFrameWithSoftwareBitmap {}
impl ::core::clone::Clone for IBitmapFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapProperties {}
impl ::core::clone::Clone for IBitmapProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapPropertiesView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapPropertiesView {}
impl ::core::clone::Clone for IBitmapPropertiesView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapTransform {}
impl ::core::clone::Clone for IBitmapTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapTypedValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapTypedValue {}
impl ::core::clone::Clone for IBitmapTypedValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapTypedValueFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapTypedValueFactory {}
impl ::core::clone::Clone for IBitmapTypedValueFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPixelDataProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPixelDataProvider {}
impl ::core::clone::Clone for IPixelDataProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISoftwareBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISoftwareBitmap {}
impl ::core::clone::Clone for ISoftwareBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISoftwareBitmapFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISoftwareBitmapFactory {}
impl ::core::clone::Clone for ISoftwareBitmapFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISoftwareBitmapStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISoftwareBitmapStatics {}
impl ::core::clone::Clone for ISoftwareBitmapStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageStream {}
impl ::core::clone::Clone for ImageStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: Self = Self(0i32);
    pub const Y4Cb2Cr0: Self = Self(1i32);
    pub const Y4Cb2Cr2: Self = Self(2i32);
    pub const Y4Cb4Cr4: Self = Self(3i32);
}
impl ::core::marker::Copy for JpegSubsamplingMode {}
impl ::core::clone::Clone for JpegSubsamplingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PixelDataProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PixelDataProvider {}
impl ::core::clone::Clone for PixelDataProvider {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PngFilterMode {}
impl ::core::clone::Clone for PngFilterMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SoftwareBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SoftwareBitmap {}
impl ::core::clone::Clone for SoftwareBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TiffCompressionMode {}
impl ::core::clone::Clone for TiffCompressionMode {
    fn clone(&self) -> Self {
        *self
    }
}
