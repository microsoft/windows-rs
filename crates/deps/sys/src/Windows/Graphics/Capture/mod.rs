#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Direct3D11CaptureFrame();
    fn Direct3D11CaptureFramePool();
    fn GraphicsCaptureAccess();
    fn GraphicsCaptureAccessKind();
    fn GraphicsCaptureItem();
    fn GraphicsCapturePicker();
    fn GraphicsCaptureSession();
    fn IDirect3D11CaptureFrame();
    fn IDirect3D11CaptureFramePool();
    fn IDirect3D11CaptureFramePoolStatics();
    fn IDirect3D11CaptureFramePoolStatics2();
    fn IGraphicsCaptureAccessStatics();
    fn IGraphicsCaptureItem();
    fn IGraphicsCaptureItemStatics();
    fn IGraphicsCaptureItemStatics2();
    fn IGraphicsCapturePicker();
    fn IGraphicsCaptureSession();
    fn IGraphicsCaptureSession2();
    fn IGraphicsCaptureSession3();
    fn IGraphicsCaptureSessionStatics();
}
