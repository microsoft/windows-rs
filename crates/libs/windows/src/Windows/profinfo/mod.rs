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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILEINFOA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_core::PSTR,
    pub lpProfilePath: windows_core::PSTR,
    pub lpDefaultPath: windows_core::PSTR,
    pub lpServerName: windows_core::PSTR,
    pub lpPolicyPath: windows_core::PSTR,
    pub hProfile: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILEINFOW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lpUserName: windows_core::PWSTR,
    pub lpProfilePath: windows_core::PWSTR,
    pub lpDefaultPath: windows_core::PWSTR,
    pub lpServerName: windows_core::PWSTR,
    pub lpPolicyPath: windows_core::PWSTR,
    pub hProfile: super::winnt::HANDLE,
}
