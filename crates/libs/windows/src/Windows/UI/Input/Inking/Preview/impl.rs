#[cfg(feature = "implement_exclusive")]
pub trait IPalmRejectionDelayZonePreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPalmRejectionDelayZonePreviewStaticsImpl: Sized {
    fn CreateForVisual(&self, inputpanelvisual: &::core::option::Option<super::super::super::Composition::Visual>, inputpanelrect: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>;
    fn CreateForVisualWithViewportClip(&self, inputpanelvisual: &::core::option::Option<super::super::super::Composition::Visual>, inputpanelrect: &super::super::super::super::Foundation::Rect, viewportvisual: &::core::option::Option<super::super::super::Composition::Visual>, viewportrect: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<PalmRejectionDelayZonePreview>;
}
