#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppRecordingContract();
    fn AppRecordingManager();
    fn AppRecordingResult();
    fn AppRecordingSaveScreenshotOption();
    fn AppRecordingSaveScreenshotResult();
    fn AppRecordingSavedScreenshotInfo();
    fn AppRecordingStatus();
    fn AppRecordingStatusDetails();
    fn IAppRecordingManager();
    fn IAppRecordingManagerStatics();
    fn IAppRecordingResult();
    fn IAppRecordingSaveScreenshotResult();
    fn IAppRecordingSavedScreenshotInfo();
    fn IAppRecordingStatus();
    fn IAppRecordingStatusDetails();
}
