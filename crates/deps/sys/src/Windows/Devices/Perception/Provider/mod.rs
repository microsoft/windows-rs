#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IKnownPerceptionFrameKindStatics();
    fn IPerceptionControlGroup();
    fn IPerceptionControlGroupFactory();
    fn IPerceptionCorrelation();
    fn IPerceptionCorrelationFactory();
    fn IPerceptionCorrelationGroup();
    fn IPerceptionCorrelationGroupFactory();
    fn IPerceptionFaceAuthenticationGroup();
    fn IPerceptionFaceAuthenticationGroupFactory();
    fn IPerceptionFrame();
    fn IPerceptionFrameProvider();
    fn IPerceptionFrameProviderInfo();
    fn IPerceptionFrameProviderManager();
    fn IPerceptionFrameProviderManagerServiceStatics();
    fn IPerceptionPropertyChangeRequest();
    fn IPerceptionVideoFrameAllocator();
    fn IPerceptionVideoFrameAllocatorFactory();
    fn KnownPerceptionFrameKind();
    fn PerceptionControlGroup();
    fn PerceptionCorrelation();
    fn PerceptionCorrelationGroup();
    fn PerceptionFaceAuthenticationGroup();
    fn PerceptionFrame();
    fn PerceptionFrameProviderInfo();
    fn PerceptionFrameProviderManagerService();
    fn PerceptionPropertyChangeRequest();
    fn PerceptionStartFaceAuthenticationHandler();
    fn PerceptionStopFaceAuthenticationHandler();
    fn PerceptionVideoFrameAllocator();
}
