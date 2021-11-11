#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IImageScanner();
    fn IImageScannerFeederConfiguration();
    fn IImageScannerFormatConfiguration();
    fn IImageScannerPreviewResult();
    fn IImageScannerScanResult();
    fn IImageScannerSourceConfiguration();
    fn IImageScannerStatics();
    fn ImageScanner();
    fn ImageScannerAutoConfiguration();
    fn ImageScannerAutoCroppingMode();
    fn ImageScannerColorMode();
    fn ImageScannerFeederConfiguration();
    fn ImageScannerFlatbedConfiguration();
    fn ImageScannerFormat();
    fn ImageScannerPreviewResult();
    fn ImageScannerResolution();
    fn ImageScannerScanResult();
    fn ImageScannerScanSource();
    fn ScannerDeviceContract();
}
