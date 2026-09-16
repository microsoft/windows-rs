#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Values {
    pub matrix: windows_numerics::Matrix3x2,
    pub point: windows_numerics::Vector2,
}
