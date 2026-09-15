#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: super::PCUWSTR, string2: super::PCUWSTR) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
    unsafe { uaw_lstrcmpW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: super::PCUWSTR, string2: super::PCUWSTR) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
    unsafe { uaw_lstrcmpiW(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_lstrlenW(string: super::LPCUWSTR) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : super::LPCUWSTR) -> i32);
    unsafe { uaw_lstrlenW(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcschr(string: super::PCUWSTR, character: u16) -> super::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcschr(string : super::PCUWSTR, character : u16) -> super::PUWSTR);
    unsafe { uaw_wcschr(string, character) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcscpy(destination: super::PUWSTR, source: super::PCUWSTR) -> super::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcscpy(destination : super::PUWSTR, source : super::PCUWSTR) -> super::PUWSTR);
    unsafe { uaw_wcscpy(destination as _, source) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcsicmp(string1: super::PCUWSTR, string2: super::PCUWSTR) -> i32 {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcsicmp(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
    unsafe { uaw_wcsicmp(string1, string2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcslen(string: super::PCUWSTR) -> usize {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcslen(string : super::PCUWSTR) -> usize);
    unsafe { uaw_wcslen(string) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn uaw_wcsrchr(string: super::PCUWSTR, character: u16) -> super::PUWSTR {
    windows_core::link!("kernel32.dll" "C" fn uaw_wcsrchr(string : super::PCUWSTR, character : u16) -> super::PUWSTR);
    unsafe { uaw_wcsrchr(string, character) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PUWSTR_C = *const u16;
pub const _STRALIGN_USE_SECURE_CRT: i32 = 1;
