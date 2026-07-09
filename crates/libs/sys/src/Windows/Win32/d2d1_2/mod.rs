#[cfg(feature = "Foundation_Numerics")]
windows_link::link!("d2d1.dll" "system" fn D2D1ComputeMaximumScaleFactor(matrix : *const super::super::Foundation::Numerics::Matrix3x2) -> f32);
pub type D2D1_RENDERING_PRIORITY = i32;
pub const D2D1_RENDERING_PRIORITY_FORCE_DWORD: D2D1_RENDERING_PRIORITY = -1;
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = 1;
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = 0;
