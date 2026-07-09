#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDateFormatA(locale : super::winnt::LCID, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCSTR, lpdatestr : windows_sys::core::PSTR, cchdate : i32) -> i32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetDateFormatEx(lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCWSTR, lpdatestr : windows_sys::core::PWSTR, cchdate : i32, lpcalendar : windows_sys::core::PCWSTR) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDateFormatW(locale : super::winnt::LCID, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCWSTR, lpdatestr : windows_sys::core::PWSTR, cchdate : i32) -> i32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetDurationFormatEx(lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lpduration : *const super::minwinbase::SYSTEMTIME, ullduration : u64, lpformat : windows_sys::core::PCWSTR, lpdurationstr : windows_sys::core::PWSTR, cchduration : i32) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetTimeFormatA(locale : super::winnt::LCID, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCSTR, lptimestr : windows_sys::core::PSTR, cchtime : i32) -> i32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetTimeFormatEx(lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCWSTR, lptimestr : windows_sys::core::PWSTR, cchtime : i32) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetTimeFormatW(locale : super::winnt::LCID, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_sys::core::PCWSTR, lptimestr : windows_sys::core::PWSTR, cchtime : i32) -> i32);
