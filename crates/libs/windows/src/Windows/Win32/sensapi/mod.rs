#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsDestinationReachableA(lpszdestination: &windows_core::PCSTR, lpqocinfo: LPQOCINFO) -> windows_core::BOOL {
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_core::PCSTR, lpqocinfo : LPQOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableA(core::mem::transmute_copy(lpszdestination), lpqocinfo as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsDestinationReachableW(lpszdestination: &windows_core::PCWSTR, lpqocinfo: LPQOCINFO) -> windows_core::BOOL {
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_core::PCWSTR, lpqocinfo : LPQOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableW(core::mem::transmute_copy(lpszdestination), lpqocinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { IsNetworkAlive(lpdwflags as _) }
}
#[cfg(feature = "shared")]
pub type LPQOCINFO = *mut QOCINFO;
pub const NETWORK_ALIVE_AOL: i32 = 4;
pub const NETWORK_ALIVE_INTERNET: i32 = 8;
pub const NETWORK_ALIVE_LAN: i32 = 1;
pub const NETWORK_ALIVE_WAN: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOCINFO {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwInSpeed: super::DWORD,
    pub dwOutSpeed: super::DWORD,
}
