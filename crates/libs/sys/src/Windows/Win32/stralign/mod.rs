#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : super::LPCUWSTR) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcschr(string : super::PCUWSTR, character : u16) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcscpy(destination : super::PUWSTR, source : super::PCUWSTR) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcsicmp(string1 : super::PCUWSTR, string2 : super::PCUWSTR) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcslen(string : super::PCUWSTR) -> usize);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcsrchr(string : super::PCUWSTR, character : u16) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PUWSTR_C = *const u16;
pub const _STRALIGN_USE_SECURE_CRT: i32 = 1;
