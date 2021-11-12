#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn D2D1ComputeMaximumScaleFactor(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> f32;
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub fn D2D1ConvertColorSpace(sourcecolorspace: D2D1_COLOR_SPACE, destinationcolorspace: D2D1_COLOR_SPACE, color: *const Common::D2D1_COLOR_F) -> Common::D2D1_COLOR_F;
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDevice(dxgidevice: super::Dxgi::IDXGIDevice, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevice: *mut ID2D1Device) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn D2D1CreateDeviceContext(dxgisurface: super::Dxgi::IDXGISurface, creationproperties: *const D2D1_CREATION_PROPERTIES, d2ddevicecontext: *mut ID2D1DeviceContext) -> ::windows_sys::core::HRESULT;
    pub fn D2D1CreateFactory(factorytype: D2D1_FACTORY_TYPE, riid: *const ::windows_sys::core::GUID, pfactoryoptions: *const D2D1_FACTORY_OPTIONS, ppifactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
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
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1InvertMatrix(matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
    pub fn D2D1IsMatrixInvertible(matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeRotateMatrix(angle: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub fn D2D1MakeSkewMatrix(anglex: f32, angley: f32, center: Common::D2D_POINT_2F, matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    pub fn D2D1SinCos(angle: f32, s: *mut f32, c: *mut f32);
    pub fn D2D1Tan(angle: f32) -> f32;
    pub fn D2D1Vec3Length(x: f32, y: f32, z: f32) -> f32;
}
pub const CLSID_D2D12DAffineTransform: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1789490309,
    data2: 25428,
    data3: 19708,
    data4: [144, 140, 228, 167, 79, 98, 201, 108],
};
pub const CLSID_D2D13DPerspectiveTransform: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3263450379,
    data2: 15750,
    data3: 18151,
    data4: [133, 186, 82, 108, 146, 64, 243, 251],
};
pub const CLSID_D2D13DTransform: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3896933124,
    data2: 60513,
    data3: 19338,
    data4: [181, 222, 212, 215, 61, 235, 234, 90],
};
pub const CLSID_D2D1AlphaMask: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3356413936, data2: 16341, data3: 20229, data4: [131, 40, 197, 209, 114, 75, 79, 10] };
pub const CLSID_D2D1ArithmeticComposite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4229239863, data2: 1178, data3: 18308, data4: [162, 74, 241, 196, 218, 242, 9, 135] };
pub const CLSID_D2D1Atlas: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2436770788, data2: 64975, data3: 20450, data4: [165, 240, 36, 84, 241, 79, 244, 8] };
pub const CLSID_D2D1BitmapSource: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1605812813, data2: 50909, data3: 16945, data4: [148, 4, 80, 244, 213, 195, 37, 45] };
pub const CLSID_D2D1Blend: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2177218427, data2: 5112, data3: 19677, data4: [173, 32, 200, 144, 84, 122, 198, 93] };
pub const CLSID_D2D1Border: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 707611072, data2: 19151, data3: 17351, data4: [140, 106, 124, 74, 39, 135, 77, 39] };
pub const CLSID_D2D1Brightness: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2364181790,
    data2: 30640,
    data3: 18822,
    data4: [179, 185, 47, 12, 14, 174, 120, 135],
};
pub const CLSID_D2D1ChromaKey: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1958747995,
    data2: 10765,
    data3: 16524,
    data4: [136, 226, 199, 163, 199, 25, 119, 66],
};
pub const CLSID_D2D1ColorManagement: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 438850124,
    data2: 64982,
    data3: 19108,
    data4: [174, 143, 131, 126, 184, 38, 123, 55],
};
pub const CLSID_D2D1ColorMatrix: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2451506134, data2: 25628, data3: 18399, data4: [133, 45, 180, 187, 97, 83, 174, 17] };
pub const CLSID_D2D1Composite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1224515409, data2: 63148, data3: 18673, data4: [139, 88, 59, 40, 172, 70, 247, 109] };
pub const CLSID_D2D1Contrast: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3058214794,
    data2: 3797,
    data3: 20352,
    data4: [169, 74, 142, 130, 90, 202, 107, 119],
};
pub const CLSID_D2D1ConvolveMatrix: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1082100744, data2: 21811, data3: 17201, data4: [163, 65, 35, 204, 56, 119, 132, 62] };
pub const CLSID_D2D1Crop: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3795808528, data2: 3738, data3: 17188, data4: [175, 71, 106, 44, 12, 70, 243, 91] };
pub const CLSID_D2D1CrossFade: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 318076392,
    data2: 19889,
    data3: 18527,
    data4: [154, 132, 3, 160, 125, 211, 130, 159],
};
pub const CLSID_D2D1DirectionalBlur: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 390273446,
    data2: 22761,
    data3: 18866,
    data4: [187, 99, 202, 242, 200, 17, 163, 219],
};
pub const CLSID_D2D1DiscreteTransfer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2424729549, data2: 18574, data3: 17739, data4: [175, 6, 229, 4, 27, 102, 195, 108] };
pub const CLSID_D2D1DisplacementMap: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3989078884, data2: 1047, data3: 16657, data4: [148, 80, 67, 132, 95, 169, 248, 144] };
pub const CLSID_D2D1DistantDiffuse: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1048509794,
    data2: 41773,
    data3: 18132,
    data4: [168, 60, 82, 120, 136, 154, 201, 84],
};
pub const CLSID_D2D1DistantSpecular: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1116479205,
    data2: 30648,
    data3: 17488,
    data4: [138, 181, 114, 33, 156, 33, 171, 218],
};
pub const CLSID_D2D1DpiCompensation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1814480327, data2: 13536, data3: 18172, data4: [156, 253, 229, 130, 55, 6, 226, 40] };
pub const CLSID_D2D1EdgeDetection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4025844682, data2: 51975, data3: 19113, data4: [172, 93, 44, 196, 76, 118, 70, 15] };
pub const CLSID_D2D1Emboss: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2982538027, data2: 840, data3: 17392, data4: [129, 7, 73, 87, 202, 203, 162, 174] };
pub const CLSID_D2D1Exposure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3043790074, data2: 63028, data3: 16878, data4: [190, 224, 255, 166, 23, 16, 96, 4] };
pub const CLSID_D2D1Flood: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1640119328, data2: 44649, data3: 19854, data4: [148, 207, 80, 7, 141, 246, 56, 242] };
pub const CLSID_D2D1GammaTransfer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1083458756,
    data2: 50201,
    data3: 16800,
    data4: [176, 193, 140, 208, 192, 161, 142, 66],
};
pub const CLSID_D2D1GaussianBlur: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 535522665,
    data2: 12262,
    data3: 19145,
    data4: [140, 88, 29, 127, 147, 231, 166, 165],
};
pub const CLSID_D2D1Grayscale: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 920510699, data2: 14117, data3: 17120, data4: [131, 109, 82, 251, 32, 174, 230, 68] };
pub const CLSID_D2D1HdrToneMap: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2064348301, data2: 17936, data3: 17542, data4: [169, 12, 153, 157, 154, 46, 43, 17] };
pub const CLSID_D2D1HighlightsShadows: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3403449220, data2: 12863, data3: 19582, data4: [163, 97, 46, 43, 36, 223, 110, 228] };
pub const CLSID_D2D1Histogram: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2283648976,
    data2: 63470,
    data3: 19789,
    data4: [166, 210, 70, 151, 172, 198, 110, 232],
};
pub const CLSID_D2D1HueRotation: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 256137452,
    data2: 19250,
    data3: 18715,
    data4: [158, 133, 189, 115, 244, 77, 62, 182],
};
pub const CLSID_D2D1HueToRgb: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2071504573, data2: 321, data3: 19951, data4: [138, 82, 99, 86, 238, 12, 189, 213] };
pub const CLSID_D2D1Invert: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3770906701,
    data2: 52025,
    data3: 20100,
    data4: [182, 253, 107, 114, 240, 129, 2, 99],
};
pub const CLSID_D2D1LinearTransfer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2907162877, data2: 25583, data3: 19148, data4: [155, 81, 103, 151, 156, 3, 108, 6] };
pub const CLSID_D2D1LookupTable3D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 882773722, data2: 136, data3: 19065, data4: [156, 163, 199, 227, 0, 32, 32, 32] };
pub const CLSID_D2D1LuminanceToAlpha: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1092950711,
    data2: 3051,
    data3: 18168,
    data4: [157, 167, 89, 233, 63, 204, 229, 222],
};
pub const CLSID_D2D1Morphology: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3940992013, data2: 25194, data3: 19501, data4: [191, 203, 57, 16, 1, 171, 226, 2] };
pub const CLSID_D2D1Opacity: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2166192548,
    data2: 56872,
    data3: 17492,
    data4: [128, 148, 198, 70, 133, 248, 189, 76],
};
pub const CLSID_D2D1OpacityMetadata: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1817378922, data2: 17488, data3: 16793, data4: [170, 91, 173, 22, 86, 254, 206, 94] };
pub const CLSID_D2D1PointDiffuse: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3118662595,
    data2: 49292,
    data3: 20369,
    data4: [139, 123, 56, 101, 107, 196, 140, 32],
};
pub const CLSID_D2D1PointSpecular: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 163826214, data2: 15074, data3: 20233, data4: [158, 188, 237, 56, 101, 213, 63, 34] };
pub const CLSID_D2D1Posterize: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 562599006, data2: 13219, data3: 17254, data4: [183, 188, 8, 107, 208, 45, 8, 132] };
pub const CLSID_D2D1Premultiply: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 116044825, data2: 57069, data3: 16408, data4: [128, 210, 62, 29, 71, 26, 222, 178] };
pub const CLSID_D2D1RgbToHue: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 603186668, data2: 37352, data3: 19773, data4: [173, 10, 175, 173, 193, 0, 74, 161] };
pub const CLSID_D2D1Saturation: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1555225039,
    data2: 12925,
    data3: 17823,
    data4: [160, 206, 64, 192, 178, 8, 107, 247],
};
pub const CLSID_D2D1Scale: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2645529449, data2: 14406, data3: 19726, data4: [164, 78, 12, 96, 121, 52, 165, 215] };
pub const CLSID_D2D1Sepia: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 974844944,
    data2: 24349,
    data3: 19902,
    data4: [132, 223, 145, 93, 167, 155, 113, 83],
};
pub const CLSID_D2D1Shadow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3330188129, data2: 6243, data3: 20073, data4: [137, 219, 105, 93, 62, 154, 91, 107] };
pub const CLSID_D2D1Sharpen: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3384313803,
    data2: 50687,
    data3: 19909,
    data4: [151, 121, 39, 61, 207, 65, 124, 125],
};
pub const CLSID_D2D1SpotDiffuse: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2173309189, data2: 31026, data3: 17652, data4: [170, 134, 8, 174, 123, 47, 44, 147] };
pub const CLSID_D2D1SpotSpecular: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3987620382,
    data2: 30292,
    data3: 18999,
    data4: [157, 184, 113, 172, 193, 190, 179, 193],
};
pub const CLSID_D2D1Straighten: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1302625042,
    data2: 31139,
    data3: 20400,
    data4: [130, 55, 187, 195, 178, 164, 222, 8],
};
pub const CLSID_D2D1TableTransfer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1542985923,
    data2: 24131,
    data3: 18635,
    data4: [182, 49, 134, 131, 150, 214, 161, 212],
};
pub const CLSID_D2D1TemperatureTint: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2300010631,
    data2: 35577,
    data3: 18952,
    data4: [174, 177, 137, 95, 56, 219, 23, 102],
};
pub const CLSID_D2D1Tile: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2960671032, data2: 15222, data3: 19397, data4: [177, 59, 15, 162, 173, 2, 101, 159] };
pub const CLSID_D2D1Tint: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 909191959,
    data2: 63453,
    data3: 16404,
    data4: [145, 93, 255, 202, 118, 140, 242, 17],
};
pub const CLSID_D2D1Turbulence: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3475748526,
    data2: 34970,
    data3: 19159,
    data4: [186, 41, 162, 253, 115, 44, 159, 201],
};
pub const CLSID_D2D1UnPremultiply: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4221224073,
    data2: 44429,
    data3: 16877,
    data4: [153, 153, 187, 99, 71, 209, 16, 247],
};
pub const CLSID_D2D1Vignette: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3222028478, data2: 24167, data3: 19619, data4: [149, 180, 244, 176, 44, 17, 81, 53] };
pub const CLSID_D2D1WhiteLevelAdjustment: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1151453915,
    data2: 27869,
    data3: 18456,
    data4: [143, 244, 38, 193, 207, 233, 91, 219],
};
pub const CLSID_D2D1YCbCr: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2572172481,
    data2: 26311,
    data3: 17865,
    data4: [168, 117, 138, 216, 167, 145, 68, 1],
};
pub struct D2D1_2DAFFINETRANSFORM_PROP(i32);
pub struct D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE(i32);
pub struct D2D1_3DPERSPECTIVETRANSFORM_PROP(i32);
pub struct D2D1_3DTRANSFORM_INTERPOLATION_MODE(i32);
pub struct D2D1_3DTRANSFORM_PROP(i32);
pub struct D2D1_ANTIALIAS_MODE(i32);
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ARC_SEGMENT(i32);
pub struct D2D1_ARC_SIZE(i32);
pub struct D2D1_ARITHMETICCOMPOSITE_PROP(i32);
pub struct D2D1_ATLAS_PROP(i32);
pub struct D2D1_BITMAPSOURCE_ALPHA_MODE(i32);
pub struct D2D1_BITMAPSOURCE_INTERPOLATION_MODE(i32);
pub struct D2D1_BITMAPSOURCE_ORIENTATION(i32);
pub struct D2D1_BITMAPSOURCE_PROP(i32);
pub struct D2D1_BITMAP_BRUSH_PROPERTIES(i32);
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1(i32);
pub struct D2D1_BITMAP_INTERPOLATION_MODE(i32);
pub struct D2D1_BITMAP_OPTIONS(i32);
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES(i32);
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES1(i32);
pub struct D2D1_BLEND(i32);
pub struct D2D1_BLEND_DESCRIPTION(i32);
pub struct D2D1_BLEND_OPERATION(i32);
pub struct D2D1_BLEND_PROP(i32);
pub struct D2D1_BORDER_EDGE_MODE(i32);
pub struct D2D1_BORDER_PROP(i32);
pub struct D2D1_BRIGHTNESS_PROP(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_BRUSH_PROPERTIES(i32);
pub struct D2D1_BUFFER_PRECISION(i32);
pub struct D2D1_CAP_STYLE(i32);
pub struct D2D1_CHANGE_TYPE(i32);
pub struct D2D1_CHANNEL_DEPTH(i32);
pub struct D2D1_CHANNEL_SELECTOR(i32);
pub struct D2D1_CHROMAKEY_PROP(i32);
pub struct D2D1_COLORMANAGEMENT_ALPHA_MODE(i32);
pub struct D2D1_COLORMANAGEMENT_PROP(i32);
pub struct D2D1_COLORMANAGEMENT_QUALITY(i32);
pub struct D2D1_COLORMANAGEMENT_RENDERING_INTENT(i32);
pub struct D2D1_COLORMATRIX_PROP(i32);
pub struct D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION(i32);
pub struct D2D1_COLOR_CONTEXT_TYPE(i32);
pub struct D2D1_COLOR_INTERPOLATION_MODE(i32);
pub struct D2D1_COLOR_SPACE(i32);
pub struct D2D1_COMBINE_MODE(i32);
pub struct D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(i32);
pub struct D2D1_COMPOSITE_PROP(i32);
pub struct D2D1_CONTRAST_PROP(i32);
pub struct D2D1_CONVOLVEMATRIX_PROP(i32);
pub struct D2D1_CONVOLVEMATRIX_SCALE_MODE(i32);
pub struct D2D1_CREATION_PROPERTIES(i32);
pub struct D2D1_CROP_PROP(i32);
pub struct D2D1_CROSSFADE_PROP(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES(i32);
pub struct D2D1_DASH_STYLE(i32);
pub struct D2D1_DC_INITIALIZE_MODE(i32);
pub struct D2D1_DEBUG_LEVEL(i32);
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
pub struct D2D1_DEVICE_CONTEXT_OPTIONS(i32);
pub struct D2D1_DIRECTIONALBLUR_OPTIMIZATION(i32);
pub struct D2D1_DIRECTIONALBLUR_PROP(i32);
pub struct D2D1_DISCRETETRANSFER_PROP(i32);
pub struct D2D1_DISPLACEMENTMAP_PROP(i32);
pub struct D2D1_DISTANTDIFFUSE_PROP(i32);
pub struct D2D1_DISTANTDIFFUSE_SCALE_MODE(i32);
pub struct D2D1_DISTANTSPECULAR_PROP(i32);
pub struct D2D1_DISTANTSPECULAR_SCALE_MODE(i32);
pub struct D2D1_DPICOMPENSATION_INTERPOLATION_MODE(i32);
pub struct D2D1_DPICOMPENSATION_PROP(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1(i32);
pub struct D2D1_DRAW_TEXT_OPTIONS(i32);
pub struct D2D1_EDGEDETECTION_MODE(i32);
pub struct D2D1_EDGEDETECTION_PROP(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ELLIPSE(i32);
pub struct D2D1_EMBOSS_PROP(i32);
pub struct D2D1_EXPOSURE_PROP(i32);
pub struct D2D1_EXTEND_MODE(i32);
pub struct D2D1_FACTORY_OPTIONS(i32);
pub struct D2D1_FACTORY_TYPE(i32);
pub struct D2D1_FEATURE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_DOUBLES(i32);
pub struct D2D1_FEATURE_LEVEL(i32);
pub struct D2D1_FILTER(i32);
pub struct D2D1_FLOOD_PROP(i32);
pub struct D2D1_GAMMA(i32);
pub struct D2D1_GAMMA1(i32);
pub struct D2D1_GAMMATRANSFER_PROP(i32);
pub struct D2D1_GAUSSIANBLUR_OPTIMIZATION(i32);
pub struct D2D1_GAUSSIANBLUR_PROP(i32);
pub struct D2D1_GEOMETRY_RELATION(i32);
pub struct D2D1_GEOMETRY_SIMPLIFICATION_OPTION(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_MESH_PATCH(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_STOP(i32);
pub struct D2D1_HDRTONEMAP_DISPLAY_MODE(i32);
pub struct D2D1_HDRTONEMAP_PROP(i32);
pub struct D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA(i32);
pub struct D2D1_HIGHLIGHTSANDSHADOWS_PROP(i32);
pub struct D2D1_HISTOGRAM_PROP(i32);
pub struct D2D1_HUEROTATION_PROP(i32);
pub struct D2D1_HUETORGB_INPUT_COLOR_SPACE(i32);
pub struct D2D1_HUETORGB_PROP(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES(i32);
pub struct D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS(i32);
pub struct D2D1_IMAGE_SOURCE_LOADING_OPTIONS(i32);
pub struct D2D1_INK_BEZIER_SEGMENT(i32);
pub struct D2D1_INK_NIB_SHAPE(i32);
pub struct D2D1_INK_POINT(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_INK_STYLE_PROPERTIES(i32);
pub struct D2D1_INPUT_DESCRIPTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_INPUT_ELEMENT_DESC(i32);
pub struct D2D1_INTERPOLATION_MODE(i32);
pub struct D2D1_INTERPOLATION_MODE_DEFINITION(i32);
pub struct D2D1_LAYER_OPTIONS(i32);
pub struct D2D1_LAYER_OPTIONS1(i32);
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS(i32);
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS1(i32);
pub struct D2D1_LINEARTRANSFER_PROP(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES(i32);
pub struct D2D1_LINE_JOIN(i32);
pub struct D2D1_LOOKUPTABLE3D_PROP(i32);
pub struct D2D1_MAPPED_RECT(i32);
pub struct D2D1_MAP_OPTIONS(i32);
pub struct D2D1_MORPHOLOGY_MODE(i32);
pub struct D2D1_MORPHOLOGY_PROP(i32);
pub struct D2D1_OPACITYMETADATA_PROP(i32);
pub struct D2D1_OPACITY_MASK_CONTENT(i32);
pub struct D2D1_OPACITY_PROP(i32);
pub struct D2D1_ORIENTATION(i32);
pub struct D2D1_PATCH_EDGE_MODE(i32);
pub struct D2D1_PIXEL_OPTIONS(i32);
pub struct D2D1_POINTDIFFUSE_PROP(i32);
pub struct D2D1_POINTDIFFUSE_SCALE_MODE(i32);
pub struct D2D1_POINTSPECULAR_PROP(i32);
pub struct D2D1_POINTSPECULAR_SCALE_MODE(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_POINT_DESCRIPTION(i32);
pub struct D2D1_POSTERIZE_PROP(i32);
pub struct D2D1_PRESENT_OPTIONS(i32);
pub struct D2D1_PRIMITIVE_BLEND(i32);
pub struct D2D1_PRINT_CONTROL_PROPERTIES(i32);
pub struct D2D1_PRINT_FONT_SUBSET_MODE(i32);
pub struct D2D1_PROPERTY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_PROPERTY_BINDING(i32);
pub struct D2D1_PROPERTY_TYPE(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RENDERING_CONTROLS(i32);
pub struct D2D1_RENDERING_PRIORITY(i32);
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_RENDER_TARGET_PROPERTIES(i32);
pub struct D2D1_RENDER_TARGET_TYPE(i32);
pub struct D2D1_RENDER_TARGET_USAGE(i32);
pub struct D2D1_RESOURCE_TEXTURE_PROPERTIES(i32);
pub struct D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE(i32);
pub struct D2D1_RGBTOHUE_PROP(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ROUNDED_RECT(i32);
pub struct D2D1_SATURATION_PROP(i32);
pub struct D2D1_SCALE_INTERPOLATION_MODE(i32);
pub struct D2D1_SCALE_PROP(i32);
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
pub struct D2D1_SEPIA_PROP(i32);
pub struct D2D1_SHADOW_OPTIMIZATION(i32);
pub struct D2D1_SHADOW_PROP(i32);
pub struct D2D1_SHARPEN_PROP(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_SIMPLE_COLOR_PROFILE(i32);
pub struct D2D1_SPOTDIFFUSE_PROP(i32);
pub struct D2D1_SPOTDIFFUSE_SCALE_MODE(i32);
pub struct D2D1_SPOTSPECULAR_PROP(i32);
pub struct D2D1_SPOTSPECULAR_SCALE_MODE(i32);
pub struct D2D1_SPRITE_OPTIONS(i32);
pub struct D2D1_STRAIGHTEN_PROP(i32);
pub struct D2D1_STRAIGHTEN_SCALE_MODE(i32);
pub struct D2D1_STROKE_STYLE_PROPERTIES(i32);
pub struct D2D1_STROKE_STYLE_PROPERTIES1(i32);
pub struct D2D1_STROKE_TRANSFORM_TYPE(i32);
pub struct D2D1_SUBPROPERTY(i32);
pub struct D2D1_SVG_ASPECT_ALIGN(i32);
pub struct D2D1_SVG_ASPECT_SCALING(i32);
pub struct D2D1_SVG_ATTRIBUTE_POD_TYPE(i32);
pub struct D2D1_SVG_ATTRIBUTE_STRING_TYPE(i32);
pub struct D2D1_SVG_DISPLAY(i32);
pub struct D2D1_SVG_LENGTH(i32);
pub struct D2D1_SVG_LENGTH_UNITS(i32);
pub struct D2D1_SVG_LINE_CAP(i32);
pub struct D2D1_SVG_LINE_JOIN(i32);
pub struct D2D1_SVG_OVERFLOW(i32);
pub struct D2D1_SVG_PAINT_TYPE(i32);
pub struct D2D1_SVG_PATH_COMMAND(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO(i32);
pub struct D2D1_SVG_UNIT_TYPE(i32);
pub struct D2D1_SVG_VIEWBOX(i32);
pub struct D2D1_SVG_VISIBILITY(i32);
pub struct D2D1_SWEEP_DIRECTION(i32);
pub struct D2D1_TABLETRANSFER_PROP(i32);
pub struct D2D1_TEMPERATUREANDTINT_PROP(i32);
pub struct D2D1_TEXT_ANTIALIAS_MODE(i32);
pub struct D2D1_THREADING_MODE(i32);
pub struct D2D1_TILE_PROP(i32);
pub struct D2D1_TINT_PROP(i32);
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS(i32);
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_TRIANGLE(i32);
pub struct D2D1_TURBULENCE_PROP(i32);
pub struct D2D1_UNIT_MODE(i32);
pub struct D2D1_VERTEX_BUFFER_PROPERTIES(i32);
pub struct D2D1_VERTEX_OPTIONS(i32);
pub struct D2D1_VERTEX_RANGE(i32);
pub struct D2D1_VERTEX_USAGE(i32);
pub struct D2D1_VIGNETTE_PROP(i32);
pub struct D2D1_WHITELEVELADJUSTMENT_PROP(i32);
pub struct D2D1_WINDOW_STATE(i32);
pub struct D2D1_YCBCR_CHROMA_SUBSAMPLING(i32);
pub struct D2D1_YCBCR_INTERPOLATION_MODE(i32);
pub struct D2D1_YCBCR_PROP(i32);
pub const FACILITY_D2D: u32 = 2201u32;
#[repr(transparent)]
pub struct ID2D1AnalysisTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Bitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Bitmap1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BitmapBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BitmapBrush1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BitmapRenderTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BlendTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BorderTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1BoundsAdjustmentTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Brush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ColorContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ColorContext1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1CommandSink5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ComputeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ComputeTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ConcreteTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DCRenderTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Device6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DeviceContext6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DrawInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DrawTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DrawingStateBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1DrawingStateBlock1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Effect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1EffectContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1EffectContext1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1EffectContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1EffectImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1EllipseGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Factory7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GdiInteropRenderTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GdiMetafile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GdiMetafile1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GdiMetafileSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GdiMetafileSink1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Geometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GeometryGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GeometryRealization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GeometrySink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GradientMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GradientStopCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1GradientStopCollection1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1HwndRenderTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Image(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ImageBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ImageSourceFromWic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Ink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1InkStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Layer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1LinearGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1LookupTable3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Mesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Multithread(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1OffsetTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1PathGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1PathGeometry1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1PrintControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Properties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1RadialGradientBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1RectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1RenderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1RenderTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Resource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1ResourceTexture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1RoundedRectangleGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SolidColorBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SourceTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SpriteBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1StrokeStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1StrokeStyle1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgGlyphStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgPaint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgPathData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgPointCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1SvgStrokeDashArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1TessellationSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1Transform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1TransformGraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1TransformNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1TransformedGeometry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1TransformedImageSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ID2D1VertexBuffer(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix4x3F(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix4x4F(i32);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct Matrix5x4F(i32);
pub struct PD2D1_EFFECT_FACTORY(i32);
pub struct PD2D1_PROPERTY_GET_FUNCTION(i32);
pub struct PD2D1_PROPERTY_SET_FUNCTION(i32);
