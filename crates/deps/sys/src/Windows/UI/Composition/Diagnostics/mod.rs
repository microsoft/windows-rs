#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CompositionDebugHeatMaps();
    fn CompositionDebugOverdrawContentKinds();
    fn CompositionDebugSettings();
    fn ICompositionDebugHeatMaps();
    fn ICompositionDebugSettings();
    fn ICompositionDebugSettingsStatics();
}
