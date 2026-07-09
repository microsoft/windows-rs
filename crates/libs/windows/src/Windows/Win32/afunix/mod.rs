#[cfg(feature = "Win32_ws2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_UN(pub *mut SOCKADDR_UN);
#[cfg(feature = "Win32_ws2")]
impl PSOCKADDR_UN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_ws2")]
impl Default for PSOCKADDR_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIO_AF_UNIX_GETPEERPID: u32 = 1476395264;
pub const SIO_AF_UNIX_SETBINDPARENTPATH: i32 = -1744830207;
pub const SIO_AF_UNIX_SETCONNPARENTPATH: i32 = -1744830206;
#[repr(C)]
#[cfg(feature = "Win32_ws2")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOCKADDR_UN {
    pub sun_family: super::ws2::ADDRESS_FAMILY,
    pub sun_path: [i8; 108],
}
#[cfg(feature = "Win32_ws2")]
impl Default for SOCKADDR_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNIX_PATH_MAX: u32 = 108;
