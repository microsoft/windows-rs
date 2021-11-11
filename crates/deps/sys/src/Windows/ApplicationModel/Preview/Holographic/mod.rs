#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HolographicApplicationPreview();
    fn HolographicKeyboardPlacementOverridePreview();
    fn IHolographicApplicationPreviewStatics();
    fn IHolographicKeyboardPlacementOverridePreview();
    fn IHolographicKeyboardPlacementOverridePreviewStatics();
}
