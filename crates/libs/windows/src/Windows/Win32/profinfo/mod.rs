#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROFILEINFO = *mut PROFILEINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROFILEINFOA = *mut PROFILEINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROFILEINFOW = *mut PROFILEINFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PROFILEINFO = PROFILEINFOA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILEINFOA {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub lpUserName: windows_core::PSTR,
    pub lpProfilePath: windows_core::PSTR,
    pub lpDefaultPath: windows_core::PSTR,
    pub lpServerName: windows_core::PSTR,
    pub lpPolicyPath: windows_core::PSTR,
    pub hProfile: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILEINFOW {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub lpUserName: windows_core::PWSTR,
    pub lpProfilePath: windows_core::PWSTR,
    pub lpDefaultPath: windows_core::PWSTR,
    pub lpServerName: windows_core::PWSTR,
    pub lpPolicyPath: windows_core::PWSTR,
    pub hProfile: super::HANDLE,
}
