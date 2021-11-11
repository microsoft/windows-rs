#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CoreAppWindowPreview();
    fn ICoreAppWindowPreview();
    fn ICoreAppWindowPreviewStatics();
    fn ISystemNavigationCloseRequestedPreviewEventArgs();
    fn ISystemNavigationManagerPreview();
    fn ISystemNavigationManagerPreviewStatics();
    fn SystemNavigationCloseRequestedPreviewEventArgs();
    fn SystemNavigationManagerPreview();
}
