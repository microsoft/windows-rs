#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::TypeKind for PDF_RENDER_PARAMS {
    type TypeKind = windows_core::CloneType;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = Option<unsafe extern "system" fn(param0: Option<super::super::super::Graphics::Dxgi::IDXGIDevice>, param1: *mut Option<IPdfRendererNative>) -> windows_core::HRESULT>;
