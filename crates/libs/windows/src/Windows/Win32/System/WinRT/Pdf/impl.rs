pub trait IPdfRendererNativeImpl: Sized {
    fn RenderPageToSurface();
    fn RenderPageToDeviceContext();
}
