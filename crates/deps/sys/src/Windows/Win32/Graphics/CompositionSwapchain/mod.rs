#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CompositionFrameDisplayInstance();
    fn CompositionFrameInstanceKind();
    fn CreatePresentationFactory();
    fn ICompositionFramePresentStatistics();
    fn IIndependentFlipFramePresentStatistics();
    fn IPresentStatistics();
    fn IPresentStatusPresentStatistics();
    fn IPresentationBuffer();
    fn IPresentationContent();
    fn IPresentationFactory();
    fn IPresentationManager();
    fn IPresentationSurface();
    fn PresentStatisticsKind();
    fn PresentStatus();
    fn PresentationTransform();
    fn SystemInterruptTime();
}
