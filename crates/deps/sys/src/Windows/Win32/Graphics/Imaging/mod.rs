#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[link(name = "windows")]
extern "system" {
    fn WICConvertBitmapSource();
    fn WICCreateBitmapFromSection();
    fn WICCreateBitmapFromSectionEx();
    fn WICGetMetadataContentSize();
    fn WICMapGuidToShortName();
    fn WICMapSchemaToName();
    fn WICMapShortNameToGuid();
    fn WICMatchMetadataContent();
    fn WICSerializeMetadataContent();
}
