#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Graphics_CompositionSwapchain")]
pub mod CompositionSwapchain;
#[cfg(feature = "Win32_Graphics_DXCore")]
pub mod DXCore;
#[cfg(feature = "Win32_Graphics_Direct2D")]
pub mod Direct2D;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub mod Direct3D;
#[cfg(feature = "Win32_Graphics_Direct3D10")]
pub mod Direct3D10;
#[cfg(feature = "Win32_Graphics_Direct3D11")]
pub mod Direct3D11;
#[cfg(feature = "Win32_Graphics_Direct3D11on12")]
pub mod Direct3D11on12;
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub mod Direct3D12;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub mod Direct3D9;
#[cfg(feature = "Win32_Graphics_Direct3D9on12")]
pub mod Direct3D9on12;
#[cfg(feature = "Win32_Graphics_DirectComposition")]
pub mod DirectComposition;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub mod DirectDraw;
#[cfg(feature = "Win32_Graphics_DirectManipulation")]
pub mod DirectManipulation;
#[cfg(feature = "Win32_Graphics_DirectWrite")]
pub mod DirectWrite;
#[cfg(feature = "Win32_Graphics_Dwm")]
pub mod Dwm;
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub mod Dxgi;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub mod Gdi;
#[cfg(feature = "Win32_Graphics_Hlsl")]
pub mod Hlsl;
#[cfg(feature = "Win32_Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub mod OpenGL;
#[cfg(feature = "Win32_Graphics_Printing")]
pub mod Printing;
#[doc = "*Required features: `Win32_Graphics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(pub u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(0u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(1u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(2u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(3u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(4u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(5u32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(4294967295u32);
impl ::core::convert::From<u32> for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type Abi = Self;
}
impl ::core::ops::BitOr for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
