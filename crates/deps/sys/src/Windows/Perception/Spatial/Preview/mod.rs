#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ISpatialGraphInteropFrameOfReferencePreview();
    fn ISpatialGraphInteropPreviewStatics();
    fn ISpatialGraphInteropPreviewStatics2();
    fn SpatialGraphInteropFrameOfReferencePreview();
    fn SpatialGraphInteropPreview();
}
