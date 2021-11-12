#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BitmapAlphaMode(i32);
pub struct BitmapBounds(i32);
#[repr(transparent)]
pub struct BitmapBuffer(pub *mut ::core::ffi::c_void);
pub struct BitmapBufferAccessMode(i32);
#[repr(transparent)]
pub struct BitmapCodecInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapEncoder(pub *mut ::core::ffi::c_void);
pub struct BitmapFlip(i32);
#[repr(transparent)]
pub struct BitmapFrame(pub *mut ::core::ffi::c_void);
pub struct BitmapInterpolationMode(i32);
pub struct BitmapPixelFormat(i32);
pub struct BitmapPlaneDescription(i32);
#[repr(transparent)]
pub struct BitmapProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapPropertiesView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapPropertySet(pub *mut ::core::ffi::c_void);
pub struct BitmapRotation(i32);
pub struct BitmapSize(i32);
#[repr(transparent)]
pub struct BitmapTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapTypedValue(pub *mut ::core::ffi::c_void);
pub struct ColorManagementMode(i32);
pub struct ExifOrientationMode(i32);
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
pub struct JpegSubsamplingMode(i32);
#[repr(transparent)]
pub struct PixelDataProvider(pub *mut ::core::ffi::c_void);
pub struct PngFilterMode(i32);
#[repr(transparent)]
pub struct SoftwareBitmap(pub *mut ::core::ffi::c_void);
pub struct TiffCompressionMode(i32);
