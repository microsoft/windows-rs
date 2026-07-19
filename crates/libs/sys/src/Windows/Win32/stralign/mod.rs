#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrcmpW(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrcmpiW(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn uaw_lstrlenW(string : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcschr(string : *const u16, character : u16) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcscpy(destination : *mut u16, source : *const u16) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "C" fn uaw_wcsicmp(string1 : *const u16, string2 : *const u16) -> i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "C" fn uaw_wcslen(string : *const u16) -> usize);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "C" fn uaw_wcsrchr(string : *const u16, character : u16) -> super::PUWSTR);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PUWSTR_C = *const u16;
