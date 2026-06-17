#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const HTTP_VERSION: PCWSTR = [72, 84, 84, 80, 47, 49, 46, 48, 0].as_ptr();
pub type PCWSTR = *const u16;
