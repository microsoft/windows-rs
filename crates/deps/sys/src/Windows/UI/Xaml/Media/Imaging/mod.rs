#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BitmapCreateOptions(i32);
#[repr(transparent)]
pub struct BitmapImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapSource(pub *mut ::core::ffi::c_void);
pub struct DecodePixelType(i32);
#[repr(transparent)]
pub struct DownloadProgressEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DownloadProgressEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImageStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapImageStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadProgressEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRenderTargetBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRenderTargetBitmapStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISoftwareBitmapSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfaceImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfaceImageSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISvgImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISvgImageSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISvgImageSourceFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISvgImageSourceOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISvgImageSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualSurfaceImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWriteableBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWriteableBitmapFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RenderTargetBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SoftwareBitmapSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SurfaceImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SvgImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SvgImageSourceFailedEventArgs(pub *mut ::core::ffi::c_void);
pub struct SvgImageSourceLoadStatus(i32);
#[repr(transparent)]
pub struct SvgImageSourceOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VirtualSurfaceImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WriteableBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlRenderingBackgroundTask(pub *mut ::core::ffi::c_void);
