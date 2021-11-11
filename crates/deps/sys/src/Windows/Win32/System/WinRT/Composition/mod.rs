#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ICompositionCapabilitiesInteropFactory();
    fn ICompositionDrawingSurfaceInterop();
    fn ICompositionDrawingSurfaceInterop2();
    fn ICompositionGraphicsDeviceInterop();
    fn ICompositorDesktopInterop();
    fn ICompositorInterop();
    fn IDesktopWindowTargetInterop();
    fn ISwapChainInterop();
    fn IVisualInteractionSourceInterop();
}
