pub trait ICompositionCapabilitiesInteropFactoryImpl: Sized {
    fn GetForWindow();
}
pub trait ICompositionDrawingSurfaceInteropImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn Resize();
    fn Scroll();
    fn ResumeDraw();
    fn SuspendDraw();
}
pub trait ICompositionDrawingSurfaceInterop2Impl: Sized + ICompositionDrawingSurfaceInteropImpl {
    fn CopySurface();
}
pub trait ICompositionGraphicsDeviceInteropImpl: Sized {
    fn GetRenderingDevice();
    fn SetRenderingDevice();
}
pub trait ICompositorDesktopInteropImpl: Sized {
    fn CreateDesktopWindowTarget();
    fn EnsureOnThread();
}
pub trait ICompositorInteropImpl: Sized {
    fn CreateCompositionSurfaceForHandle();
    fn CreateCompositionSurfaceForSwapChain();
    fn CreateGraphicsDevice();
}
pub trait IDesktopWindowTargetInteropImpl: Sized {
    fn Hwnd();
}
pub trait ISwapChainInteropImpl: Sized {
    fn SetSwapChain();
}
pub trait IVisualInteractionSourceInteropImpl: Sized {
    fn TryRedirectForManipulation();
}
