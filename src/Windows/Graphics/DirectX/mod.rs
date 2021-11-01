#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
pub mod Direct3D11;
#[doc = "*Required features: `Graphics_DirectX`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DirectXAlphaMode(pub i32);
impl DirectXAlphaMode {
    pub const Unspecified: DirectXAlphaMode = DirectXAlphaMode(0i32);
    pub const Premultiplied: DirectXAlphaMode = DirectXAlphaMode(1i32);
    pub const Straight: DirectXAlphaMode = DirectXAlphaMode(2i32);
    pub const Ignore: DirectXAlphaMode = DirectXAlphaMode(3i32);
}
impl ::std::convert::From<i32> for DirectXAlphaMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DirectXAlphaMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DirectXAlphaMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXAlphaMode;i4)");
}
#[doc = "*Required features: `Graphics_DirectX`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DirectXColorSpace(pub i32);
impl DirectXColorSpace {
    pub const RgbFullG22NoneP709: DirectXColorSpace = DirectXColorSpace(0i32);
    pub const RgbFullG10NoneP709: DirectXColorSpace = DirectXColorSpace(1i32);
    pub const RgbStudioG22NoneP709: DirectXColorSpace = DirectXColorSpace(2i32);
    pub const RgbStudioG22NoneP2020: DirectXColorSpace = DirectXColorSpace(3i32);
    pub const Reserved: DirectXColorSpace = DirectXColorSpace(4i32);
    pub const YccFullG22NoneP709X601: DirectXColorSpace = DirectXColorSpace(5i32);
    pub const YccStudioG22LeftP601: DirectXColorSpace = DirectXColorSpace(6i32);
    pub const YccFullG22LeftP601: DirectXColorSpace = DirectXColorSpace(7i32);
    pub const YccStudioG22LeftP709: DirectXColorSpace = DirectXColorSpace(8i32);
    pub const YccFullG22LeftP709: DirectXColorSpace = DirectXColorSpace(9i32);
    pub const YccStudioG22LeftP2020: DirectXColorSpace = DirectXColorSpace(10i32);
    pub const YccFullG22LeftP2020: DirectXColorSpace = DirectXColorSpace(11i32);
    pub const RgbFullG2084NoneP2020: DirectXColorSpace = DirectXColorSpace(12i32);
    pub const YccStudioG2084LeftP2020: DirectXColorSpace = DirectXColorSpace(13i32);
    pub const RgbStudioG2084NoneP2020: DirectXColorSpace = DirectXColorSpace(14i32);
    pub const YccStudioG22TopLeftP2020: DirectXColorSpace = DirectXColorSpace(15i32);
    pub const YccStudioG2084TopLeftP2020: DirectXColorSpace = DirectXColorSpace(16i32);
    pub const RgbFullG22NoneP2020: DirectXColorSpace = DirectXColorSpace(17i32);
    pub const YccStudioGHlgTopLeftP2020: DirectXColorSpace = DirectXColorSpace(18i32);
    pub const YccFullGHlgTopLeftP2020: DirectXColorSpace = DirectXColorSpace(19i32);
    pub const RgbStudioG24NoneP709: DirectXColorSpace = DirectXColorSpace(20i32);
    pub const RgbStudioG24NoneP2020: DirectXColorSpace = DirectXColorSpace(21i32);
    pub const YccStudioG24LeftP709: DirectXColorSpace = DirectXColorSpace(22i32);
    pub const YccStudioG24LeftP2020: DirectXColorSpace = DirectXColorSpace(23i32);
    pub const YccStudioG24TopLeftP2020: DirectXColorSpace = DirectXColorSpace(24i32);
}
impl ::std::convert::From<i32> for DirectXColorSpace {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DirectXColorSpace {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DirectXColorSpace {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXColorSpace;i4)");
}
#[doc = "*Required features: `Graphics_DirectX`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DirectXPixelFormat(pub i32);
impl DirectXPixelFormat {
    pub const Unknown: DirectXPixelFormat = DirectXPixelFormat(0i32);
    pub const R32G32B32A32Typeless: DirectXPixelFormat = DirectXPixelFormat(1i32);
    pub const R32G32B32A32Float: DirectXPixelFormat = DirectXPixelFormat(2i32);
    pub const R32G32B32A32UInt: DirectXPixelFormat = DirectXPixelFormat(3i32);
    pub const R32G32B32A32Int: DirectXPixelFormat = DirectXPixelFormat(4i32);
    pub const R32G32B32Typeless: DirectXPixelFormat = DirectXPixelFormat(5i32);
    pub const R32G32B32Float: DirectXPixelFormat = DirectXPixelFormat(6i32);
    pub const R32G32B32UInt: DirectXPixelFormat = DirectXPixelFormat(7i32);
    pub const R32G32B32Int: DirectXPixelFormat = DirectXPixelFormat(8i32);
    pub const R16G16B16A16Typeless: DirectXPixelFormat = DirectXPixelFormat(9i32);
    pub const R16G16B16A16Float: DirectXPixelFormat = DirectXPixelFormat(10i32);
    pub const R16G16B16A16UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(11i32);
    pub const R16G16B16A16UInt: DirectXPixelFormat = DirectXPixelFormat(12i32);
    pub const R16G16B16A16IntNormalized: DirectXPixelFormat = DirectXPixelFormat(13i32);
    pub const R16G16B16A16Int: DirectXPixelFormat = DirectXPixelFormat(14i32);
    pub const R32G32Typeless: DirectXPixelFormat = DirectXPixelFormat(15i32);
    pub const R32G32Float: DirectXPixelFormat = DirectXPixelFormat(16i32);
    pub const R32G32UInt: DirectXPixelFormat = DirectXPixelFormat(17i32);
    pub const R32G32Int: DirectXPixelFormat = DirectXPixelFormat(18i32);
    pub const R32G8X24Typeless: DirectXPixelFormat = DirectXPixelFormat(19i32);
    pub const D32FloatS8X24UInt: DirectXPixelFormat = DirectXPixelFormat(20i32);
    pub const R32FloatX8X24Typeless: DirectXPixelFormat = DirectXPixelFormat(21i32);
    pub const X32TypelessG8X24UInt: DirectXPixelFormat = DirectXPixelFormat(22i32);
    pub const R10G10B10A2Typeless: DirectXPixelFormat = DirectXPixelFormat(23i32);
    pub const R10G10B10A2UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(24i32);
    pub const R10G10B10A2UInt: DirectXPixelFormat = DirectXPixelFormat(25i32);
    pub const R11G11B10Float: DirectXPixelFormat = DirectXPixelFormat(26i32);
    pub const R8G8B8A8Typeless: DirectXPixelFormat = DirectXPixelFormat(27i32);
    pub const R8G8B8A8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(28i32);
    pub const R8G8B8A8UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(29i32);
    pub const R8G8B8A8UInt: DirectXPixelFormat = DirectXPixelFormat(30i32);
    pub const R8G8B8A8IntNormalized: DirectXPixelFormat = DirectXPixelFormat(31i32);
    pub const R8G8B8A8Int: DirectXPixelFormat = DirectXPixelFormat(32i32);
    pub const R16G16Typeless: DirectXPixelFormat = DirectXPixelFormat(33i32);
    pub const R16G16Float: DirectXPixelFormat = DirectXPixelFormat(34i32);
    pub const R16G16UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(35i32);
    pub const R16G16UInt: DirectXPixelFormat = DirectXPixelFormat(36i32);
    pub const R16G16IntNormalized: DirectXPixelFormat = DirectXPixelFormat(37i32);
    pub const R16G16Int: DirectXPixelFormat = DirectXPixelFormat(38i32);
    pub const R32Typeless: DirectXPixelFormat = DirectXPixelFormat(39i32);
    pub const D32Float: DirectXPixelFormat = DirectXPixelFormat(40i32);
    pub const R32Float: DirectXPixelFormat = DirectXPixelFormat(41i32);
    pub const R32UInt: DirectXPixelFormat = DirectXPixelFormat(42i32);
    pub const R32Int: DirectXPixelFormat = DirectXPixelFormat(43i32);
    pub const R24G8Typeless: DirectXPixelFormat = DirectXPixelFormat(44i32);
    pub const D24UIntNormalizedS8UInt: DirectXPixelFormat = DirectXPixelFormat(45i32);
    pub const R24UIntNormalizedX8Typeless: DirectXPixelFormat = DirectXPixelFormat(46i32);
    pub const X24TypelessG8UInt: DirectXPixelFormat = DirectXPixelFormat(47i32);
    pub const R8G8Typeless: DirectXPixelFormat = DirectXPixelFormat(48i32);
    pub const R8G8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(49i32);
    pub const R8G8UInt: DirectXPixelFormat = DirectXPixelFormat(50i32);
    pub const R8G8IntNormalized: DirectXPixelFormat = DirectXPixelFormat(51i32);
    pub const R8G8Int: DirectXPixelFormat = DirectXPixelFormat(52i32);
    pub const R16Typeless: DirectXPixelFormat = DirectXPixelFormat(53i32);
    pub const R16Float: DirectXPixelFormat = DirectXPixelFormat(54i32);
    pub const D16UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(55i32);
    pub const R16UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(56i32);
    pub const R16UInt: DirectXPixelFormat = DirectXPixelFormat(57i32);
    pub const R16IntNormalized: DirectXPixelFormat = DirectXPixelFormat(58i32);
    pub const R16Int: DirectXPixelFormat = DirectXPixelFormat(59i32);
    pub const R8Typeless: DirectXPixelFormat = DirectXPixelFormat(60i32);
    pub const R8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(61i32);
    pub const R8UInt: DirectXPixelFormat = DirectXPixelFormat(62i32);
    pub const R8IntNormalized: DirectXPixelFormat = DirectXPixelFormat(63i32);
    pub const R8Int: DirectXPixelFormat = DirectXPixelFormat(64i32);
    pub const A8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(65i32);
    pub const R1UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(66i32);
    pub const R9G9B9E5SharedExponent: DirectXPixelFormat = DirectXPixelFormat(67i32);
    pub const R8G8B8G8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(68i32);
    pub const G8R8G8B8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(69i32);
    pub const BC1Typeless: DirectXPixelFormat = DirectXPixelFormat(70i32);
    pub const BC1UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(71i32);
    pub const BC1UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(72i32);
    pub const BC2Typeless: DirectXPixelFormat = DirectXPixelFormat(73i32);
    pub const BC2UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(74i32);
    pub const BC2UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(75i32);
    pub const BC3Typeless: DirectXPixelFormat = DirectXPixelFormat(76i32);
    pub const BC3UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(77i32);
    pub const BC3UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(78i32);
    pub const BC4Typeless: DirectXPixelFormat = DirectXPixelFormat(79i32);
    pub const BC4UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(80i32);
    pub const BC4IntNormalized: DirectXPixelFormat = DirectXPixelFormat(81i32);
    pub const BC5Typeless: DirectXPixelFormat = DirectXPixelFormat(82i32);
    pub const BC5UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(83i32);
    pub const BC5IntNormalized: DirectXPixelFormat = DirectXPixelFormat(84i32);
    pub const B5G6R5UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(85i32);
    pub const B5G5R5A1UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(86i32);
    pub const B8G8R8A8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(87i32);
    pub const B8G8R8X8UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(88i32);
    pub const R10G10B10XRBiasA2UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(89i32);
    pub const B8G8R8A8Typeless: DirectXPixelFormat = DirectXPixelFormat(90i32);
    pub const B8G8R8A8UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(91i32);
    pub const B8G8R8X8Typeless: DirectXPixelFormat = DirectXPixelFormat(92i32);
    pub const B8G8R8X8UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(93i32);
    pub const BC6HTypeless: DirectXPixelFormat = DirectXPixelFormat(94i32);
    pub const BC6H16UnsignedFloat: DirectXPixelFormat = DirectXPixelFormat(95i32);
    pub const BC6H16Float: DirectXPixelFormat = DirectXPixelFormat(96i32);
    pub const BC7Typeless: DirectXPixelFormat = DirectXPixelFormat(97i32);
    pub const BC7UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(98i32);
    pub const BC7UIntNormalizedSrgb: DirectXPixelFormat = DirectXPixelFormat(99i32);
    pub const Ayuv: DirectXPixelFormat = DirectXPixelFormat(100i32);
    pub const Y410: DirectXPixelFormat = DirectXPixelFormat(101i32);
    pub const Y416: DirectXPixelFormat = DirectXPixelFormat(102i32);
    pub const NV12: DirectXPixelFormat = DirectXPixelFormat(103i32);
    pub const P010: DirectXPixelFormat = DirectXPixelFormat(104i32);
    pub const P016: DirectXPixelFormat = DirectXPixelFormat(105i32);
    pub const Opaque420: DirectXPixelFormat = DirectXPixelFormat(106i32);
    pub const Yuy2: DirectXPixelFormat = DirectXPixelFormat(107i32);
    pub const Y210: DirectXPixelFormat = DirectXPixelFormat(108i32);
    pub const Y216: DirectXPixelFormat = DirectXPixelFormat(109i32);
    pub const NV11: DirectXPixelFormat = DirectXPixelFormat(110i32);
    pub const AI44: DirectXPixelFormat = DirectXPixelFormat(111i32);
    pub const IA44: DirectXPixelFormat = DirectXPixelFormat(112i32);
    pub const P8: DirectXPixelFormat = DirectXPixelFormat(113i32);
    pub const A8P8: DirectXPixelFormat = DirectXPixelFormat(114i32);
    pub const B4G4R4A4UIntNormalized: DirectXPixelFormat = DirectXPixelFormat(115i32);
    pub const P208: DirectXPixelFormat = DirectXPixelFormat(130i32);
    pub const V208: DirectXPixelFormat = DirectXPixelFormat(131i32);
    pub const V408: DirectXPixelFormat = DirectXPixelFormat(132i32);
    pub const SamplerFeedbackMinMipOpaque: DirectXPixelFormat = DirectXPixelFormat(189i32);
    pub const SamplerFeedbackMipRegionUsedOpaque: DirectXPixelFormat = DirectXPixelFormat(190i32);
}
impl ::std::convert::From<i32> for DirectXPixelFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DirectXPixelFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DirectXPixelFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4)");
}
#[doc = "*Required features: `Graphics_DirectX`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DirectXPrimitiveTopology(pub i32);
impl DirectXPrimitiveTopology {
    pub const Undefined: DirectXPrimitiveTopology = DirectXPrimitiveTopology(0i32);
    pub const PointList: DirectXPrimitiveTopology = DirectXPrimitiveTopology(1i32);
    pub const LineList: DirectXPrimitiveTopology = DirectXPrimitiveTopology(2i32);
    pub const LineStrip: DirectXPrimitiveTopology = DirectXPrimitiveTopology(3i32);
    pub const TriangleList: DirectXPrimitiveTopology = DirectXPrimitiveTopology(4i32);
    pub const TriangleStrip: DirectXPrimitiveTopology = DirectXPrimitiveTopology(5i32);
}
impl ::std::convert::From<i32> for DirectXPrimitiveTopology {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DirectXPrimitiveTopology {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DirectXPrimitiveTopology {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXPrimitiveTopology;i4)");
}
