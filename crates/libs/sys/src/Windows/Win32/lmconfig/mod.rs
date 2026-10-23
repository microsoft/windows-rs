#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetConfigGet(server : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, parameter : windows_sys::core::PCWSTR, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetConfigGetAll(server : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, bufptr : *mut super::LPBYTE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetConfigSet(server : windows_sys::core::PCWSTR, reserved1 : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, level : super::DWORD, reserved2 : super::DWORD, buf : super::LPBYTE, reserved3 : super::DWORD) -> super::DWORD);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: windows_sys::core::PWSTR,
    pub cfgi0_data: windows_sys::core::PWSTR,
}
pub type LPCONFIG_INFO_0 = *mut CONFIG_INFO_0;
pub type PCONFIG_INFO_0 = *mut CONFIG_INFO_0;
