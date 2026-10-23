#[cfg(feature = "shared")]
windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_sys::core::PCSTR, lpqocinfo : LPQOCINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_sys::core::PCWSTR, lpqocinfo : LPQOCINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
pub type LPQOCINFO = *mut QOCINFO;
pub const NETWORK_ALIVE_AOL: i32 = 4;
pub const NETWORK_ALIVE_INTERNET: i32 = 8;
pub const NETWORK_ALIVE_LAN: i32 = 1;
pub const NETWORK_ALIVE_WAN: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QOCINFO {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwInSpeed: super::DWORD,
    pub dwOutSpeed: super::DWORD,
}
