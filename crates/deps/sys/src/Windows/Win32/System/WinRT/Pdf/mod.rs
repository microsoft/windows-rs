#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPdfRendererNative();
    fn PDF_RENDER_PARAMS();
    fn PFN_PDF_CREATE_RENDERER();
    fn PdfCreateRenderer();
}
