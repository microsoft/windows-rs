#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IKnownSimpleHapticsControllerWaveformsStatics();
    fn IKnownSimpleHapticsControllerWaveformsStatics2();
    fn ISimpleHapticsController();
    fn ISimpleHapticsControllerFeedback();
    fn IVibrationDevice();
    fn IVibrationDeviceStatics();
    fn KnownSimpleHapticsControllerWaveforms();
    fn SimpleHapticsController();
    fn SimpleHapticsControllerFeedback();
    fn VibrationAccessStatus();
    fn VibrationDevice();
}
