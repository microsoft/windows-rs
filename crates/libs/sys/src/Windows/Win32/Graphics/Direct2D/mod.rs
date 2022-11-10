#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[cfg(feature = "Foundation_Numerics")]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1ComputeMaximumScaleFactor ( matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2 ) -> f32 );
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1ConvertColorSpace ( sourcecolorspace : D2D1_COLOR_SPACE , destinationcolorspace : D2D1_COLOR_SPACE , color : *const Common:: D2D1_COLOR_F ) -> Common:: D2D1_COLOR_F );
#[cfg(feature = "Win32_Graphics_Dxgi")]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1CreateDevice ( dxgidevice : super::Dxgi:: IDXGIDevice , creationproperties : *const D2D1_CREATION_PROPERTIES , d2ddevice : *mut ID2D1Device ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Graphics_Dxgi")]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1CreateDeviceContext ( dxgisurface : super::Dxgi:: IDXGISurface , creationproperties : *const D2D1_CREATION_PROPERTIES , d2ddevicecontext : *mut ID2D1DeviceContext ) -> :: windows_sys::core::HRESULT );
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1CreateFactory ( factorytype : D2D1_FACTORY_TYPE , riid : *const :: windows_sys::core::GUID , pfactoryoptions : *const D2D1_FACTORY_OPTIONS , ppifactory : *mut *mut ::core::ffi::c_void ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch ( ppoint0 : *const Common:: D2D_POINT_2F , ppoint1 : *const Common:: D2D_POINT_2F , ppoint2 : *const Common:: D2D_POINT_2F , ppoint3 : *const Common:: D2D_POINT_2F , ppoint4 : *const Common:: D2D_POINT_2F , ppoint5 : *const Common:: D2D_POINT_2F , ppoint6 : *const Common:: D2D_POINT_2F , ppoint7 : *const Common:: D2D_POINT_2F , ppoint8 : *const Common:: D2D_POINT_2F , ppoint9 : *const Common:: D2D_POINT_2F , ppoint10 : *const Common:: D2D_POINT_2F , ppoint11 : *const Common:: D2D_POINT_2F , ptensorpoint11 : *mut Common:: D2D_POINT_2F , ptensorpoint12 : *mut Common:: D2D_POINT_2F , ptensorpoint21 : *mut Common:: D2D_POINT_2F , ptensorpoint22 : *mut Common:: D2D_POINT_2F ) -> ( ) );
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1InvertMatrix ( matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation"))]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1IsMatrixInvertible ( matrix : *const super::super::super::Foundation::Numerics:: Matrix3x2 ) -> super::super::Foundation:: BOOL );
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1MakeRotateMatrix ( angle : f32 , center : Common:: D2D_POINT_2F , matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> ( ) );
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1MakeSkewMatrix ( anglex : f32 , angley : f32 , center : Common:: D2D_POINT_2F , matrix : *mut super::super::super::Foundation::Numerics:: Matrix3x2 ) -> ( ) );
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1SinCos ( angle : f32 , s : *mut f32 , c : *mut f32 ) -> ( ) );
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1Tan ( angle : f32 ) -> f32 );
::windows_sys::core::windows_link ! ( "d2d1.dll" ,"system" fn D2D1Vec3Length ( x : f32 , y : f32 , z : f32 ) -> f32 );
pub type ID2D1AnalysisTransform = *mut ::core::ffi::c_void;
pub type ID2D1Bitmap = *mut ::core::ffi::c_void;
pub type ID2D1Bitmap1 = *mut ::core::ffi::c_void;
pub type ID2D1BitmapBrush = *mut ::core::ffi::c_void;
pub type ID2D1BitmapBrush1 = *mut ::core::ffi::c_void;
pub type ID2D1BitmapRenderTarget = *mut ::core::ffi::c_void;
pub type ID2D1BlendTransform = *mut ::core::ffi::c_void;
pub type ID2D1BorderTransform = *mut ::core::ffi::c_void;
pub type ID2D1BoundsAdjustmentTransform = *mut ::core::ffi::c_void;
pub type ID2D1Brush = *mut ::core::ffi::c_void;
pub type ID2D1ColorContext = *mut ::core::ffi::c_void;
pub type ID2D1ColorContext1 = *mut ::core::ffi::c_void;
pub type ID2D1CommandList = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink1 = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink2 = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink3 = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink4 = *mut ::core::ffi::c_void;
pub type ID2D1CommandSink5 = *mut ::core::ffi::c_void;
pub type ID2D1ComputeInfo = *mut ::core::ffi::c_void;
pub type ID2D1ComputeTransform = *mut ::core::ffi::c_void;
pub type ID2D1ConcreteTransform = *mut ::core::ffi::c_void;
pub type ID2D1DCRenderTarget = *mut ::core::ffi::c_void;
pub type ID2D1Device = *mut ::core::ffi::c_void;
pub type ID2D1Device1 = *mut ::core::ffi::c_void;
pub type ID2D1Device2 = *mut ::core::ffi::c_void;
pub type ID2D1Device3 = *mut ::core::ffi::c_void;
pub type ID2D1Device4 = *mut ::core::ffi::c_void;
pub type ID2D1Device5 = *mut ::core::ffi::c_void;
pub type ID2D1Device6 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext1 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext2 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext3 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext4 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext5 = *mut ::core::ffi::c_void;
pub type ID2D1DeviceContext6 = *mut ::core::ffi::c_void;
pub type ID2D1DrawInfo = *mut ::core::ffi::c_void;
pub type ID2D1DrawTransform = *mut ::core::ffi::c_void;
pub type ID2D1DrawingStateBlock = *mut ::core::ffi::c_void;
pub type ID2D1DrawingStateBlock1 = *mut ::core::ffi::c_void;
pub type ID2D1Effect = *mut ::core::ffi::c_void;
pub type ID2D1EffectContext = *mut ::core::ffi::c_void;
pub type ID2D1EffectContext1 = *mut ::core::ffi::c_void;
pub type ID2D1EffectContext2 = *mut ::core::ffi::c_void;
pub type ID2D1EffectImpl = *mut ::core::ffi::c_void;
pub type ID2D1EllipseGeometry = *mut ::core::ffi::c_void;
pub type ID2D1Factory = *mut ::core::ffi::c_void;
pub type ID2D1Factory1 = *mut ::core::ffi::c_void;
pub type ID2D1Factory2 = *mut ::core::ffi::c_void;
pub type ID2D1Factory3 = *mut ::core::ffi::c_void;
pub type ID2D1Factory4 = *mut ::core::ffi::c_void;
pub type ID2D1Factory5 = *mut ::core::ffi::c_void;
pub type ID2D1Factory6 = *mut ::core::ffi::c_void;
pub type ID2D1Factory7 = *mut ::core::ffi::c_void;
pub type ID2D1GdiInteropRenderTarget = *mut ::core::ffi::c_void;
pub type ID2D1GdiMetafile = *mut ::core::ffi::c_void;
pub type ID2D1GdiMetafile1 = *mut ::core::ffi::c_void;
pub type ID2D1GdiMetafileSink = *mut ::core::ffi::c_void;
pub type ID2D1GdiMetafileSink1 = *mut ::core::ffi::c_void;
pub type ID2D1Geometry = *mut ::core::ffi::c_void;
pub type ID2D1GeometryGroup = *mut ::core::ffi::c_void;
pub type ID2D1GeometryRealization = *mut ::core::ffi::c_void;
pub type ID2D1GeometrySink = *mut ::core::ffi::c_void;
pub type ID2D1GradientMesh = *mut ::core::ffi::c_void;
pub type ID2D1GradientStopCollection = *mut ::core::ffi::c_void;
pub type ID2D1GradientStopCollection1 = *mut ::core::ffi::c_void;
pub type ID2D1HwndRenderTarget = *mut ::core::ffi::c_void;
pub type ID2D1Image = *mut ::core::ffi::c_void;
pub type ID2D1ImageBrush = *mut ::core::ffi::c_void;
pub type ID2D1ImageSource = *mut ::core::ffi::c_void;
pub type ID2D1ImageSourceFromWic = *mut ::core::ffi::c_void;
pub type ID2D1Ink = *mut ::core::ffi::c_void;
pub type ID2D1InkStyle = *mut ::core::ffi::c_void;
pub type ID2D1Layer = *mut ::core::ffi::c_void;
pub type ID2D1LinearGradientBrush = *mut ::core::ffi::c_void;
pub type ID2D1LookupTable3D = *mut ::core::ffi::c_void;
pub type ID2D1Mesh = *mut ::core::ffi::c_void;
pub type ID2D1Multithread = *mut ::core::ffi::c_void;
pub type ID2D1OffsetTransform = *mut ::core::ffi::c_void;
pub type ID2D1PathGeometry = *mut ::core::ffi::c_void;
pub type ID2D1PathGeometry1 = *mut ::core::ffi::c_void;
pub type ID2D1PrintControl = *mut ::core::ffi::c_void;
pub type ID2D1Properties = *mut ::core::ffi::c_void;
pub type ID2D1RadialGradientBrush = *mut ::core::ffi::c_void;
pub type ID2D1RectangleGeometry = *mut ::core::ffi::c_void;
pub type ID2D1RenderInfo = *mut ::core::ffi::c_void;
pub type ID2D1RenderTarget = *mut ::core::ffi::c_void;
pub type ID2D1Resource = *mut ::core::ffi::c_void;
pub type ID2D1ResourceTexture = *mut ::core::ffi::c_void;
pub type ID2D1RoundedRectangleGeometry = *mut ::core::ffi::c_void;
pub type ID2D1SolidColorBrush = *mut ::core::ffi::c_void;
pub type ID2D1SourceTransform = *mut ::core::ffi::c_void;
pub type ID2D1SpriteBatch = *mut ::core::ffi::c_void;
pub type ID2D1StrokeStyle = *mut ::core::ffi::c_void;
pub type ID2D1StrokeStyle1 = *mut ::core::ffi::c_void;
pub type ID2D1SvgAttribute = *mut ::core::ffi::c_void;
pub type ID2D1SvgDocument = *mut ::core::ffi::c_void;
pub type ID2D1SvgElement = *mut ::core::ffi::c_void;
pub type ID2D1SvgGlyphStyle = *mut ::core::ffi::c_void;
pub type ID2D1SvgPaint = *mut ::core::ffi::c_void;
pub type ID2D1SvgPathData = *mut ::core::ffi::c_void;
pub type ID2D1SvgPointCollection = *mut ::core::ffi::c_void;
pub type ID2D1SvgStrokeDashArray = *mut ::core::ffi::c_void;
pub type ID2D1TessellationSink = *mut ::core::ffi::c_void;
pub type ID2D1Transform = *mut ::core::ffi::c_void;
pub type ID2D1TransformGraph = *mut ::core::ffi::c_void;
pub type ID2D1TransformNode = *mut ::core::ffi::c_void;
pub type ID2D1TransformedGeometry = *mut ::core::ffi::c_void;
pub type ID2D1TransformedImageSource = *mut ::core::ffi::c_void;
pub type ID2D1VertexBuffer = *mut ::core::ffi::c_void;
pub const CLSID_D2D12DAffineTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6aa97485_6354_4cfc_908c_e4a74f62c96c);
pub const CLSID_D2D13DPerspectiveTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc2844d0b_3d86_46e7_85ba_526c9240f3fb);
pub const CLSID_D2D13DTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe8467b04_ec61_4b8a_b5de_d4d73debea5a);
pub const CLSID_D2D1AlphaMask: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc80ecff0_3fd5_4f05_8328_c5d1724b4f0a);
pub const CLSID_D2D1ArithmeticComposite: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc151437_049a_4784_a24a_f1c4daf20987);
pub const CLSID_D2D1Atlas: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x913e2be4_fdcf_4fe2_a5f0_2454f14ff408);
pub const CLSID_D2D1BitmapSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fb6c24d_c6dd_4231_9404_50f4d5c3252d);
pub const CLSID_D2D1Blend: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x81c5b77b_13f8_4cdd_ad20_c890547ac65d);
pub const CLSID_D2D1Border: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a2d49c0_4acf_43c7_8c6a_7c4a27874d27);
pub const CLSID_D2D1Brightness: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8cea8d1e_77b0_4986_b3b9_2f0c0eae7887);
pub const CLSID_D2D1ChromaKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x74c01f5b_2a0d_408c_88e2_c7a3c7197742);
pub const CLSID_D2D1ColorManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1a28524c_fdd6_4aa4_ae8f_837eb8267b37);
pub const CLSID_D2D1ColorMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x921f03d6_641c_47df_852d_b4bb6153ae11);
pub const CLSID_D2D1Composite: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x48fc9f51_f6ac_48f1_8b58_3b28ac46f76d);
pub const CLSID_D2D1Contrast: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb648a78a_0ed5_4f80_a94a_8e825aca6b77);
pub const CLSID_D2D1ConvolveMatrix: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x407f8c08_5533_4331_a341_23cc3877843e);
pub const CLSID_D2D1Crop: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe23f7110_0e9a_4324_af47_6a2c0c46f35b);
pub const CLSID_D2D1CrossFade: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x12f575e8_4db1_485f_9a84_03a07dd3829f);
pub const CLSID_D2D1DirectionalBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x174319a6_58e9_49b2_bb63_caf2c811a3db);
pub const CLSID_D2D1DiscreteTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x90866fcd_488e_454b_af06_e5041b66c36c);
pub const CLSID_D2D1DisplacementMap: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xedc48364_0417_4111_9450_43845fa9f890);
pub const CLSID_D2D1DistantDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3e7efd62_a32d_46d4_a83c_5278889ac954);
pub const CLSID_D2D1DistantSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x428c1ee5_77b8_4450_8ab5_72219c21abda);
pub const CLSID_D2D1DpiCompensation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c26c5c7_34e0_46fc_9cfd_e5823706e228);
pub const CLSID_D2D1EdgeDetection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeff583ca_cb07_4aa9_ac5d_2cc44c76460f);
pub const CLSID_D2D1Emboss: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb1c5eb2b_0348_43f0_8107_4957cacba2ae);
pub const CLSID_D2D1Exposure: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb56c8cfa_f634_41ee_bee0_ffa617106004);
pub const CLSID_D2D1Flood: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x61c23c20_ae69_4d8e_94cf_50078df638f2);
pub const CLSID_D2D1GammaTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x409444c4_c419_41a0_b0c1_8cd0c0a18e42);
pub const CLSID_D2D1GaussianBlur: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1feb6d69_2fe6_4ac9_8c58_1d7f93e7a6a5);
pub const CLSID_D2D1Grayscale: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36dde0eb_3725_42e0_836d_52fb20aee644);
pub const CLSID_D2D1HdrToneMap: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b0b748d_4610_4486_a90c_999d9a2e2b11);
pub const CLSID_D2D1HighlightsShadows: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcadc8384_323f_4c7e_a361_2e2b24df6ee4);
pub const CLSID_D2D1Histogram: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x881db7d0_f7ee_4d4d_a6d2_4697acc66ee8);
pub const CLSID_D2D1HueRotation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0f4458ec_4b32_491b_9e85_bd73f44d3eb6);
pub const CLSID_D2D1HueToRgb: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7b78a6bd_0141_4def_8a52_6356ee0cbdd5);
pub const CLSID_D2D1Invert: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0c3784d_cb39_4e84_b6fd_6b72f0810263);
pub const CLSID_D2D1LinearTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xad47c8fd_63ef_4acc_9b51_67979c036c06);
pub const CLSID_D2D1LookupTable3D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x349e0eda_0088_4a79_9ca3_c7e300202020);
pub const CLSID_D2D1LuminanceToAlpha: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41251ab7_0beb_46f8_9da7_59e93fcce5de);
pub const CLSID_D2D1Morphology: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeae6c40d_626a_4c2d_bfcb_391001abe202);
pub const CLSID_D2D1Opacity: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x811d79a4_de28_4454_8094_c64685f8bd4c);
pub const CLSID_D2D1OpacityMetadata: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6c53006a_4450_4199_aa5b_ad1656fece5e);
pub const CLSID_D2D1PointDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb9e303c3_c08c_4f91_8b7b_38656bc48c20);
pub const CLSID_D2D1PointSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x09c3ca26_3ae2_4f09_9ebc_ed3865d53f22);
pub const CLSID_D2D1Posterize: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2188945e_33a3_4366_b7bc_086bd02d0884);
pub const CLSID_D2D1Premultiply: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x06eab419_deed_4018_80d2_3e1d471adeb2);
pub const CLSID_D2D1RgbToHue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x23f3e5ec_91e8_4d3d_ad0a_afadc1004aa1);
pub const CLSID_D2D1Saturation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5cb2d9cf_327d_459f_a0ce_40c0b2086bf7);
pub const CLSID_D2D1Scale: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9daf9369_3846_4d0e_a44e_0c607934a5d7);
pub const CLSID_D2D1Sepia: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3a1af410_5f1d_4dbe_84df_915da79b7153);
pub const CLSID_D2D1Shadow: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc67ea361_1863_4e69_89db_695d3e9a5b6b);
pub const CLSID_D2D1Sharpen: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9b887cb_c5ff_4dc5_9779_273dcf417c7d);
pub const CLSID_D2D1SpotDiffuse: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x818a1105_7932_44f4_aa86_08ae7b2f2c93);
pub const CLSID_D2D1SpotSpecular: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xedae421e_7654_4a37_9db8_71acc1beb3c1);
pub const CLSID_D2D1Straighten: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4da47b12_79a3_4fb0_8237_bbc3b2a4de08);
pub const CLSID_D2D1TableTransfer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5bf818c3_5e43_48cb_b631_868396d6a1d4);
pub const CLSID_D2D1TemperatureTint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x89176087_8af9_4a08_aeb1_895f38db1766);
pub const CLSID_D2D1Tile: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb0784138_3b76_4bc5_b13b_0fa2ad02659f);
pub const CLSID_D2D1Tint: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36312b17_f7dd_4014_915d_ffca768cf211);
pub const CLSID_D2D1Turbulence: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcf2bb6ae_889a_4ad7_ba29_a2fd732c9fc9);
pub const CLSID_D2D1UnPremultiply: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb9ac489_ad8d_41ed_9999_bb6347d110f7);
pub const CLSID_D2D1Vignette: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc00c40be_5e67_4ca3_95b4_f4b02c115135);
pub const CLSID_D2D1WhiteLevelAdjustment: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x44a1cadb_6cdd_4818_8ff4_26c1cfe95bdb);
pub const CLSID_D2D1YCbCr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x99503cc1_66c7_45c9_a875_8ad8a7914401);
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCENE_REFERRED_SDR_WHITE_LEVEL: f32 = 80f32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const FACILITY_D2D: u32 = 2201u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_2DAFFINETRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_2DAFFINETRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE: D2D1_2DAFFINETRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_2DAFFINETRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS: D2D1_2DAFFINETRANSFORM_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_2DAFFINETRANSFORM_PROP_FORCE_DWORD: D2D1_2DAFFINETRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DPERSPECTIVETRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE: D2D1_3DPERSPECTIVETRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH: D2D1_3DPERSPECTIVETRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET: D2D1_3DPERSPECTIVETRANSFORM_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN: D2D1_3DPERSPECTIVETRANSFORM_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION: D2D1_3DPERSPECTIVETRANSFORM_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DPERSPECTIVETRANSFORM_PROP_FORCE_DWORD: D2D1_3DPERSPECTIVETRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DTRANSFORM_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_3DTRANSFORM_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_3DTRANSFORM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE: D2D1_3DTRANSFORM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_BORDER_MODE: D2D1_3DTRANSFORM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX: D2D1_3DTRANSFORM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_3DTRANSFORM_PROP_FORCE_DWORD: D2D1_3DTRANSFORM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ANTIALIAS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ARC_SIZE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARC_SIZE_FORCE_DWORD: D2D1_ARC_SIZE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ARITHMETICCOMPOSITE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS: D2D1_ARITHMETICCOMPOSITE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT: D2D1_ARITHMETICCOMPOSITE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ARITHMETICCOMPOSITE_PROP_FORCE_DWORD: D2D1_ARITHMETICCOMPOSITE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ATLAS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_RECT: D2D1_ATLAS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_INPUT_PADDING_RECT: D2D1_ATLAS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ATLAS_PROP_FORCE_DWORD: D2D1_ATLAS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_ALPHA_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED: D2D1_BITMAPSOURCE_ALPHA_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT: D2D1_BITMAPSOURCE_ALPHA_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ALPHA_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_ALPHA_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAPSOURCE_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_ORIENTATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT: D2D1_BITMAPSOURCE_ORIENTATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_BITMAPSOURCE_ORIENTATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_BITMAPSOURCE_ORIENTATION = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_BITMAPSOURCE_ORIENTATION = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_BITMAPSOURCE_ORIENTATION = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_ORIENTATION_FORCE_DWORD: D2D1_BITMAPSOURCE_ORIENTATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAPSOURCE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE: D2D1_BITMAPSOURCE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_SCALE: D2D1_BITMAPSOURCE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE: D2D1_BITMAPSOURCE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION: D2D1_BITMAPSOURCE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ALPHA_MODE: D2D1_BITMAPSOURCE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_ORIENTATION: D2D1_BITMAPSOURCE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAPSOURCE_PROP_FORCE_DWORD: D2D1_BITMAPSOURCE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAP_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD: D2D1_BITMAP_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BITMAP_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BITMAP_OPTIONS_FORCE_DWORD: D2D1_BITMAP_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ZERO: D2D1_BLEND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_ONE: D2D1_BLEND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_COLOR: D2D1_BLEND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_COLOR: D2D1_BLEND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA: D2D1_BLEND = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_SRC_ALPHA: D2D1_BLEND = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_ALPHA: D2D1_BLEND = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_ALPHA: D2D1_BLEND = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_DEST_COLOR: D2D1_BLEND = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_DEST_COLOR: D2D1_BLEND = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_SRC_ALPHA_SAT: D2D1_BLEND = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_BLEND_FACTOR: D2D1_BLEND = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_INV_BLEND_FACTOR: D2D1_BLEND = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_FORCE_DWORD: D2D1_BLEND = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_ADD: D2D1_BLEND_OPERATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_SUBTRACT: D2D1_BLEND_OPERATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_REV_SUBTRACT: D2D1_BLEND_OPERATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MIN: D2D1_BLEND_OPERATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_MAX: D2D1_BLEND_OPERATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_OPERATION_FORCE_DWORD: D2D1_BLEND_OPERATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BLEND_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_MODE: D2D1_BLEND_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BLEND_PROP_FORCE_DWORD: D2D1_BLEND_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BORDER_EDGE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_CLAMP: D2D1_BORDER_EDGE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_WRAP: D2D1_BORDER_EDGE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_MIRROR: D2D1_BORDER_EDGE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_EDGE_MODE_FORCE_DWORD: D2D1_BORDER_EDGE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BORDER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_X: D2D1_BORDER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_EDGE_MODE_Y: D2D1_BORDER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BORDER_PROP_FORCE_DWORD: D2D1_BORDER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BRIGHTNESS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_WHITE_POINT: D2D1_BRIGHTNESS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_BLACK_POINT: D2D1_BRIGHTNESS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BRIGHTNESS_PROP_FORCE_DWORD: D2D1_BRIGHTNESS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_BUFFER_PRECISION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_UNKNOWN: D2D1_BUFFER_PRECISION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM: D2D1_BUFFER_PRECISION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB: D2D1_BUFFER_PRECISION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_UNORM: D2D1_BUFFER_PRECISION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_16BPC_FLOAT: D2D1_BUFFER_PRECISION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_32BPC_FLOAT: D2D1_BUFFER_PRECISION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_BUFFER_PRECISION_FORCE_DWORD: D2D1_BUFFER_PRECISION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CAP_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CAP_STYLE_FORCE_DWORD: D2D1_CAP_STYLE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANGE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_NONE: D2D1_CHANGE_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_PROPERTIES: D2D1_CHANGE_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_CONTEXT: D2D1_CHANGE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_GRAPH: D2D1_CHANGE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANGE_TYPE_FORCE_DWORD: D2D1_CHANGE_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANNEL_DEPTH = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_DEFAULT: D2D1_CHANNEL_DEPTH = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_1: D2D1_CHANNEL_DEPTH = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_4: D2D1_CHANNEL_DEPTH = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_DEPTH_FORCE_DWORD: D2D1_CHANNEL_DEPTH = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHANNEL_SELECTOR = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_R: D2D1_CHANNEL_SELECTOR = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_G: D2D1_CHANNEL_SELECTOR = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_B: D2D1_CHANNEL_SELECTOR = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_A: D2D1_CHANNEL_SELECTOR = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHANNEL_SELECTOR_FORCE_DWORD: D2D1_CHANNEL_SELECTOR = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CHROMAKEY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_COLOR: D2D1_CHROMAKEY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_TOLERANCE: D2D1_CHROMAKEY_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_INVERT_ALPHA: D2D1_CHROMAKEY_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FEATHER: D2D1_CHROMAKEY_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CHROMAKEY_PROP_FORCE_DWORD: D2D1_CHROMAKEY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_ALPHA_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMANAGEMENT_ALPHA_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT: D2D1_COLORMANAGEMENT_ALPHA_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_ALPHA_MODE_FORCE_DWORD: D2D1_COLORMANAGEMENT_ALPHA_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT: D2D1_COLORMANAGEMENT_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT: D2D1_COLORMANAGEMENT_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE: D2D1_COLORMANAGEMENT_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_QUALITY: D2D1_COLORMANAGEMENT_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_PROP_FORCE_DWORD: D2D1_COLORMANAGEMENT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_QUALITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_PROOF: D2D1_COLORMANAGEMENT_QUALITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_NORMAL: D2D1_COLORMANAGEMENT_QUALITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_BEST: D2D1_COLORMANAGEMENT_QUALITY = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_QUALITY_FORCE_DWORD: D2D1_COLORMANAGEMENT_QUALITY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMANAGEMENT_RENDERING_INTENT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMANAGEMENT_RENDERING_INTENT_FORCE_DWORD: D2D1_COLORMANAGEMENT_RENDERING_INTENT = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLORMATRIX_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_COLOR_MATRIX: D2D1_COLORMATRIX_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_ALPHA_MODE: D2D1_COLORMATRIX_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT: D2D1_COLORMATRIX_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLORMATRIX_PROP_FORCE_DWORD: D2D1_COLORMATRIX_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_CONTEXT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD: D2D1_COLOR_CONTEXT_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT: D2D1_COLOR_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED: D2D1_COLOR_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_COLOR_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_CUSTOM: D2D1_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SRGB: D2D1_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_SCRGB: D2D1_COLOR_SPACE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COLOR_SPACE_FORCE_DWORD: D2D1_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMBINE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_UNION: D2D1_COMBINE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_INTERSECT: D2D1_COMBINE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_XOR: D2D1_COMBINE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_EXCLUDE: D2D1_COMBINE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMBINE_MODE_FORCE_DWORD: D2D1_COMBINE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_COMPOSITE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_MODE: D2D1_COMPOSITE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_COMPOSITE_PROP_FORCE_DWORD: D2D1_COMPOSITE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONTRAST_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CONTRAST: D2D1_CONTRAST_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_CLAMP_INPUT: D2D1_CONTRAST_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONTRAST_PROP_FORCE_DWORD: D2D1_CONTRAST_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONVOLVEMATRIX_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH: D2D1_CONVOLVEMATRIX_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE: D2D1_CONVOLVEMATRIX_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X: D2D1_CONVOLVEMATRIX_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y: D2D1_CONVOLVEMATRIX_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX: D2D1_CONVOLVEMATRIX_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_DIVISOR: D2D1_CONVOLVEMATRIX_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BIAS: D2D1_CONVOLVEMATRIX_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET: D2D1_CONVOLVEMATRIX_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA: D2D1_CONVOLVEMATRIX_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE: D2D1_CONVOLVEMATRIX_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT: D2D1_CONVOLVEMATRIX_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_PROP_FORCE_DWORD: D2D1_CONVOLVEMATRIX_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CONVOLVEMATRIX_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_CONVOLVEMATRIX_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_CONVOLVEMATRIX_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CONVOLVEMATRIX_SCALE_MODE_FORCE_DWORD: D2D1_CONVOLVEMATRIX_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CROP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_RECT: D2D1_CROP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_BORDER_MODE: D2D1_CROP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROP_PROP_FORCE_DWORD: D2D1_CROP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_CROSSFADE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_WEIGHT: D2D1_CROSSFADE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_CROSSFADE_PROP_FORCE_DWORD: D2D1_CROSSFADE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DASH_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DASH_STYLE_FORCE_DWORD: D2D1_DASH_STYLE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DC_INITIALIZE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_COPY: D2D1_DC_INITIALIZE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_CLEAR: D2D1_DC_INITIALIZE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DC_INITIALIZE_MODE_FORCE_DWORD: D2D1_DC_INITIALIZE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DEBUG_LEVEL = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEBUG_LEVEL_FORCE_DWORD: D2D1_DEBUG_LEVEL = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DEVICE_CONTEXT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD: D2D1_DEVICE_CONTEXT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DIRECTIONALBLUR_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_DIRECTIONALBLUR_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DIRECTIONALBLUR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION: D2D1_DIRECTIONALBLUR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_ANGLE: D2D1_DIRECTIONALBLUR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION: D2D1_DIRECTIONALBLUR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE: D2D1_DIRECTIONALBLUR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DIRECTIONALBLUR_PROP_FORCE_DWORD: D2D1_DIRECTIONALBLUR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISCRETETRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_TABLE: D2D1_DISCRETETRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_RED_DISABLE: D2D1_DISCRETETRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE: D2D1_DISCRETETRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE: D2D1_DISCRETETRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE: D2D1_DISCRETETRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE: D2D1_DISCRETETRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE: D2D1_DISCRETETRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE: D2D1_DISCRETETRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT: D2D1_DISCRETETRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISCRETETRANSFER_PROP_FORCE_DWORD: D2D1_DISCRETETRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISPLACEMENTMAP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_SCALE: D2D1_DISPLACEMENTMAP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT: D2D1_DISPLACEMENTMAP_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISPLACEMENTMAP_PROP_FORCE_DWORD: D2D1_DISPLACEMENTMAP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_AZIMUTH: D2D1_DISTANTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_ELEVATION: D2D1_DISTANTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_DISTANTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE: D2D1_DISTANTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_COLOR: D2D1_DISTANTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE: D2D1_DISTANTDIFFUSE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_PROP_FORCE_DWORD: D2D1_DISTANTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_AZIMUTH: D2D1_DISTANTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_ELEVATION: D2D1_DISTANTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_DISTANTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_DISTANTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE: D2D1_DISTANTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_COLOR: D2D1_DISTANTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_DISTANTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_SCALE_MODE: D2D1_DISTANTSPECULAR_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_PROP_FORCE_DWORD: D2D1_DISTANTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DISTANTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_DISTANTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_DISTANTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_DISTANTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DISTANTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_DISTANTSPECULAR_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DPICOMPENSATION_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_INTERPOLATION_MODE_FORCE_DWORD: D2D1_DPICOMPENSATION_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DPICOMPENSATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE: D2D1_DPICOMPENSATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_BORDER_MODE: D2D1_DPICOMPENSATION_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_INPUT_DPI: D2D1_DPICOMPENSATION_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DPICOMPENSATION_PROP_FORCE_DWORD: D2D1_DPICOMPENSATION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EDGEDETECTION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_SOBEL: D2D1_EDGEDETECTION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_PREWITT: D2D1_EDGEDETECTION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_MODE_FORCE_DWORD: D2D1_EDGEDETECTION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EDGEDETECTION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_STRENGTH: D2D1_EDGEDETECTION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_BLUR_RADIUS: D2D1_EDGEDETECTION_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_MODE: D2D1_EDGEDETECTION_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES: D2D1_EDGEDETECTION_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_ALPHA_MODE: D2D1_EDGEDETECTION_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EDGEDETECTION_PROP_FORCE_DWORD: D2D1_EDGEDETECTION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EMBOSS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_HEIGHT: D2D1_EMBOSS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_DIRECTION: D2D1_EMBOSS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EMBOSS_PROP_FORCE_DWORD: D2D1_EMBOSS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EXPOSURE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_EXPOSURE_VALUE: D2D1_EXPOSURE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXPOSURE_PROP_FORCE_DWORD: D2D1_EXPOSURE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_EXTEND_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FACTORY_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FACTORY_TYPE_FORCE_DWORD: D2D1_FACTORY_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FEATURE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_DOUBLES: D2D1_FEATURE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS: D2D1_FEATURE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_FORCE_DWORD: D2D1_FEATURE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FEATURE_LEVEL = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = 37120u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = 40960u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FEATURE_LEVEL_FORCE_DWORD: D2D1_FEATURE_LEVEL = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FILTER = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_POINT: D2D1_FILTER = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR: D2D1_FILTER = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: D2D1_FILTER = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR: D2D1_FILTER = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT: D2D1_FILTER = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: D2D1_FILTER = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT: D2D1_FILTER = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_MIN_MAG_MIP_LINEAR: D2D1_FILTER = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_ANISOTROPIC: D2D1_FILTER = 85u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FILTER_FORCE_DWORD: D2D1_FILTER = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_FLOOD_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_COLOR: D2D1_FLOOD_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_FLOOD_PROP_FORCE_DWORD: D2D1_FLOOD_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMA = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA_FORCE_DWORD: D2D1_GAMMA = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMA1 = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMA1_FORCE_DWORD: D2D1_GAMMA1 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAMMATRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_EXPONENT: D2D1_GAMMATRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_OFFSET: D2D1_GAMMATRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_RED_DISABLE: D2D1_GAMMATRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT: D2D1_GAMMATRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET: D2D1_GAMMATRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE: D2D1_GAMMATRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT: D2D1_GAMMATRANSFER_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET: D2D1_GAMMATRANSFER_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE: D2D1_GAMMATRANSFER_PROP = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE: D2D1_GAMMATRANSFER_PROP = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT: D2D1_GAMMATRANSFER_PROP = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET: D2D1_GAMMATRANSFER_PROP = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE: D2D1_GAMMATRANSFER_PROP = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT: D2D1_GAMMATRANSFER_PROP = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAMMATRANSFER_PROP_FORCE_DWORD: D2D1_GAMMATRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAUSSIANBLUR_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED: D2D1_GAUSSIANBLUR_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED: D2D1_GAUSSIANBLUR_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY: D2D1_GAUSSIANBLUR_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_OPTIMIZATION_FORCE_DWORD: D2D1_GAUSSIANBLUR_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GAUSSIANBLUR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION: D2D1_GAUSSIANBLUR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION: D2D1_GAUSSIANBLUR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_BORDER_MODE: D2D1_GAUSSIANBLUR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GAUSSIANBLUR_PROP_FORCE_DWORD: D2D1_GAUSSIANBLUR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GEOMETRY_RELATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_UNKNOWN: D2D1_GEOMETRY_RELATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_DISJOINT: D2D1_GEOMETRY_RELATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_IS_CONTAINED: D2D1_GEOMETRY_RELATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_CONTAINS: D2D1_GEOMETRY_RELATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_OVERLAP: D2D1_GEOMETRY_RELATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_RELATION_FORCE_DWORD: D2D1_GEOMETRY_RELATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_GEOMETRY_SIMPLIFICATION_OPTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_GEOMETRY_SIMPLIFICATION_OPTION_FORCE_DWORD: D2D1_GEOMETRY_SIMPLIFICATION_OPTION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HDRTONEMAP_DISPLAY_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_SDR: D2D1_HDRTONEMAP_DISPLAY_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_HDR: D2D1_HDRTONEMAP_DISPLAY_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_DISPLAY_MODE_FORCE_DWORD: D2D1_HDRTONEMAP_DISPLAY_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HDRTONEMAP_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE: D2D1_HDRTONEMAP_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_DISPLAY_MODE: D2D1_HDRTONEMAP_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HDRTONEMAP_PROP_FORCE_DWORD: D2D1_HDRTONEMAP_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HIGHLIGHTSANDSHADOWS_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HIGHLIGHTSANDSHADOWS_PROP_FORCE_DWORD: D2D1_HIGHLIGHTSANDSHADOWS_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HISTOGRAM_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_NUM_BINS: D2D1_HISTOGRAM_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_CHANNEL_SELECT: D2D1_HISTOGRAM_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT: D2D1_HISTOGRAM_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HISTOGRAM_PROP_FORCE_DWORD: D2D1_HISTOGRAM_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUEROTATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_ANGLE: D2D1_HUEROTATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUEROTATION_PROP_FORCE_DWORD: D2D1_HUEROTATION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUETORGB_INPUT_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_HUETORGB_INPUT_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_HUETORGB_INPUT_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_INPUT_COLOR_SPACE_FORCE_DWORD: D2D1_HUETORGB_INPUT_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_HUETORGB_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE: D2D1_HUETORGB_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_HUETORGB_PROP_FORCE_DWORD: D2D1_HUETORGB_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_IMAGE_SOURCE_LOADING_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INK_NIB_SHAPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INK_NIB_SHAPE_FORCE_DWORD: D2D1_INK_NIB_SHAPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_LINEAR: D2D1_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_CUBIC: D2D1_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_ANISOTROPIC: D2D1_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_FORCE_DWORD: D2D1_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_INTERPOLATION_MODE_DEFINITION = i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: D2D1_INTERPOLATION_MODE_DEFINITION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: D2D1_INTERPOLATION_MODE_DEFINITION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: D2D1_INTERPOLATION_MODE_DEFINITION = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: D2D1_INTERPOLATION_MODE_DEFINITION = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: D2D1_INTERPOLATION_MODE_DEFINITION = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LAYER_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS_FORCE_DWORD: D2D1_LAYER_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LAYER_OPTIONS1 = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_NONE: D2D1_LAYER_OPTIONS1 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND: D2D1_LAYER_OPTIONS1 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_IGNORE_ALPHA: D2D1_LAYER_OPTIONS1 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LAYER_OPTIONS1_FORCE_DWORD: D2D1_LAYER_OPTIONS1 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LINEARTRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_SLOPE: D2D1_LINEARTRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_RED_DISABLE: D2D1_LINEARTRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE: D2D1_LINEARTRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE: D2D1_LINEARTRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE: D2D1_LINEARTRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE: D2D1_LINEARTRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT: D2D1_LINEARTRANSFER_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE: D2D1_LINEARTRANSFER_PROP = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE: D2D1_LINEARTRANSFER_PROP = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT: D2D1_LINEARTRANSFER_PROP = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINEARTRANSFER_PROP_FORCE_DWORD: D2D1_LINEARTRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LINE_JOIN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LINE_JOIN_FORCE_DWORD: D2D1_LINE_JOIN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_LOOKUPTABLE3D_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_LUT: D2D1_LOOKUPTABLE3D_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE: D2D1_LOOKUPTABLE3D_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_LOOKUPTABLE3D_PROP_FORCE_DWORD: D2D1_LOOKUPTABLE3D_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MAP_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_NONE: D2D1_MAP_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_READ: D2D1_MAP_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_WRITE: D2D1_MAP_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_DISCARD: D2D1_MAP_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MAP_OPTIONS_FORCE_DWORD: D2D1_MAP_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MORPHOLOGY_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_ERODE: D2D1_MORPHOLOGY_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_DILATE: D2D1_MORPHOLOGY_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_MODE_FORCE_DWORD: D2D1_MORPHOLOGY_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_MORPHOLOGY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_MODE: D2D1_MORPHOLOGY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_WIDTH: D2D1_MORPHOLOGY_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_HEIGHT: D2D1_MORPHOLOGY_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_MORPHOLOGY_PROP_FORCE_DWORD: D2D1_MORPHOLOGY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITYMETADATA_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT: D2D1_OPACITYMETADATA_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITYMETADATA_PROP_FORCE_DWORD: D2D1_OPACITYMETADATA_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITY_MASK_CONTENT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD: D2D1_OPACITY_MASK_CONTENT = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_OPACITY_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_OPACITY: D2D1_OPACITY_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_OPACITY_PROP_FORCE_DWORD: D2D1_OPACITY_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_ORIENTATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_ORIENTATION_FORCE_DWORD: D2D1_ORIENTATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PATCH_EDGE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PATCH_EDGE_MODE_FORCE_DWORD: D2D1_PATCH_EDGE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PIXEL_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_NONE: D2D1_PIXEL_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING: D2D1_PIXEL_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PIXEL_OPTIONS_FORCE_DWORD: D2D1_PIXEL_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION: D2D1_POINTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_POINTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE: D2D1_POINTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_COLOR: D2D1_POINTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_SCALE_MODE: D2D1_POINTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_PROP_FORCE_DWORD: D2D1_POINTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_POINTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_POINTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_LIGHT_POSITION: D2D1_POINTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_POINTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_POINTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SURFACE_SCALE: D2D1_POINTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_COLOR: D2D1_POINTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_POINTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_SCALE_MODE: D2D1_POINTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_PROP_FORCE_DWORD: D2D1_POINTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POINTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_POINTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_POINTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_POINTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_POINTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POINTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_POINTSPECULAR_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_POSTERIZE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_RED_VALUE_COUNT: D2D1_POSTERIZE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT: D2D1_POSTERIZE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT: D2D1_POSTERIZE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_POSTERIZE_PROP_FORCE_DWORD: D2D1_POSTERIZE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRESENT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRESENT_OPTIONS_FORCE_DWORD: D2D1_PRESENT_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRIMITIVE_BLEND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRIMITIVE_BLEND_FORCE_DWORD: D2D1_PRIMITIVE_BLEND = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PRINT_FONT_SUBSET_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT: D2D1_PRINT_FONT_SUBSET_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE: D2D1_PRINT_FONT_SUBSET_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_NONE: D2D1_PRINT_FONT_SUBSET_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD: D2D1_PRINT_FONT_SUBSET_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PROPERTY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CLSID: D2D1_PROPERTY = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DISPLAYNAME: D2D1_PROPERTY = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_AUTHOR: D2D1_PROPERTY = 2147483650u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CATEGORY: D2D1_PROPERTY = 2147483651u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_DESCRIPTION: D2D1_PROPERTY = 2147483652u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_INPUTS: D2D1_PROPERTY = 2147483653u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_CACHED: D2D1_PROPERTY = 2147483654u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_PRECISION: D2D1_PROPERTY = 2147483655u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MIN_INPUTS: D2D1_PROPERTY = 2147483656u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_MAX_INPUTS: D2D1_PROPERTY = 2147483657u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_FORCE_DWORD: D2D1_PROPERTY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_PROPERTY_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_PROPERTY_TYPE_FORCE_DWORD: D2D1_PROPERTY_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDERING_PRIORITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDERING_PRIORITY_FORCE_DWORD: D2D1_RENDERING_PRIORITY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDER_TARGET_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_TYPE_FORCE_DWORD: D2D1_RENDER_TARGET_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RENDER_TARGET_USAGE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RENDER_TARGET_USAGE_FORCE_DWORD: D2D1_RENDER_TARGET_USAGE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_FORCE_DWORD: D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_RGBTOHUE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE: D2D1_RGBTOHUE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_RGBTOHUE_PROP_FORCE_DWORD: D2D1_RGBTOHUE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SATURATION_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_SATURATION: D2D1_SATURATION_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SATURATION_PROP_FORCE_DWORD: D2D1_SATURATION_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SCALE_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_SCALE_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_SCALE_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC: D2D1_SCALE_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_SCALE_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_INTERPOLATION_MODE_FORCE_DWORD: D2D1_SCALE_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SCALE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SCALE: D2D1_SCALE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_CENTER_POINT: D2D1_SCALE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_INTERPOLATION_MODE: D2D1_SCALE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_BORDER_MODE: D2D1_SCALE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_SHARPNESS: D2D1_SCALE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SCALE_PROP_FORCE_DWORD: D2D1_SCALE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SEPIA_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_INTENSITY: D2D1_SEPIA_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_ALPHA_MODE: D2D1_SEPIA_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SEPIA_PROP_FORCE_DWORD: D2D1_SEPIA_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHADOW_OPTIMIZATION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_SPEED: D2D1_SHADOW_OPTIMIZATION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_BALANCED: D2D1_SHADOW_OPTIMIZATION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_QUALITY: D2D1_SHADOW_OPTIMIZATION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_OPTIMIZATION_FORCE_DWORD: D2D1_SHADOW_OPTIMIZATION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHADOW_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION: D2D1_SHADOW_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_COLOR: D2D1_SHADOW_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_OPTIMIZATION: D2D1_SHADOW_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHADOW_PROP_FORCE_DWORD: D2D1_SHADOW_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SHARPEN_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_SHARPNESS: D2D1_SHARPEN_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_THRESHOLD: D2D1_SHARPEN_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SHARPEN_PROP_FORCE_DWORD: D2D1_SHARPEN_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTDIFFUSE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION: D2D1_SPOTDIFFUSE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_POINTS_AT: D2D1_SPOTDIFFUSE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FOCUS: D2D1_SPOTDIFFUSE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTDIFFUSE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT: D2D1_SPOTDIFFUSE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE: D2D1_SPOTDIFFUSE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_COLOR: D2D1_SPOTDIFFUSE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTDIFFUSE_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_SCALE_MODE: D2D1_SPOTDIFFUSE_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_PROP_FORCE_DWORD: D2D1_SPOTDIFFUSE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTDIFFUSE_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTDIFFUSE_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTDIFFUSE_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTDIFFUSE_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTDIFFUSE_SCALE_MODE_FORCE_DWORD: D2D1_SPOTDIFFUSE_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTSPECULAR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION: D2D1_SPOTSPECULAR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_POINTS_AT: D2D1_SPOTSPECULAR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FOCUS: D2D1_SPOTSPECULAR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE: D2D1_SPOTSPECULAR_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT: D2D1_SPOTSPECULAR_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT: D2D1_SPOTSPECULAR_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE: D2D1_SPOTSPECULAR_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_COLOR: D2D1_SPOTSPECULAR_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH: D2D1_SPOTSPECULAR_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_SCALE_MODE: D2D1_SPOTSPECULAR_PROP = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_PROP_FORCE_DWORD: D2D1_SPOTSPECULAR_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPOTSPECULAR_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_SPOTSPECULAR_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_SPOTSPECULAR_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC: D2D1_SPOTSPECULAR_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC: D2D1_SPOTSPECULAR_SCALE_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPOTSPECULAR_SCALE_MODE_FORCE_DWORD: D2D1_SPOTSPECULAR_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SPRITE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SPRITE_OPTIONS_FORCE_DWORD: D2D1_SPRITE_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STRAIGHTEN_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_ANGLE: D2D1_STRAIGHTEN_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE: D2D1_STRAIGHTEN_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_SCALE_MODE: D2D1_STRAIGHTEN_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_PROP_FORCE_DWORD: D2D1_STRAIGHTEN_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STRAIGHTEN_SCALE_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR: D2D1_STRAIGHTEN_SCALE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_CUBIC: D2D1_STRAIGHTEN_SCALE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR: D2D1_STRAIGHTEN_SCALE_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC: D2D1_STRAIGHTEN_SCALE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STRAIGHTEN_SCALE_MODE_FORCE_DWORD: D2D1_STRAIGHTEN_SCALE_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_STROKE_TRANSFORM_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD: D2D1_STROKE_TRANSFORM_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SUBPROPERTY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DISPLAYNAME: D2D1_SUBPROPERTY = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_ISREADONLY: D2D1_SUBPROPERTY = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MIN: D2D1_SUBPROPERTY = 2147483650u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_MAX: D2D1_SUBPROPERTY = 2147483651u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_DEFAULT: D2D1_SUBPROPERTY = 2147483652u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FIELDS: D2D1_SUBPROPERTY = 2147483653u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_INDEX: D2D1_SUBPROPERTY = 2147483654u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SUBPROPERTY_FORCE_DWORD: D2D1_SUBPROPERTY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ASPECT_ALIGN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_NONE: D2D1_SVG_ASPECT_ALIGN = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID: D2D1_SVG_ASPECT_ALIGN = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID: D2D1_SVG_ASPECT_ALIGN = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID: D2D1_SVG_ASPECT_ALIGN = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_ALIGN_FORCE_DWORD: D2D1_SVG_ASPECT_ALIGN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ASPECT_SCALING = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_MEET: D2D1_SVG_ASPECT_SCALING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_SLICE: D2D1_SVG_ASPECT_SCALING = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ASPECT_SCALING_FORCE_DWORD: D2D1_SVG_ASPECT_SCALING = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ATTRIBUTE_POD_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT: D2D1_SVG_ATTRIBUTE_POD_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR: D2D1_SVG_ATTRIBUTE_POD_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW: D2D1_SVG_ATTRIBUTE_POD_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP: D2D1_SVG_ATTRIBUTE_POD_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN: D2D1_SVG_ATTRIBUTE_POD_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO: D2D1_SVG_ATTRIBUTE_POD_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH: D2D1_SVG_ATTRIBUTE_POD_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_POD_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_ATTRIBUTE_STRING_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_DISPLAY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_INLINE: D2D1_SVG_DISPLAY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_NONE: D2D1_SVG_DISPLAY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_DISPLAY_FORCE_DWORD: D2D1_SVG_DISPLAY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LENGTH_UNITS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_NUMBER: D2D1_SVG_LENGTH_UNITS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_PERCENTAGE: D2D1_SVG_LENGTH_UNITS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LENGTH_UNITS_FORCE_DWORD: D2D1_SVG_LENGTH_UNITS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LINE_CAP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_BUTT: D2D1_SVG_LINE_CAP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_SQUARE: D2D1_SVG_LINE_CAP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_ROUND: D2D1_SVG_LINE_CAP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_CAP_FORCE_DWORD: D2D1_SVG_LINE_CAP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_LINE_JOIN = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_BEVEL: D2D1_SVG_LINE_JOIN = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_MITER: D2D1_SVG_LINE_JOIN = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_ROUND: D2D1_SVG_LINE_JOIN = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_LINE_JOIN_FORCE_DWORD: D2D1_SVG_LINE_JOIN = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_OVERFLOW = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_VISIBLE: D2D1_SVG_OVERFLOW = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_HIDDEN: D2D1_SVG_OVERFLOW = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_OVERFLOW_FORCE_DWORD: D2D1_SVG_OVERFLOW = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_PAINT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_NONE: D2D1_SVG_PAINT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_COLOR: D2D1_SVG_PAINT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI: D2D1_SVG_PAINT_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_NONE: D2D1_SVG_PAINT_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_COLOR: D2D1_SVG_PAINT_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PAINT_TYPE_FORCE_DWORD: D2D1_SVG_PAINT_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_PATH_COMMAND = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CLOSE_PATH: D2D1_SVG_PATH_COMMAND = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE: D2D1_SVG_PATH_COMMAND = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_LINE_RELATIVE: D2D1_SVG_PATH_COMMAND = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_ARC_RELATIVE: D2D1_SVG_PATH_COMMAND = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_PATH_COMMAND_FORCE_DWORD: D2D1_SVG_PATH_COMMAND = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_UNIT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE: D2D1_SVG_UNIT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX: D2D1_SVG_UNIT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_UNIT_TYPE_FORCE_DWORD: D2D1_SVG_UNIT_TYPE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SVG_VISIBILITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_VISIBLE: D2D1_SVG_VISIBILITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_HIDDEN: D2D1_SVG_VISIBILITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SVG_VISIBILITY_FORCE_DWORD: D2D1_SVG_VISIBILITY = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_SWEEP_DIRECTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_SWEEP_DIRECTION_FORCE_DWORD: D2D1_SWEEP_DIRECTION = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TABLETRANSFER_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_TABLE: D2D1_TABLETRANSFER_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_RED_DISABLE: D2D1_TABLETRANSFER_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_TABLE: D2D1_TABLETRANSFER_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_GREEN_DISABLE: D2D1_TABLETRANSFER_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_TABLE: D2D1_TABLETRANSFER_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_BLUE_DISABLE: D2D1_TABLETRANSFER_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_TABLE: D2D1_TABLETRANSFER_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE: D2D1_TABLETRANSFER_PROP = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT: D2D1_TABLETRANSFER_PROP = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TABLETRANSFER_PROP_FORCE_DWORD: D2D1_TABLETRANSFER_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TEMPERATUREANDTINT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE: D2D1_TEMPERATUREANDTINT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_TINT: D2D1_TEMPERATUREANDTINT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEMPERATUREANDTINT_PROP_FORCE_DWORD: D2D1_TEMPERATUREANDTINT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TEXT_ANTIALIAS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_THREADING_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_THREADING_MODE_FORCE_DWORD: D2D1_THREADING_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TILE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_RECT: D2D1_TILE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TILE_PROP_FORCE_DWORD: D2D1_TILE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TINT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_COLOR: D2D1_TINT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_CLAMP_OUTPUT: D2D1_TINT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TINT_PROP_FORCE_DWORD: D2D1_TINT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_TURBULENCE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_OFFSET: D2D1_TURBULENCE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SIZE: D2D1_TURBULENCE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_BASE_FREQUENCY: D2D1_TURBULENCE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NUM_OCTAVES: D2D1_TURBULENCE_PROP = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_SEED: D2D1_TURBULENCE_PROP = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_NOISE: D2D1_TURBULENCE_PROP = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_STITCHABLE: D2D1_TURBULENCE_PROP = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_TURBULENCE_PROP_FORCE_DWORD: D2D1_TURBULENCE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_UNIT_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_UNIT_MODE_FORCE_DWORD: D2D1_UNIT_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VERTEX_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_NONE: D2D1_VERTEX_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR: D2D1_VERTEX_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER: D2D1_VERTEX_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP: D2D1_VERTEX_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_OPTIONS_FORCE_DWORD: D2D1_VERTEX_OPTIONS = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VERTEX_USAGE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_STATIC: D2D1_VERTEX_USAGE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_DYNAMIC: D2D1_VERTEX_USAGE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VERTEX_USAGE_FORCE_DWORD: D2D1_VERTEX_USAGE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_VIGNETTE_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_COLOR: D2D1_VIGNETTE_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_TRANSITION_SIZE: D2D1_VIGNETTE_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_STRENGTH: D2D1_VIGNETTE_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_VIGNETTE_PROP_FORCE_DWORD: D2D1_VIGNETTE_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_WHITELEVELADJUSTMENT_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL: D2D1_WHITELEVELADJUSTMENT_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WHITELEVELADJUSTMENT_PROP_FORCE_DWORD: D2D1_WHITELEVELADJUSTMENT_PROP = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_WINDOW_STATE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_CHROMA_SUBSAMPLING = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO: D2D1_YCBCR_CHROMA_SUBSAMPLING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_420: D2D1_YCBCR_CHROMA_SUBSAMPLING = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_422: D2D1_YCBCR_CHROMA_SUBSAMPLING = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_444: D2D1_YCBCR_CHROMA_SUBSAMPLING = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_440: D2D1_YCBCR_CHROMA_SUBSAMPLING = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_CHROMA_SUBSAMPLING_FORCE_DWORD: D2D1_YCBCR_CHROMA_SUBSAMPLING = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_INTERPOLATION_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_YCBCR_INTERPOLATION_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_YCBCR_INTERPOLATION_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC: D2D1_YCBCR_INTERPOLATION_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_YCBCR_INTERPOLATION_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_INTERPOLATION_MODE_FORCE_DWORD: D2D1_YCBCR_INTERPOLATION_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type D2D1_YCBCR_PROP = u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING: D2D1_YCBCR_PROP = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_TRANSFORM_MATRIX: D2D1_YCBCR_PROP = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_INTERPOLATION_MODE: D2D1_YCBCR_PROP = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub const D2D1_YCBCR_PROP_FORCE_DWORD: D2D1_YCBCR_PROP = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ARC_SEGMENT {
    pub point: Common::D2D_POINT_2F,
    pub size: Common::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ARC_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ARC_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for D2D1_BITMAP_BRUSH_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_BITMAP_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: ID2D1ColorContext,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_BITMAP_PROPERTIES1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_BITMAP_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_BLEND_DESCRIPTION {
    pub sourceBlend: D2D1_BLEND,
    pub destinationBlend: D2D1_BLEND,
    pub blendOperation: D2D1_BLEND_OPERATION,
    pub sourceBlendAlpha: D2D1_BLEND,
    pub destinationBlendAlpha: D2D1_BLEND,
    pub blendOperationAlpha: D2D1_BLEND_OPERATION,
    pub blendFactor: [f32; 4],
}
impl ::core::marker::Copy for D2D1_BLEND_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_BLEND_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_BRUSH_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_CREATION_PROPERTIES {
    pub threadingMode: D2D1_THREADING_MODE,
    pub debugLevel: D2D1_DEBUG_LEVEL,
    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
}
impl ::core::marker::Copy for D2D1_CREATION_PROPERTIES {}
impl ::core::clone::Clone for D2D1_CREATION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    pub shaderBufferWithInputSignature: *const u8,
    pub shaderBufferSize: u32,
    pub inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    pub elementCount: u32,
    pub stride: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
    pub unitMode: D2D1_UNIT_MODE,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_DRAWING_STATE_DESCRIPTION1 {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: ID2D1Effect,
    pub inputIndex: u32,
    pub inputRectangle: Common::D2D_RECT_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_EFFECT_INPUT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ELLIPSE {
    pub point: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ELLIPSE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
impl ::core::marker::Copy for D2D1_FACTORY_OPTIONS {}
impl ::core::clone::Clone for D2D1_FACTORY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_FEATURE_DATA_DOUBLES {
    pub doublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_FEATURE_DATA_DOUBLES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: Common::D2D_POINT_2F,
    pub point01: Common::D2D_POINT_2F,
    pub point02: Common::D2D_POINT_2F,
    pub point03: Common::D2D_POINT_2F,
    pub point10: Common::D2D_POINT_2F,
    pub point11: Common::D2D_POINT_2F,
    pub point12: Common::D2D_POINT_2F,
    pub point13: Common::D2D_POINT_2F,
    pub point20: Common::D2D_POINT_2F,
    pub point21: Common::D2D_POINT_2F,
    pub point22: Common::D2D_POINT_2F,
    pub point23: Common::D2D_POINT_2F,
    pub point30: Common::D2D_POINT_2F,
    pub point31: Common::D2D_POINT_2F,
    pub point32: Common::D2D_POINT_2F,
    pub point33: Common::D2D_POINT_2F,
    pub color00: Common::D2D1_COLOR_F,
    pub color03: Common::D2D1_COLOR_F,
    pub color30: Common::D2D1_COLOR_F,
    pub color33: Common::D2D1_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_MESH_PATCH {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_MESH_PATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: Common::D2D1_COLOR_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_GRADIENT_STOP {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_GRADIENT_STOP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: super::super::Foundation::HWND,
    pub pixelSize: Common::D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_HWND_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: Common::D2D_RECT_F,
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_IMAGE_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
impl ::core::marker::Copy for D2D1_INK_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_INK_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for D2D1_INK_POINT {}
impl ::core::clone::Clone for D2D1_INK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: super::super::super::Foundation::Numerics::Matrix3x2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for D2D1_INK_STYLE_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for D2D1_INK_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_INPUT_DESCRIPTION {
    pub filter: D2D1_FILTER,
    pub levelOfDetailCount: u32,
}
impl ::core::marker::Copy for D2D1_INPUT_DESCRIPTION {}
impl ::core::clone::Clone for D2D1_INPUT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_INPUT_ELEMENT_DESC {
    pub semanticName: ::windows_sys::core::PCSTR,
    pub semanticIndex: u32,
    pub format: super::Dxgi::Common::DXGI_FORMAT,
    pub inputSlot: u32,
    pub alignedByteOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: ID2D1Geometry,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: ID2D1Brush,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_LAYER_PARAMETERS {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: Common::D2D_RECT_F,
    pub geometricMask: ID2D1Geometry,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: ID2D1Brush,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for D2D1_LAYER_PARAMETERS1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for D2D1_LAYER_PARAMETERS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: Common::D2D_POINT_2F,
    pub endPoint: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl ::core::marker::Copy for D2D1_MAPPED_RECT {}
impl ::core::clone::Clone for D2D1_MAPPED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_POINT_DESCRIPTION {
    pub point: Common::D2D_POINT_2F,
    pub unitTangentVector: Common::D2D_POINT_2F,
    pub endSegment: u32,
    pub endFigure: u32,
    pub lengthToEndSegment: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_POINT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_POINT_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
impl ::core::marker::Copy for D2D1_PRINT_CONTROL_PROPERTIES {}
impl ::core::clone::Clone for D2D1_PRINT_CONTROL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_PROPERTY_BINDING {
    pub propertyName: ::windows_sys::core::PCWSTR,
    pub setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    pub getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}
impl ::core::marker::Copy for D2D1_PROPERTY_BINDING {}
impl ::core::clone::Clone for D2D1_PROPERTY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_QUADRATIC_BEZIER_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: Common::D2D_POINT_2F,
    pub gradientOriginOffset: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: Common::D2D_SIZE_U,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_RENDERING_CONTROLS {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_RENDERING_CONTROLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D2D1_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D2D1_RENDER_TARGET_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    pub extents: *const u32,
    pub dimensions: u32,
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub channelDepth: D2D1_CHANNEL_DEPTH,
    pub filter: D2D1_FILTER,
    pub extendModes: *const D2D1_EXTEND_MODE,
}
impl ::core::marker::Copy for D2D1_RESOURCE_TEXTURE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_ROUNDED_RECT {
    pub rect: Common::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_ROUNDED_RECT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_ROUNDED_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: Common::D2D_POINT_2F,
    pub greenPrimary: Common::D2D_POINT_2F,
    pub bluePrimary: Common::D2D_POINT_2F,
    pub whitePointXZ: Common::D2D_POINT_2F,
    pub gamma: D2D1_GAMMA1,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_SIMPLE_COLOR_PROFILE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_SIMPLE_COLOR_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
}
impl ::core::marker::Copy for D2D1_STROKE_STYLE_PROPERTIES1 {}
impl ::core::clone::Clone for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_LENGTH {
    pub value: f32,
    pub units: D2D1_SVG_LENGTH_UNITS,
}
impl ::core::marker::Copy for D2D1_SVG_LENGTH {}
impl ::core::clone::Clone for D2D1_SVG_LENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    pub defer: super::super::Foundation::BOOL,
    pub align: D2D1_SVG_ASPECT_ALIGN,
    pub meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D2D1_SVG_PRESERVE_ASPECT_RATIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_SVG_VIEWBOX {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D1_SVG_VIEWBOX {}
impl ::core::clone::Clone for D2D1_SVG_VIEWBOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
impl ::core::marker::Copy for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {}
impl ::core::clone::Clone for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct D2D1_TRIANGLE {
    pub point1: Common::D2D_POINT_2F,
    pub point2: Common::D2D_POINT_2F,
    pub point3: Common::D2D_POINT_2F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for D2D1_TRIANGLE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for D2D1_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_BUFFER_PROPERTIES {
    pub inputCount: u32,
    pub usage: D2D1_VERTEX_USAGE,
    pub data: *const u8,
    pub byteWidth: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_BUFFER_PROPERTIES {}
impl ::core::clone::Clone for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub struct D2D1_VERTEX_RANGE {
    pub startVertex: u32,
    pub vertexCount: u32,
}
impl ::core::marker::Copy for D2D1_VERTEX_RANGE {}
impl ::core::clone::Clone for D2D1_VERTEX_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_EFFECT_FACTORY = ::core::option::Option<unsafe extern "system" fn(effectimpl: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_GET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: ::windows_sys::core::IUnknown, data: *mut u8, datasize: u32, actualsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
pub type PD2D1_PROPERTY_SET_FUNCTION = ::core::option::Option<unsafe extern "system" fn(effect: ::windows_sys::core::IUnknown, data: *const u8, datasize: u32) -> ::windows_sys::core::HRESULT>;
