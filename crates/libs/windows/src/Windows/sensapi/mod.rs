#[inline]
pub unsafe fn IsDestinationReachableA<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_core::PCSTR, lpqocinfo : *mut QOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableA(lpszdestination.param().abi(), lpqocinfo as _) }
}
#[inline]
pub unsafe fn IsDestinationReachableW<P0>(lpszdestination: P0, lpqocinfo: *mut QOCINFO) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_core::PCWSTR, lpqocinfo : *mut QOCINFO) -> windows_core::BOOL);
    unsafe { IsDestinationReachableW(lpszdestination.param().abi(), lpqocinfo as _) }
}
#[inline]
pub unsafe fn IsNetworkAlive(lpdwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { IsNetworkAlive(lpdwflags as _) }
}
pub type LPQOCINFO = *mut QOCINFO;
pub const NETWORK_ALIVE_AOL: u32 = 4;
pub const NETWORK_ALIVE_INTERNET: u32 = 8;
pub const NETWORK_ALIVE_LAN: u32 = 1;
pub const NETWORK_ALIVE_WAN: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
