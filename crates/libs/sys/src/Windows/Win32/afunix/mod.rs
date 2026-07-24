#[cfg(feature = "ws2")]
pub type PSOCKADDR_UN = *mut SOCKADDR_UN;
pub const SIO_AF_UNIX_GETPEERPID: i32 = 1476395264;
pub const SIO_AF_UNIX_SETBINDPARENTPATH: u32 = 2550137089;
pub const SIO_AF_UNIX_SETCONNPARENTPATH: u32 = 2550137090;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_UN {
    pub sun_family: super::ADDRESS_FAMILY,
    pub sun_path: [i8; 108],
}
#[cfg(feature = "ws2")]
impl Default for SOCKADDR_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNIX_PATH_MAX: i32 = 108;
