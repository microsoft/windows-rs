#[cfg(feature = "implement_exclusive")]
pub trait IClosedCaptionPropertiesStaticsImpl: Sized {
    fn FontColor(&self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedFontColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn FontOpacity(&self) -> ::windows::core::Result<ClosedCaptionOpacity>;
    fn FontSize(&self) -> ::windows::core::Result<ClosedCaptionSize>;
    fn FontStyle(&self) -> ::windows::core::Result<ClosedCaptionStyle>;
    fn FontEffect(&self) -> ::windows::core::Result<ClosedCaptionEdgeEffect>;
    fn BackgroundColor(&self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn BackgroundOpacity(&self) -> ::windows::core::Result<ClosedCaptionOpacity>;
    fn RegionColor(&self) -> ::windows::core::Result<ClosedCaptionColor>;
    fn ComputedRegionColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn RegionOpacity(&self) -> ::windows::core::Result<ClosedCaptionOpacity>;
}
