#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn D2D1ComputeMaximumScaleFactor(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> f32;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const Common::D2D1_COLOR_F) -> Common::D2D1_COLOR_F;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDevice(dxgidevice: ::windows::runtime::RawPtr, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDeviceContext(dxgisurface: ::windows::runtime::RawPtr, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevicecontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1CreateFactory(factorytype: D2D1_FACTORY_TYPE, riid: *const ::windows::runtime::GUID, pfactoryoptions: *const D2D1_FACTORY_OPTIONS, ppifactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(
        ppoint0: *const Common::D2D_POINT_2F,
        ppoint1: *const Common::D2D_POINT_2F,
        ppoint2: *const Common::D2D_POINT_2F,
        ppoint3: *const Common::D2D_POINT_2F,
        ppoint4: *const Common::D2D_POINT_2F,
        ppoint5: *const Common::D2D_POINT_2F,
        ppoint6: *const Common::D2D_POINT_2F,
        ppoint7: *const Common::D2D_POINT_2F,
        ppoint8: *const Common::D2D_POINT_2F,
        ppoint9: *const Common::D2D_POINT_2F,
        ppoint10: *const Common::D2D_POINT_2F,
        ppoint11: *const Common::D2D_POINT_2F,
        ptensorpoint11: *mut Common::D2D_POINT_2F,
        ptensorpoint12: *mut Common::D2D_POINT_2F,
        ptensorpoint21: *mut Common::D2D_POINT_2F,
        ptensorpoint22: *mut Common::D2D_POINT_2F,
    );
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1InvertMatrix(matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1IsMatrixInvertible(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeRotateMatrix(angle: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    #[doc = "*Required features: `Win32_Graphics_Direct2D`, `Foundation_Numerics`, `Win32_Graphics_Direct2D_Common`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1Tan(angle: f32) -> f32;
    #[doc = "*Required features: `Win32_Graphics_Direct2D`*"]
    pub fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32;
}
