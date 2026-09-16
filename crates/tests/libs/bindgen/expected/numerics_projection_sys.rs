windows_link::link!("test.dll" "system" fn UseMatrix(matrix : *const D2D1_MATRIX_3X2_F));
pub type D2D1_MATRIX_3X2_F = D2D_MATRIX_3X2_F;
pub type D2D1_MATRIX_4X4_F = D2D_MATRIX_4X4_F;
pub type D2D1_POINT_2F = D2D_POINT_2F;
pub type D2D1_VECTOR_2F = D2D_VECTOR_2F;
pub type D2D1_VECTOR_3F = D2D_VECTOR_3F;
pub type D2D1_VECTOR_4F = D2D_VECTOR_4F;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_MATRIX_3X2_F {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X4_F {
    pub value: [f32; 16],
}
impl Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DMATRIX {
    pub value: [f32; 16],
}
impl Default for D3DMATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Values {
    pub matrix3: D2D_MATRIX_3X2_F,
    pub matrix3_alias: D2D1_MATRIX_3X2_F,
    pub matrix4: D2D_MATRIX_4X4_F,
    pub matrix4_alias: D2D1_MATRIX_4X4_F,
    pub d3d_matrix: D3DMATRIX,
    pub point: D2D_POINT_2F,
    pub point_alias: D2D1_POINT_2F,
    pub vector2: D2D_VECTOR_2F,
    pub vector2_alias: D2D1_VECTOR_2F,
    pub vector3: D2D_VECTOR_3F,
    pub vector3_alias: D2D1_VECTOR_3F,
    pub vector4: D2D_VECTOR_4F,
    pub vector4_alias: D2D1_VECTOR_4F,
    pub native_matrix: DWRITE_MATRIX,
    pub native_size: D2D_SIZE_F,
}
