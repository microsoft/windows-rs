#[cfg(feature = "winnt")]
pub type LPPROFILEINFO = LPPROFILEINFOA;
#[cfg(feature = "winnt")]
pub type LPPROFILEINFOA = *mut PROFILEINFOA;
#[cfg(feature = "winnt")]
pub type LPPROFILEINFOW = *mut PROFILEINFOW;
#[cfg(feature = "winnt")]
pub type PROFILEINFO = PROFILEINFOA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PROFILEINFOA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_sys::core::PSTR,
    pub lpProfilePath: windows_sys::core::PSTR,
    pub lpDefaultPath: windows_sys::core::PSTR,
    pub lpServerName: windows_sys::core::PSTR,
    pub lpPolicyPath: windows_sys::core::PSTR,
    pub hProfile: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for PROFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PROFILEINFOW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_sys::core::PWSTR,
    pub lpProfilePath: windows_sys::core::PWSTR,
    pub lpDefaultPath: windows_sys::core::PWSTR,
    pub lpServerName: windows_sys::core::PWSTR,
    pub lpPolicyPath: windows_sys::core::PWSTR,
    pub hProfile: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for PROFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
