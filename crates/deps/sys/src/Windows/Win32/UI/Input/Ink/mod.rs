#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IInkCommitRequestHandler();
    fn IInkD2DRenderer();
    fn IInkD2DRenderer2();
    fn IInkDesktopHost();
    fn IInkHostWorkItem();
    fn IInkPresenterDesktop();
    fn INK_HIGH_CONTRAST_ADJUSTMENT();
    fn InkD2DRenderer();
    fn InkDesktopHost();
}
