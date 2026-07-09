#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPPROFILEINFO(pub LPPROFILEINFOA);
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROFILEINFOA(pub *mut PROFILEINFOA);
#[cfg(feature = "winnt")]
impl LPPROFILEINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPPROFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROFILEINFOW(pub *mut PROFILEINFOW);
#[cfg(feature = "winnt")]
impl LPPROFILEINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPPROFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type PROFILEINFO = PROFILEINFOA;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
