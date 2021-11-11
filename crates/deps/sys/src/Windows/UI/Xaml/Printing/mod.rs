#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddPagesEventArgs();
    fn AddPagesEventHandler();
    fn GetPreviewPageEventArgs();
    fn GetPreviewPageEventHandler();
    fn IAddPagesEventArgs();
    fn IGetPreviewPageEventArgs();
    fn IPaginateEventArgs();
    fn IPrintDocument();
    fn IPrintDocumentFactory();
    fn IPrintDocumentStatics();
    fn PaginateEventArgs();
    fn PaginateEventHandler();
    fn PreviewPageCountType();
    fn PrintDocument();
}
