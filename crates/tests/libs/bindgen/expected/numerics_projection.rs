#[inline]
pub unsafe fn UseMatrix(matrix: *const windows_numerics::Matrix3x2) {
    windows_core::link!("test.dll" "system" fn UseMatrix(matrix : *const windows_numerics::Matrix3x2));
    unsafe { UseMatrix(matrix) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Values {
    pub matrix3: windows_numerics::Matrix3x2,
    pub matrix3_alias: windows_numerics::Matrix3x2,
    pub matrix4: windows_numerics::Matrix4x4,
    pub matrix4_alias: windows_numerics::Matrix4x4,
    pub d3d_matrix: windows_numerics::Matrix4x4,
    pub point: windows_numerics::Vector2,
    pub point_alias: windows_numerics::Vector2,
    pub vector2: windows_numerics::Vector2,
    pub vector2_alias: windows_numerics::Vector2,
    pub vector3: windows_numerics::Vector3,
    pub vector3_alias: windows_numerics::Vector3,
    pub vector4: windows_numerics::Vector4,
    pub vector4_alias: windows_numerics::Vector4,
    pub native_matrix: DWRITE_MATRIX,
    pub native_size: D2D_SIZE_F,
    pub unrelated_point: D2D_POINT_2F,
    pub flat_point: windows_numerics::Vector2,
    pub root_point: windows_numerics::Vector2,
}
