#[cfg(feature = "dxgi")]
windows_link::link!("windows.data.pdf.dll" "system" fn PdfCreateRenderer(pdevice : *mut core::ffi::c_void, pprenderer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(all(feature = "d2d", feature = "dcommon", feature = "dxgi"))]
#[derive(Clone, Copy, Default)]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::D2D_COLOR_F,
    pub IgnoreHighContrast: bool,
}
#[cfg(feature = "dxgi")]
pub type PFN_PDF_CREATE_RENDERER = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
