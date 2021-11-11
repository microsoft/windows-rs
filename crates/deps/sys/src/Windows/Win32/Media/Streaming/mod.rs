#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CapturedMetadataExposureCompensation();
    fn CapturedMetadataISOGains();
    fn CapturedMetadataWhiteBalanceGains();
    fn FaceCharacterization();
    fn FaceCharacterizationBlobHeader();
    fn FaceRectInfo();
    fn FaceRectInfoBlobHeader();
    fn HistogramBlobHeader();
    fn HistogramDataHeader();
    fn HistogramGrid();
    fn HistogramHeader();
    fn IMFDeviceTransform();
    fn IMFDeviceTransformCallback();
    fn MF_MEDIASOURCE_STATUS_INFO();
    fn MF_TRANSFER_VIDEO_FRAME_FLAGS();
    fn MetadataTimeStamps();
}
