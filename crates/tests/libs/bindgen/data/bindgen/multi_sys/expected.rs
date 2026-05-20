#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const HTTP_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("HTTP/1.0");
