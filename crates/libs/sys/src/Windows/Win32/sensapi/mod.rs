windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_sys::core::PCSTR, lpqocinfo : LPQOCINFO) -> windows_sys::core::BOOL);
windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_sys::core::PCWSTR, lpqocinfo : LPQOCINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : super::LPDWORD) -> windows_sys::core::BOOL);
pub type LPQOCINFO = *mut QOCINFO;
pub const NETWORK_ALIVE_AOL: i32 = 4;
pub const NETWORK_ALIVE_INTERNET: i32 = 8;
pub const NETWORK_ALIVE_LAN: i32 = 1;
pub const NETWORK_ALIVE_WAN: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
