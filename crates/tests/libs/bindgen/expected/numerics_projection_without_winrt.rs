#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_MATRIX_3X2_F {
    pub value: [f32; 6],
}
impl Default for D2D_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Values {
    pub matrix: D2D_MATRIX_3X2_F,
}
