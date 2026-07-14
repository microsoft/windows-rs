pub type BigArray = [u16; 64];
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Struct {
    pub field: BigArray,
    pub other: i32,
}
impl Default for Struct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
