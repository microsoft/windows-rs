#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DetectedFace();
    fn FaceDetector();
    fn FaceTracker();
    fn IDetectedFace();
    fn IFaceDetector();
    fn IFaceDetectorStatics();
    fn IFaceTracker();
    fn IFaceTrackerStatics();
}
