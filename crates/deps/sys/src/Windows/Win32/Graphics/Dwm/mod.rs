#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DwmAttachMilContent();
    fn DwmDefWindowProc();
    fn DwmDetachMilContent();
    fn DwmEnableBlurBehindWindow();
    fn DwmEnableComposition();
    fn DwmEnableMMCSS();
    fn DwmExtendFrameIntoClientArea();
    fn DwmFlush();
    fn DwmGetColorizationColor();
    fn DwmGetCompositionTimingInfo();
    fn DwmGetGraphicsStreamClient();
    fn DwmGetGraphicsStreamTransformHint();
    fn DwmGetTransportAttributes();
    fn DwmGetUnmetTabRequirements();
    fn DwmGetWindowAttribute();
    fn DwmInvalidateIconicBitmaps();
    fn DwmIsCompositionEnabled();
    fn DwmModifyPreviousDxFrameDuration();
    fn DwmQueryThumbnailSourceSize();
    fn DwmRegisterThumbnail();
    fn DwmRenderGesture();
    fn DwmSetDxFrameDuration();
    fn DwmSetIconicLivePreviewBitmap();
    fn DwmSetIconicThumbnail();
    fn DwmSetPresentParameters();
    fn DwmSetWindowAttribute();
    fn DwmShowContact();
    fn DwmTetherContact();
    fn DwmTransitionOwnedWindow();
    fn DwmUnregisterThumbnail();
    fn DwmUpdateThumbnailProperties();
}
