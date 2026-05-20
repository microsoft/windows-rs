#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const HTTP_VERSION: windows_core::PCWSTR = windows_core::w!("HTTP/1.0");
