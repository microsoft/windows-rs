windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_sys::core::PCSTR, lpqocinfo : *mut QOCINFO) -> windows_sys::core::BOOL);
windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_sys::core::PCWSTR, lpqocinfo : *mut QOCINFO) -> windows_sys::core::BOOL);
windows_link::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : *mut u32) -> windows_sys::core::BOOL);
pub type LPQOCINFO = *mut QOCINFO;
pub const NETWORK_ALIVE_AOL: u32 = 4;
pub const NETWORK_ALIVE_INTERNET: u32 = 8;
pub const NETWORK_ALIVE_LAN: u32 = 1;
pub const NETWORK_ALIVE_WAN: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
