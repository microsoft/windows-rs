#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IMediaTranscoder();
    fn IMediaTranscoder2();
    fn IPrepareTranscodeResult();
    fn MediaTranscoder();
    fn MediaVideoProcessingAlgorithm();
    fn PrepareTranscodeResult();
    fn TranscodeFailureReason();
}
