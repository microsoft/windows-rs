#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetConfigGet<P0, P1, P2>(server: P0, component: P1, parameter: P2, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetConfigGet(server : windows_core::PCWSTR, component : windows_core::PCWSTR, parameter : windows_core::PCWSTR, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetConfigGet(server.param().abi(), component.param().abi(), parameter.param().abi(), bufptr as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetConfigGetAll<P0, P1>(server: P0, component: P1, bufptr: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetConfigGetAll(server : windows_core::PCWSTR, component : windows_core::PCWSTR, bufptr : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetConfigGetAll(server.param().abi(), component.param().abi(), bufptr as _) }
}
#[inline]
pub unsafe fn NetConfigSet<P0, P1, P2>(server: P0, reserved1: P1, component: P2, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetConfigSet(server : windows_core::PCWSTR, reserved1 : windows_core::PCWSTR, component : windows_core::PCWSTR, level : u32, reserved2 : u32, buf : *mut u8, reserved3 : u32) -> u32);
    unsafe { NetConfigSet(server.param().abi(), reserved1.param().abi(), component.param().abi(), level, reserved2, buf as _, reserved3) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: windows_core::PWSTR,
    pub cfgi0_data: windows_core::PWSTR,
}
pub type LPCONFIG_INFO_0 = *mut CONFIG_INFO_0;
pub type PCONFIG_INFO_0 = *mut CONFIG_INFO_0;
