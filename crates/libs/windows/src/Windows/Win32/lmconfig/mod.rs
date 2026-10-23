#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetConfigGet(server: &windows_core::PCWSTR, component: &windows_core::PCWSTR, parameter: &windows_core::PCWSTR, bufptr: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetConfigGet(server : windows_core::PCWSTR, component : windows_core::PCWSTR, parameter : windows_core::PCWSTR, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetConfigGet(core::mem::transmute_copy(server), core::mem::transmute_copy(component), core::mem::transmute_copy(parameter), bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetConfigGetAll(server: &windows_core::PCWSTR, component: &windows_core::PCWSTR, bufptr: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetConfigGetAll(server : windows_core::PCWSTR, component : windows_core::PCWSTR, bufptr : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetConfigGetAll(core::mem::transmute_copy(server), core::mem::transmute_copy(component), bufptr as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetConfigSet(server: &windows_core::PCWSTR, reserved1: &windows_core::PCWSTR, component: &windows_core::PCWSTR, level: super::DWORD, reserved2: super::DWORD, buf: super::LPBYTE, reserved3: super::DWORD) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetConfigSet(server : windows_core::PCWSTR, reserved1 : windows_core::PCWSTR, component : windows_core::PCWSTR, level : super::DWORD, reserved2 : super::DWORD, buf : super::LPBYTE, reserved3 : super::DWORD) -> super::DWORD);
    unsafe { NetConfigSet(core::mem::transmute_copy(server), core::mem::transmute_copy(reserved1), core::mem::transmute_copy(component), level, reserved2, buf as _, reserved3) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: windows_core::PWSTR,
    pub cfgi0_data: windows_core::PWSTR,
}
pub type LPCONFIG_INFO_0 = *mut CONFIG_INFO_0;
pub type PCONFIG_INFO_0 = *mut CONFIG_INFO_0;
