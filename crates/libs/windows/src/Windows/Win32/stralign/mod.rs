#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_lstrcmpW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_lstrcmpiW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrlenW(string: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : *const u16) -> i32);
    unsafe { uaw_lstrlenW(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcschr(string: *const u16, character: u16) -> super::winnt::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcschr(string : *const u16, character : u16) -> super::winnt::PUWSTR);
    unsafe { uaw_wcschr(string, character) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> super::winnt::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcscpy(destination : *mut u16, source : *const u16) -> super::winnt::PUWSTR);
    unsafe { uaw_wcscpy(destination as _, source) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcsicmp(string1 : *const u16, string2 : *const u16) -> i32);
    unsafe { uaw_wcsicmp(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcslen(string: *const u16) -> usize {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcslen(string : *const u16) -> usize);
    unsafe { uaw_wcslen(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcsrchr(string: *const u16, character: u16) -> super::winnt::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcsrchr(string : *const u16, character : u16) -> super::winnt::PUWSTR);
    unsafe { uaw_wcsrchr(string, character) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUWSTR_C(pub *const u16);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PUWSTR_C {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUWSTR_C {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
