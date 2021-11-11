#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CoreIncrementalInkStroke();
    fn CoreInkIndependentInputSource();
    fn CoreInkPresenterHost();
    fn CoreWetStrokeDisposition();
    fn CoreWetStrokeUpdateEventArgs();
    fn CoreWetStrokeUpdateSource();
    fn ICoreIncrementalInkStroke();
    fn ICoreIncrementalInkStrokeFactory();
    fn ICoreInkIndependentInputSource();
    fn ICoreInkIndependentInputSource2();
    fn ICoreInkIndependentInputSourceStatics();
    fn ICoreInkPresenterHost();
    fn ICoreWetStrokeUpdateEventArgs();
    fn ICoreWetStrokeUpdateSource();
    fn ICoreWetStrokeUpdateSourceStatics();
}
