#[cfg(feature = "Win32_ws2def")]
pub type PSOCKADDR_UN = *mut SOCKADDR_UN;
pub const SIO_AF_UNIX_GETPEERPID: u32 = 1476395264;
pub const SIO_AF_UNIX_SETBINDPARENTPATH: i32 = -1744830207;
pub const SIO_AF_UNIX_SETCONNPARENTPATH: i32 = -1744830206;
#[repr(C)]
#[cfg(feature = "Win32_ws2def")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_UN {
    pub sun_family: super::ws2def::ADDRESS_FAMILY,
    pub sun_path: [i8; 108],
}
#[cfg(feature = "Win32_ws2def")]
impl Default for SOCKADDR_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNIX_PATH_MAX: u32 = 108;
