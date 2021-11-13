#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: Self = Self(0u32);
    pub const IgnoreImageCache: Self = Self(8u32);
}
impl ::core::marker::Copy for BitmapCreateOptions {}
impl ::core::clone::Clone for BitmapCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapImage {}
impl ::core::clone::Clone for BitmapImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapSource {}
impl ::core::clone::Clone for BitmapSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: Self = Self(0i32);
    pub const Logical: Self = Self(1i32);
}
impl ::core::marker::Copy for DecodePixelType {}
impl ::core::clone::Clone for DecodePixelType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DownloadProgressEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DownloadProgressEventArgs {}
impl ::core::clone::Clone for DownloadProgressEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DownloadProgressEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DownloadProgressEventHandler {}
impl ::core::clone::Clone for DownloadProgressEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImage {}
impl ::core::clone::Clone for IBitmapImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImage2 {}
impl ::core::clone::Clone for IBitmapImage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImage3 {}
impl ::core::clone::Clone for IBitmapImage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImageFactory {}
impl ::core::clone::Clone for IBitmapImageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImageStatics {}
impl ::core::clone::Clone for IBitmapImageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImageStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImageStatics2 {}
impl ::core::clone::Clone for IBitmapImageStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapImageStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapImageStatics3 {}
impl ::core::clone::Clone for IBitmapImageStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapSource {}
impl ::core::clone::Clone for IBitmapSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapSourceFactory {}
impl ::core::clone::Clone for IBitmapSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapSourceStatics {}
impl ::core::clone::Clone for IBitmapSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDownloadProgressEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDownloadProgressEventArgs {}
impl ::core::clone::Clone for IDownloadProgressEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRenderTargetBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRenderTargetBitmap {}
impl ::core::clone::Clone for IRenderTargetBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRenderTargetBitmapStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRenderTargetBitmapStatics {}
impl ::core::clone::Clone for IRenderTargetBitmapStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISoftwareBitmapSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISoftwareBitmapSource {}
impl ::core::clone::Clone for ISoftwareBitmapSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurfaceImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurfaceImageSource {}
impl ::core::clone::Clone for ISurfaceImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurfaceImageSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurfaceImageSourceFactory {}
impl ::core::clone::Clone for ISurfaceImageSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISvgImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISvgImageSource {}
impl ::core::clone::Clone for ISvgImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISvgImageSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISvgImageSourceFactory {}
impl ::core::clone::Clone for ISvgImageSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISvgImageSourceFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISvgImageSourceFailedEventArgs {}
impl ::core::clone::Clone for ISvgImageSourceFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISvgImageSourceOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISvgImageSourceOpenedEventArgs {}
impl ::core::clone::Clone for ISvgImageSourceOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISvgImageSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISvgImageSourceStatics {}
impl ::core::clone::Clone for ISvgImageSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualSurfaceImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualSurfaceImageSource {}
impl ::core::clone::Clone for IVirtualSurfaceImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualSurfaceImageSourceFactory {}
impl ::core::clone::Clone for IVirtualSurfaceImageSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWriteableBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWriteableBitmap {}
impl ::core::clone::Clone for IWriteableBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWriteableBitmapFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWriteableBitmapFactory {}
impl ::core::clone::Clone for IWriteableBitmapFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlRenderingBackgroundTask {}
impl ::core::clone::Clone for IXamlRenderingBackgroundTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlRenderingBackgroundTaskFactory {}
impl ::core::clone::Clone for IXamlRenderingBackgroundTaskFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlRenderingBackgroundTaskOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlRenderingBackgroundTaskOverrides {}
impl ::core::clone::Clone for IXamlRenderingBackgroundTaskOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RenderTargetBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RenderTargetBitmap {}
impl ::core::clone::Clone for RenderTargetBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SoftwareBitmapSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SoftwareBitmapSource {}
impl ::core::clone::Clone for SoftwareBitmapSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SurfaceImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SurfaceImageSource {}
impl ::core::clone::Clone for SurfaceImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SvgImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SvgImageSource {}
impl ::core::clone::Clone for SvgImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SvgImageSourceFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SvgImageSourceFailedEventArgs {}
impl ::core::clone::Clone for SvgImageSourceFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for SvgImageSourceLoadStatus {}
impl ::core::clone::Clone for SvgImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SvgImageSourceOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SvgImageSourceOpenedEventArgs {}
impl ::core::clone::Clone for SvgImageSourceOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VirtualSurfaceImageSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VirtualSurfaceImageSource {}
impl ::core::clone::Clone for VirtualSurfaceImageSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WriteableBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WriteableBitmap {}
impl ::core::clone::Clone for WriteableBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlRenderingBackgroundTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlRenderingBackgroundTask {}
impl ::core::clone::Clone for XamlRenderingBackgroundTask {
    fn clone(&self) -> Self {
        *self
    }
}
