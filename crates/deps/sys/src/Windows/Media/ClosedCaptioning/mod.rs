#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ClosedCaptionColor();
    fn ClosedCaptionEdgeEffect();
    fn ClosedCaptionOpacity();
    fn ClosedCaptionProperties();
    fn ClosedCaptionSize();
    fn ClosedCaptionStyle();
    fn IClosedCaptionPropertiesStatics();
}
