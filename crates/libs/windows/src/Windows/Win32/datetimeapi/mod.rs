#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetDateFormatA<P3>(locale: super::LCID, dwflags: super::DWORD, lpdate: Option<*const super::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<windows_core::PSTR>, cchdate: i32) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatA(locale : super::LCID, dwflags : super::DWORD, lpdate : *const super::SYSTEMTIME, lpformat : windows_core::PCSTR, lpdatestr : windows_core::PSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatA(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lpdatestr.unwrap_or(core::mem::zeroed()) as _, cchdate) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetDateFormatEx<P0, P3, P6>(lplocalename: P0, dwflags: super::DWORD, lpdate: Option<*const super::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<windows_core::PWSTR>, cchdate: i32, lpcalendar: P6) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatEx(lplocalename : windows_core::PCWSTR, dwflags : super::DWORD, lpdate : *const super::SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32, lpcalendar : windows_core::PCWSTR) -> i32);
    unsafe { GetDateFormatEx(lplocalename.param().abi(), dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lpdatestr.unwrap_or(core::mem::zeroed()) as _, cchdate, lpcalendar.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetDateFormatW<P3>(locale: super::LCID, dwflags: super::DWORD, lpdate: Option<*const super::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<windows_core::PWSTR>, cchdate: i32) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatW(locale : super::LCID, dwflags : super::DWORD, lpdate : *const super::SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatW(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lpdatestr.unwrap_or(core::mem::zeroed()) as _, cchdate) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetDurationFormatEx<P0, P4>(lplocalename: P0, dwflags: super::DWORD, lpduration: Option<*const super::SYSTEMTIME>, ullduration: super::ULONGLONG, lpformat: P4, lpdurationstr: Option<windows_core::PWSTR>, cchduration: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDurationFormatEx(lplocalename : windows_core::PCWSTR, dwflags : super::DWORD, lpduration : *const super::SYSTEMTIME, ullduration : super::ULONGLONG, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    unsafe { GetDurationFormatEx(lplocalename.param().abi(), dwflags, lpduration.unwrap_or(core::mem::zeroed()) as _, ullduration, lpformat.param().abi(), lpdurationstr.unwrap_or(core::mem::zeroed()) as _, cchduration) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetTimeFormatA<P3>(locale: super::LCID, dwflags: super::DWORD, lptime: Option<*const super::SYSTEMTIME>, lpformat: P3, lptimestr: Option<windows_core::PSTR>, cchtime: i32) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatA(locale : super::LCID, dwflags : super::DWORD, lptime : *const super::SYSTEMTIME, lpformat : windows_core::PCSTR, lptimestr : windows_core::PSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatA(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lptimestr.unwrap_or(core::mem::zeroed()) as _, cchtime) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetTimeFormatEx<P0, P3>(lplocalename: P0, dwflags: super::DWORD, lptime: Option<*const super::SYSTEMTIME>, lpformat: P3, lptimestr: Option<windows_core::PWSTR>, cchtime: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatEx(lplocalename : windows_core::PCWSTR, dwflags : super::DWORD, lptime : *const super::SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatEx(lplocalename.param().abi(), dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lptimestr.unwrap_or(core::mem::zeroed()) as _, cchtime) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetTimeFormatW<P3>(locale: super::LCID, dwflags: super::DWORD, lptime: Option<*const super::SYSTEMTIME>, lpformat: P3, lptimestr: Option<windows_core::PWSTR>, cchtime: i32) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatW(locale : super::LCID, dwflags : super::DWORD, lptime : *const super::SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatW(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), lptimestr.unwrap_or(core::mem::zeroed()) as _, cchtime) }
}
