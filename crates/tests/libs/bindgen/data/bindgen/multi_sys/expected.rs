#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const HTTP_VERSION: PCWSTR = {
    const VALUE: [u16; 9usize] = [72, 84, 84, 80, 47, 49, 46, 48, 0];
    VALUE.as_ptr()
};
pub type PCWSTR = *const u16;
