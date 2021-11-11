#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPdfDocument();
    fn IPdfDocumentStatics();
    fn IPdfPage();
    fn IPdfPageDimensions();
    fn IPdfPageRenderOptions();
    fn PdfDocument();
    fn PdfPage();
    fn PdfPageDimensions();
    fn PdfPageRenderOptions();
    fn PdfPageRotation();
}
