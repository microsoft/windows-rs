#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: ClosedCaptionColor = ClosedCaptionColor(0i32);
    pub const White: ClosedCaptionColor = ClosedCaptionColor(1i32);
    pub const Black: ClosedCaptionColor = ClosedCaptionColor(2i32);
    pub const Red: ClosedCaptionColor = ClosedCaptionColor(3i32);
    pub const Green: ClosedCaptionColor = ClosedCaptionColor(4i32);
    pub const Blue: ClosedCaptionColor = ClosedCaptionColor(5i32);
    pub const Yellow: ClosedCaptionColor = ClosedCaptionColor(6i32);
    pub const Magenta: ClosedCaptionColor = ClosedCaptionColor(7i32);
    pub const Cyan: ClosedCaptionColor = ClosedCaptionColor(8i32);
}
#[repr(transparent)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(0i32);
    pub const None: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(1i32);
    pub const Raised: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(2i32);
    pub const Depressed: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(3i32);
    pub const Uniform: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(4i32);
    pub const DropShadow: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(5i32);
}
#[repr(transparent)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: ClosedCaptionOpacity = ClosedCaptionOpacity(0i32);
    pub const OneHundredPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(1i32);
    pub const SeventyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(2i32);
    pub const TwentyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(3i32);
    pub const ZeroPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(4i32);
}
#[repr(transparent)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: ClosedCaptionSize = ClosedCaptionSize(0i32);
    pub const FiftyPercent: ClosedCaptionSize = ClosedCaptionSize(1i32);
    pub const OneHundredPercent: ClosedCaptionSize = ClosedCaptionSize(2i32);
    pub const OneHundredFiftyPercent: ClosedCaptionSize = ClosedCaptionSize(3i32);
    pub const TwoHundredPercent: ClosedCaptionSize = ClosedCaptionSize(4i32);
}
#[repr(transparent)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: ClosedCaptionStyle = ClosedCaptionStyle(0i32);
    pub const MonospacedWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(1i32);
    pub const ProportionalWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(2i32);
    pub const MonospacedWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(3i32);
    pub const ProportionalWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(4i32);
    pub const Casual: ClosedCaptionStyle = ClosedCaptionStyle(5i32);
    pub const Cursive: ClosedCaptionStyle = ClosedCaptionStyle(6i32);
    pub const SmallCapitals: ClosedCaptionStyle = ClosedCaptionStyle(7i32);
}
#[repr(transparent)]
pub struct IClosedCaptionPropertiesStatics(pub *mut ::core::ffi::c_void);
