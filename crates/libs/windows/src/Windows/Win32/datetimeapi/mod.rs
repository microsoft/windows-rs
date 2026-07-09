#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn GetDateFormatA<P3>(locale: super::winnt::LCID, dwflags: u32, lpdate: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u8]>) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatA(locale : super::winnt::LCID, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCSTR, lpdatestr : windows_core::PSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatA(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetDateFormatEx<P0, P3, P6>(lplocalename: P0, dwflags: u32, lpdate: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u16]>, lpcalendar: P6) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32, lpcalendar : windows_core::PCWSTR) -> i32);
    unsafe { GetDateFormatEx(lplocalename.param().abi(), dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcalendar.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn GetDateFormatW<P3>(locale: super::winnt::LCID, dwflags: u32, lpdate: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lpdatestr: Option<&mut [u16]>) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDateFormatW(locale : super::winnt::LCID, dwflags : u32, lpdate : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCWSTR, lpdatestr : windows_core::PWSTR, cchdate : i32) -> i32);
    unsafe { GetDateFormatW(locale, dwflags, lpdate.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lpdatestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdatestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetDurationFormatEx<P0, P4>(lplocalename: P0, dwflags: u32, lpduration: Option<*const super::minwinbase::SYSTEMTIME>, ullduration: u64, lpformat: P4, lpdurationstr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDurationFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lpduration : *const super::minwinbase::SYSTEMTIME, ullduration : u64, lpformat : windows_core::PCWSTR, lpdurationstr : windows_core::PWSTR, cchduration : i32) -> i32);
    unsafe { GetDurationFormatEx(lplocalename.param().abi(), dwflags, lpduration.unwrap_or(core::mem::zeroed()) as _, ullduration, lpformat.param().abi(), core::mem::transmute(lpdurationstr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdurationstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn GetTimeFormatA<P3>(locale: super::winnt::LCID, dwflags: u32, lptime: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u8]>) -> i32
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatA(locale : super::winnt::LCID, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCSTR, lptimestr : windows_core::PSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatA(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetTimeFormatEx<P0, P3>(lplocalename: P0, dwflags: u32, lptime: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u16]>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatEx(lplocalename : windows_core::PCWSTR, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatEx(lplocalename.param().abi(), dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn GetTimeFormatW<P3>(locale: super::winnt::LCID, dwflags: u32, lptime: Option<*const super::minwinbase::SYSTEMTIME>, lpformat: P3, lptimestr: Option<&mut [u16]>) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTimeFormatW(locale : super::winnt::LCID, dwflags : u32, lptime : *const super::minwinbase::SYSTEMTIME, lpformat : windows_core::PCWSTR, lptimestr : windows_core::PWSTR, cchtime : i32) -> i32);
    unsafe { GetTimeFormatW(locale, dwflags, lptime.unwrap_or(core::mem::zeroed()) as _, lpformat.param().abi(), core::mem::transmute(lptimestr.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lptimestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
