#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetConfigGet(server : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, parameter : windows_sys::core::PCWSTR, bufptr : *mut super::minwindef::LPBYTE) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetConfigGetAll(server : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, bufptr : *mut super::minwindef::LPBYTE) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetConfigSet(server : windows_sys::core::PCWSTR, reserved1 : windows_sys::core::PCWSTR, component : windows_sys::core::PCWSTR, level : u32, reserved2 : u32, buf : *mut u8, reserved3 : u32) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: windows_sys::core::PWSTR,
    pub cfgi0_data: windows_sys::core::PWSTR,
}
impl Default for CONFIG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPCONFIG_INFO_0 = *mut CONFIG_INFO_0;
pub type PCONFIG_INFO_0 = *mut CONFIG_INFO_0;
