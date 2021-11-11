#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn D2D1ComputeMaximumScaleFactor();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1ConvertColorSpace();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDevice();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDeviceContext();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1CreateFactory();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1InvertMatrix();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1IsMatrixInvertible();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeRotateMatrix();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeSkewMatrix();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1SinCos();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1Tan();
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1Vec3Length();
}
