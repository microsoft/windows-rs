#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn AddDelBackupEntryA<P0, P1, P2>(lpcszfilelist: P0, lpcszbackupdir: P1, lpcszbasename: P2, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn AddDelBackupEntryA ( lpcszfilelist : ::windows::core::PCSTR , lpcszbackupdir : ::windows::core::PCSTR , lpcszbasename : ::windows::core::PCSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    AddDelBackupEntryA(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn AddDelBackupEntryW<P0, P1, P2>(lpcszfilelist: P0, lpcszbackupdir: P1, lpcszbasename: P2, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn AddDelBackupEntryW ( lpcszfilelist : ::windows::core::PCWSTR , lpcszbackupdir : ::windows::core::PCWSTR , lpcszbasename : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    AddDelBackupEntryW(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileA<P0, P1, P2, P3, P4>(hwnd: P0, lpszsourcedir: P1, lpszsourcefile: P2, lpszdestdir: P3, lpszdestfile: P4, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn AdvInstallFileA ( hwnd : super::super::Foundation:: HWND , lpszsourcedir : ::windows::core::PCSTR , lpszsourcefile : ::windows::core::PCSTR , lpszdestdir : ::windows::core::PCSTR , lpszdestfile : ::windows::core::PCSTR , dwflags : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    AdvInstallFileA(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), dwflags, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileW<P0, P1, P2, P3, P4>(hwnd: P0, lpszsourcedir: P1, lpszsourcefile: P2, lpszdestdir: P3, lpszdestfile: P4, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn AdvInstallFileW ( hwnd : super::super::Foundation:: HWND , lpszsourcedir : ::windows::core::PCWSTR , lpszsourcefile : ::windows::core::PCWSTR , lpszdestdir : ::windows::core::PCWSTR , lpszdestfile : ::windows::core::PCWSTR , dwflags : u32 , dwreserved : u32 ) -> ::windows::core::HRESULT );
    AdvInstallFileW(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), dwflags, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApphelpCheckShellObject<P0>(objectclsid: *const ::windows::core::GUID, bshimifnecessary: P0, pullflags: *mut u64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "apphelp.dll""system" fn ApphelpCheckShellObject ( objectclsid : *const ::windows::core::GUID , bshimifnecessary : super::super::Foundation:: BOOL , pullflags : *mut u64 ) -> super::super::Foundation:: BOOL );
    ApphelpCheckShellObject(objectclsid, bshimifnecessary.into_param().abi(), pullflags)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelDeviceWakeupRequest<P0>(hdevice: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CancelDeviceWakeupRequest ( hdevice : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CancelDeviceWakeupRequest(hdevice.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelTimerQueueTimer<P0, P1>(timerqueue: P0, timer: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CancelTimerQueueTimer ( timerqueue : super::super::Foundation:: HANDLE , timer : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CancelTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn CloseINFEngine(hinf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "advpack.dll""system" fn CloseINFEngine ( hinf : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    CloseINFEngine(hinf).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-2.dll""system" fn ConvertAuxiliaryCounterToPerformanceCounter ( ullauxiliarycountervalue : u64 , lpperformancecountervalue : *mut u64 , lpconversionerror : *mut u64 ) -> ::windows::core::HRESULT );
    ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue, lpperformancecountervalue, ::core::mem::transmute(lpconversionerror.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-2.dll""system" fn ConvertPerformanceCounterToAuxiliaryCounter ( ullperformancecountervalue : u64 , lpauxiliarycountervalue : *mut u64 , lpconversionerror : *mut u64 ) -> ::windows::core::HRESULT );
    ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue, lpauxiliarycountervalue, ::core::mem::transmute(lpconversionerror.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerA<P0, P1>(lptimerattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P0, lptimername: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateWaitableTimerA ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , bmanualreset : super::super::Foundation:: BOOL , lptimername : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    CreateWaitableTimerA(::core::mem::transmute(lptimerattributes.unwrap_or(::std::ptr::null())), bmanualreset.into_param().abi(), lptimername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerExA<P0>(lptimerattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lptimername: P0, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateWaitableTimerExA ( lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , lptimername : ::windows::core::PCSTR , dwflags : u32 , dwdesiredaccess : u32 ) -> super::super::Foundation:: HANDLE );
    CreateWaitableTimerExA(::core::mem::transmute(lptimerattributes.unwrap_or(::std::ptr::null())), lptimername.into_param().abi(), dwflags, dwdesiredaccess)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIBeginAccess ( pdci : *mut DCISURFACEINFO , x : i32 , y : i32 , dx : i32 , dy : i32 ) -> i32 );
    DCIBeginAccess(pdci, x, y, dx, dy)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICloseProvider<P0>(hdc: P0)
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCICloseProvider ( hdc : super::super::Graphics::Gdi:: HDC ) -> ( ) );
    DCICloseProvider(hdc.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOffscreen<P0>(hdc: P0, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCICreateOffscreen ( hdc : super::super::Graphics::Gdi:: HDC , dwcompression : u32 , dwredmask : u32 , dwgreenmask : u32 , dwbluemask : u32 , dwwidth : u32 , dwheight : u32 , dwdcicaps : u32 , dwbitcount : u32 , lplpsurface : *mut *mut DCIOFFSCREEN ) -> i32 );
    DCICreateOffscreen(hdc.into_param().abi(), dwcompression, dwredmask, dwgreenmask, dwbluemask, dwwidth, dwheight, dwdcicaps, dwbitcount, lplpsurface)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOverlay<P0>(hdc: P0, lpoffscreensurf: *mut ::core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCICreateOverlay ( hdc : super::super::Graphics::Gdi:: HDC , lpoffscreensurf : *mut ::core::ffi::c_void , lplpsurface : *mut *mut DCIOVERLAY ) -> i32 );
    DCICreateOverlay(hdc.into_param().abi(), lpoffscreensurf, lplpsurface)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreatePrimary<P0>(hdc: P0, lplpsurface: *mut *mut DCISURFACEINFO) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCICreatePrimary ( hdc : super::super::Graphics::Gdi:: HDC , lplpsurface : *mut *mut DCISURFACEINFO ) -> i32 );
    DCICreatePrimary(hdc.into_param().abi(), lplpsurface)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIDestroy(pdci: *mut DCISURFACEINFO) {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIDestroy ( pdci : *mut DCISURFACEINFO ) -> ( ) );
    DCIDestroy(pdci)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32 {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIDraw ( pdci : *mut DCIOFFSCREEN ) -> i32 );
    DCIDraw(pdci)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DCIEndAccess(pdci: *mut DCISURFACEINFO) {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIEndAccess ( pdci : *mut DCISURFACEINFO ) -> ( ) );
    DCIEndAccess(pdci)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCIEnum<P0>(hdc: P0, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIEnum ( hdc : super::super::Graphics::Gdi:: HDC , lprdst : *mut super::super::Foundation:: RECT , lprsrc : *mut super::super::Foundation:: RECT , lpfncallback : *mut ::core::ffi::c_void , lpcontext : *mut ::core::ffi::c_void ) -> i32 );
    DCIEnum(hdc.into_param().abi(), lprdst, lprsrc, lpfncallback, lpcontext)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCIOpenProvider ( ) -> super::super::Graphics::Gdi:: HDC );
    DCIOpenProvider()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCISetClipList ( pdci : *mut DCIOFFSCREEN , prd : *mut super::super::Graphics::Gdi:: RGNDATA ) -> i32 );
    DCISetClipList(pdci, prd)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32 {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCISetDestination ( pdci : *mut DCIOFFSCREEN , dst : *mut super::super::Foundation:: RECT , src : *mut super::super::Foundation:: RECT ) -> i32 );
    DCISetDestination(pdci, dst, src)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    ::windows_targets::link ! ( "dciman32.dll""system" fn DCISetSrcDestClip ( pdci : *mut DCIOFFSCREEN , srcrc : *mut super::super::Foundation:: RECT , destrc : *mut super::super::Foundation:: RECT , prd : *mut super::super::Graphics::Gdi:: RGNDATA ) -> i32 );
    DCISetSrcDestClip(pdci, srcrc, destrc, prd)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DelNodeA<P0>(pszfileordirname: P0, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn DelNodeA ( pszfileordirname : ::windows::core::PCSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    DelNodeA(pszfileordirname.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DelNodeRunDLL32W<P0, P1>(hwnd: P0, hinstance: P1, pszparms: ::windows::core::PWSTR, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn DelNodeRunDLL32W ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PWSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    DelNodeRunDLL32W(hwnd.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(pszparms), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn DelNodeW<P0>(pszfileordirname: P0, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn DelNodeW ( pszfileordirname : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    DelNodeW(pszfileordirname.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameA<P0>(hostname: P0, computername: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DnsHostnameToComputerNameA ( hostname : ::windows::core::PCSTR , computername : ::windows::core::PSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    DnsHostnameToComputerNameA(hostname.into_param().abi(), ::core::mem::transmute(computername), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameW<P0>(hostname: P0, computername: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DnsHostnameToComputerNameW ( hostname : ::windows::core::PCWSTR , computername : ::windows::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    DnsHostnameToComputerNameW(hostname.into_param().abi(), ::core::mem::transmute(computername), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DosDateTimeToFileTime ( wfatdate : u16 , wfattime : u16 , lpfiletime : *mut super::super::Foundation:: FILETIME ) -> super::super::Foundation:: BOOL );
    DosDateTimeToFileTime(wfatdate, wfattime, lpfiletime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn EnableProcessOptionalXStateFeatures ( features : u64 ) -> super::super::Foundation:: BOOL );
    EnableProcessOptionalXStateFeatures(features)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabA<P0>(hwnd: P0, pcab: *mut CABINFOA, preserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn ExecuteCabA ( hwnd : super::super::Foundation:: HWND , pcab : *mut CABINFOA , preserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    ExecuteCabA(hwnd.into_param().abi(), pcab, preserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabW<P0>(hwnd: P0, pcab: *mut CABINFOW, preserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn ExecuteCabW ( hwnd : super::super::Foundation:: HWND , pcab : *mut CABINFOW , preserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    ExecuteCabW(hwnd.into_param().abi(), pcab, preserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ExtractFilesA<P0, P1, P2>(pszcabname: P0, pszexpanddir: P1, dwflags: u32, pszfilelist: P2, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn ExtractFilesA ( pszcabname : ::windows::core::PCSTR , pszexpanddir : ::windows::core::PCSTR , dwflags : u32 , pszfilelist : ::windows::core::PCSTR , lpreserved : *mut ::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    ExtractFilesA(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), dwflags, pszfilelist.into_param().abi(), lpreserved, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn ExtractFilesW<P0, P1, P2>(pszcabname: P0, pszexpanddir: P1, dwflags: u32, pszfilelist: P2, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn ExtractFilesW ( pszcabname : ::windows::core::PCWSTR , pszexpanddir : ::windows::core::PCWSTR , dwflags : u32 , pszfilelist : ::windows::core::PCWSTR , lpreserved : *mut ::core::ffi::c_void , dwreserved : u32 ) -> ::windows::core::HRESULT );
    ExtractFilesW(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), dwflags, pszfilelist.into_param().abi(), lpreserved, dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn FileSaveMarkNotExistA<P0, P1, P2>(lpfilelist: P0, lpdir: P1, lpbasename: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn FileSaveMarkNotExistA ( lpfilelist : ::windows::core::PCSTR , lpdir : ::windows::core::PCSTR , lpbasename : ::windows::core::PCSTR ) -> ::windows::core::HRESULT );
    FileSaveMarkNotExistA(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn FileSaveMarkNotExistW<P0, P1, P2>(lpfilelist: P0, lpdir: P1, lpbasename: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn FileSaveMarkNotExistW ( lpfilelist : ::windows::core::PCWSTR , lpdir : ::windows::core::PCWSTR , lpbasename : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    FileSaveMarkNotExistW(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFA<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitle: P1, pszinf: P2, pszsection: P3, pszbackupdir: P4, pszbasebackupfile: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn FileSaveRestoreOnINFA ( hwnd : super::super::Foundation:: HWND , psztitle : ::windows::core::PCSTR , pszinf : ::windows::core::PCSTR , pszsection : ::windows::core::PCSTR , pszbackupdir : ::windows::core::PCSTR , pszbasebackupfile : ::windows::core::PCSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    FileSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFW<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitle: P1, pszinf: P2, pszsection: P3, pszbackupdir: P4, pszbasebackupfile: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn FileSaveRestoreOnINFW ( hwnd : super::super::Foundation:: HWND , psztitle : ::windows::core::PCWSTR , pszinf : ::windows::core::PCWSTR , pszsection : ::windows::core::PCWSTR , pszbackupdir : ::windows::core::PCWSTR , pszbasebackupfile : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    FileSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreW<P0, P1, P2, P3>(hdlg: P0, lpfilelist: P1, lpdir: P2, lpbasename: P3, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn FileSaveRestoreW ( hdlg : super::super::Foundation:: HWND , lpfilelist : ::windows::core::PCWSTR , lpdir : ::windows::core::PCWSTR , lpbasename : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    FileSaveRestoreW(hdlg.into_param().abi(), lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FileTimeToDosDateTime ( lpfiletime : *const super::super::Foundation:: FILETIME , lpfatdate : *mut u16 , lpfattime : *mut u16 ) -> super::super::Foundation:: BOOL );
    FileTimeToDosDateTime(lpfiletime, lpfatdate, lpfattime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GdiEntry13() -> u32 {
    ::windows_targets::link ! ( "api-ms-win-dx-d3dkmt-l1-1-0.dll""system" fn GdiEntry13 ( ) -> u32 );
    GdiEntry13()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameA(lpbuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetComputerNameA ( lpbuffer : ::windows::core::PSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetComputerNameA(::core::mem::transmute(lpbuffer), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameW(lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetComputerNameW ( lpbuffer : ::windows::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetComputerNameW(::core::mem::transmute(lpbuffer), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetCurrentHwProfileA ( lphwprofileinfo : *mut HW_PROFILE_INFOA ) -> super::super::Foundation:: BOOL );
    GetCurrentHwProfileA(lphwprofileinfo)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetCurrentHwProfileW ( lphwprofileinfo : *mut HW_PROFILE_INFOW ) -> super::super::Foundation:: BOOL );
    GetCurrentHwProfileW(lphwprofileinfo)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetDCRegionData<P0>(hdc: P0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn GetDCRegionData ( hdc : super::super::Graphics::Gdi:: HDC , size : u32 , prd : *mut super::super::Graphics::Gdi:: RGNDATA ) -> u32 );
    GetDCRegionData(hdc.into_param().abi(), size, prd)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE {
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-0.dll""system" fn GetFeatureEnabledState ( featureid : u32 , changetime : FEATURE_CHANGE_TIME ) -> FEATURE_ENABLED_STATE );
    GetFeatureEnabledState(featureid, changetime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32 {
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-1.dll""system" fn GetFeatureVariant ( featureid : u32 , changetime : FEATURE_CHANGE_TIME , payloadid : *mut u32 , hasnotification : *mut super::super::Foundation:: BOOL ) -> u32 );
    GetFeatureVariant(featureid, changetime, payloadid, hasnotification)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nsize: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFirmwareEnvironmentVariableA ( lpname : ::windows::core::PCSTR , lpguid : ::windows::core::PCSTR , pbuffer : *mut ::core::ffi::c_void , nsize : u32 ) -> u32 );
    GetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nsize: u32, pdwattribubutes: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFirmwareEnvironmentVariableExA ( lpname : ::windows::core::PCSTR , lpguid : ::windows::core::PCSTR , pbuffer : *mut ::core::ffi::c_void , nsize : u32 , pdwattribubutes : *mut u32 ) -> u32 );
    GetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), nsize, ::core::mem::transmute(pdwattribubutes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nsize: u32, pdwattribubutes: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFirmwareEnvironmentVariableExW ( lpname : ::windows::core::PCWSTR , lpguid : ::windows::core::PCWSTR , pbuffer : *mut ::core::ffi::c_void , nsize : u32 , pdwattribubutes : *mut u32 ) -> u32 );
    GetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), nsize, ::core::mem::transmute(pdwattribubutes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, nsize: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetFirmwareEnvironmentVariableW ( lpname : ::windows::core::PCWSTR , lpguid : ::windows::core::PCWSTR , pbuffer : *mut ::core::ffi::c_void , nsize : u32 ) -> u32 );
    GetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileIntA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, ndefault: i32, lpfilename: P2) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileIntA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , ndefault : i32 , lpfilename : ::windows::core::PCSTR ) -> u32 );
    GetPrivateProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ndefault, lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileIntW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, ndefault: i32, lpfilename: P2) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileIntW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , ndefault : i32 , lpfilename : ::windows::core::PCWSTR ) -> i32 );
    GetPrivateProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ndefault, lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionA<P0, P1>(lpappname: P0, lpreturnedstring: ::core::option::Option<&mut [u8]>, lpfilename: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileSectionA ( lpappname : ::windows::core::PCSTR , lpreturnedstring : ::windows::core::PSTR , nsize : u32 , lpfilename : ::windows::core::PCSTR ) -> u32 );
    GetPrivateProfileSectionA(lpappname.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesA<P0>(lpszreturnbuffer: ::core::option::Option<&mut [u8]>, lpfilename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileSectionNamesA ( lpszreturnbuffer : ::windows::core::PSTR , nsize : u32 , lpfilename : ::windows::core::PCSTR ) -> u32 );
    GetPrivateProfileSectionNamesA(::core::mem::transmute(lpszreturnbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszreturnbuffer.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesW<P0>(lpszreturnbuffer: ::core::option::Option<&mut [u16]>, lpfilename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileSectionNamesW ( lpszreturnbuffer : ::windows::core::PWSTR , nsize : u32 , lpfilename : ::windows::core::PCWSTR ) -> u32 );
    GetPrivateProfileSectionNamesW(::core::mem::transmute(lpszreturnbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszreturnbuffer.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileSectionW<P0, P1>(lpappname: P0, lpreturnedstring: ::core::option::Option<&mut [u16]>, lpfilename: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileSectionW ( lpappname : ::windows::core::PCWSTR , lpreturnedstring : ::windows::core::PWSTR , nsize : u32 , lpfilename : ::windows::core::PCWSTR ) -> u32 );
    GetPrivateProfileSectionW(lpappname.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileStringA<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: ::core::option::Option<&mut [u8]>, lpfilename: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileStringA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , lpdefault : ::windows::core::PCSTR , lpreturnedstring : ::windows::core::PSTR , nsize : u32 , lpfilename : ::windows::core::PCSTR ) -> u32 );
    GetPrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetPrivateProfileStringW<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: ::core::option::Option<&mut [u16]>, lpfilename: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileStringW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , lpdefault : ::windows::core::PCWSTR , lpreturnedstring : ::windows::core::PWSTR , nsize : u32 , lpfilename : ::windows::core::PCWSTR ) -> u32 );
    GetPrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructA<P0, P1, P2>(lpszsection: P0, lpszkey: P1, lpstruct: ::core::option::Option<*mut ::core::ffi::c_void>, usizestruct: u32, szfile: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileStructA ( lpszsection : ::windows::core::PCSTR , lpszkey : ::windows::core::PCSTR , lpstruct : *mut ::core::ffi::c_void , usizestruct : u32 , szfile : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    GetPrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct.unwrap_or(::std::ptr::null_mut())), usizestruct, szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructW<P0, P1, P2>(lpszsection: P0, lpszkey: P1, lpstruct: ::core::option::Option<*mut ::core::ffi::c_void>, usizestruct: u32, szfile: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetPrivateProfileStructW ( lpszsection : ::windows::core::PCWSTR , lpszkey : ::windows::core::PCWSTR , lpstruct : *mut ::core::ffi::c_void , usizestruct : u32 , szfile : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    GetPrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct.unwrap_or(::std::ptr::null_mut())), usizestruct, szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileIntA<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: i32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileIntA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , ndefault : i32 ) -> u32 );
    GetProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ndefault)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileIntW<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: i32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileIntW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , ndefault : i32 ) -> u32 );
    GetProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ndefault)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileSectionA<P0>(lpappname: P0, lpreturnedstring: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileSectionA ( lpappname : ::windows::core::PCSTR , lpreturnedstring : ::windows::core::PSTR , nsize : u32 ) -> u32 );
    GetProfileSectionA(lpappname.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileSectionW<P0>(lpappname: P0, lpreturnedstring: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileSectionW ( lpappname : ::windows::core::PCWSTR , lpreturnedstring : ::windows::core::PWSTR , nsize : u32 ) -> u32 );
    GetProfileSectionW(lpappname.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileStringA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , lpdefault : ::windows::core::PCSTR , lpreturnedstring : ::windows::core::PSTR , nsize : u32 ) -> u32 );
    GetProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GetProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProfileStringW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , lpdefault : ::windows::core::PCWSTR , lpreturnedstring : ::windows::core::PWSTR , nsize : u32 ) -> u32 );
    GetProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::core::mem::transmute(lpreturnedstring.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpreturnedstring.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemRegistryQuota(pdwquotaallowed: ::core::option::Option<*mut u32>, pdwquotaused: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetSystemRegistryQuota ( pdwquotaallowed : *mut u32 , pdwquotaused : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetSystemRegistryQuota(::core::mem::transmute(pdwquotaallowed.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwquotaused.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetThreadEnabledXStateFeatures() -> u64 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetThreadEnabledXStateFeatures ( ) -> u64 );
    GetThreadEnabledXStateFeatures()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameA(lpbuffer: ::windows::core::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetUserNameA ( lpbuffer : ::windows::core::PSTR , pcbbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetUserNameA(::core::mem::transmute(lpbuffer), pcbbuffer)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameW(lpbuffer: ::windows::core::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetUserNameW ( lpbuffer : ::windows::core::PWSTR , pcbbuffer : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetUserNameW(::core::mem::transmute(lpbuffer), pcbbuffer)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileA<P0, P1>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn GetVersionFromFileA ( lpszfilename : ::windows::core::PCSTR , pdwmsver : *mut u32 , pdwlsver : *mut u32 , bversion : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetVersionFromFileA(lpszfilename.into_param().abi(), pdwmsver, pdwlsver, bversion.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExA<P0, P1>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn GetVersionFromFileExA ( lpszfilename : ::windows::core::PCSTR , pdwmsver : *mut u32 , pdwlsver : *mut u32 , bversion : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetVersionFromFileExA(lpszfilename.into_param().abi(), pdwmsver, pdwlsver, bversion.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExW<P0, P1>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn GetVersionFromFileExW ( lpszfilename : ::windows::core::PCWSTR , pdwmsver : *mut u32 , pdwlsver : *mut u32 , bversion : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetVersionFromFileExW(lpszfilename.into_param().abi(), pdwmsver, pdwlsver, bversion.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileW<P0, P1>(lpszfilename: P0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn GetVersionFromFileW ( lpszfilename : ::windows::core::PCWSTR , pdwmsver : *mut u32 , pdwlsver : *mut u32 , bversion : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetVersionFromFileW(lpszfilename.into_param().abi(), pdwmsver, pdwlsver, bversion.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetWindowRegionData<P0>(hwnd: P0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn GetWindowRegionData ( hwnd : super::super::Foundation:: HWND , size : u32 , prd : *mut super::super::Graphics::Gdi:: RGNDATA ) -> u32 );
    GetWindowRegionData(hwnd.into_param().abi(), size, prd)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn GlobalCompact(dwminfree: u32) -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalCompact ( dwminfree : u32 ) -> usize );
    GlobalCompact(dwminfree)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalFix<P0>(hmem: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalFix ( hmem : super::super::Foundation:: HGLOBAL ) -> ( ) );
    GlobalFix(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnWire<P0>(hmem: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalUnWire ( hmem : super::super::Foundation:: HGLOBAL ) -> super::super::Foundation:: BOOL );
    GlobalUnWire(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnfix<P0>(hmem: P0)
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalUnfix ( hmem : super::super::Foundation:: HGLOBAL ) -> ( ) );
    GlobalUnfix(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalWire<P0>(hmem: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalWire ( hmem : super::super::Foundation:: HGLOBAL ) -> *mut ::core::ffi::c_void );
    GlobalWire(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEA<P0>(param0: P0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn IMPGetIMEA ( param0 : super::super::Foundation:: HWND , param1 : *mut IMEPROA ) -> super::super::Foundation:: BOOL );
    IMPGetIMEA(param0.into_param().abi(), param1)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEW<P0>(param0: P0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn IMPGetIMEW ( param0 : super::super::Foundation:: HWND , param1 : *mut IMEPROW ) -> super::super::Foundation:: BOOL );
    IMPGetIMEW(param0.into_param().abi(), param1)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "user32.dll""system" fn IMPQueryIMEA ( param0 : *mut IMEPROA ) -> super::super::Foundation:: BOOL );
    IMPQueryIMEA(param0)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "user32.dll""system" fn IMPQueryIMEW ( param0 : *mut IMEPROW ) -> super::super::Foundation:: BOOL );
    IMPQueryIMEW(param0)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEA<P0>(param0: P0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn IMPSetIMEA ( param0 : super::super::Foundation:: HWND , param1 : *mut IMEPROA ) -> super::super::Foundation:: BOOL );
    IMPSetIMEA(param0.into_param().abi(), param1)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEW<P0>(param0: P0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn IMPSetIMEW ( param0 : super::super::Foundation:: HWND , param1 : *mut IMEPROW ) -> super::super::Foundation:: BOOL );
    IMPSetIMEW(param0.into_param().abi(), param1)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsApiSetImplemented<P0>(contract: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-apiquery-l2-1-0.dll""system" fn IsApiSetImplemented ( contract : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    IsApiSetImplemented(contract.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeReadPtr(lp: ::core::option::Option<*const ::core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadHugeReadPtr ( lp : *const ::core::ffi::c_void , ucb : usize ) -> super::super::Foundation:: BOOL );
    IsBadHugeReadPtr(::core::mem::transmute(lp.unwrap_or(::std::ptr::null())), ucb)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeWritePtr(lp: ::core::option::Option<*const ::core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadHugeWritePtr ( lp : *const ::core::ffi::c_void , ucb : usize ) -> super::super::Foundation:: BOOL );
    IsBadHugeWritePtr(::core::mem::transmute(lp.unwrap_or(::std::ptr::null())), ucb)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advpack.dll""system" fn IsNTAdmin ( dwreserved : u32 , lpdwreserved : *mut u32 ) -> super::super::Foundation:: BOOL );
    IsNTAdmin(dwreserved, lpdwreserved)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsNativeVhdBoot ( nativevhdboot : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    IsNativeVhdBoot(nativevhdboot)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTokenUntrusted<P0>(tokenhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "advapi32.dll""system" fn IsTokenUntrusted ( tokenhandle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    IsTokenUntrusted(tokenhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionExW<P0, P1, P2>(hwnd: P0, hinstance: P1, pszparms: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn LaunchINFSectionExW ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PCWSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    LaunchINFSectionExW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionW<P0, P1>(hwndowner: P0, hinstance: P1, pszparams: ::windows::core::PWSTR, nshow: i32) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn LaunchINFSectionW ( hwndowner : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparams : ::windows::core::PWSTR , nshow : i32 ) -> i32 );
    LaunchINFSectionW(hwndowner.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(pszparams), nshow)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn LocalCompact(uminfree: u32) -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalCompact ( uminfree : u32 ) -> usize );
    LocalCompact(uminfree)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalShrink<P0>(hmem: P0, cbnewsize: u32) -> usize
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalShrink ( hmem : super::super::Foundation:: HLOCAL , cbnewsize : u32 ) -> usize );
    LocalShrink(hmem.into_param().abi(), cbnewsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn MulDiv ( nnumber : i32 , nnumerator : i32 , ndenominator : i32 ) -> i32 );
    MulDiv(nnumber, nnumerator, ndenominator)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advpack.dll""system" fn NeedReboot ( dwrebootcheck : u32 ) -> super::super::Foundation:: BOOL );
    NeedReboot(dwrebootcheck)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn NeedRebootInit() -> u32 {
    ::windows_targets::link ! ( "advpack.dll""system" fn NeedRebootInit ( ) -> u32 );
    NeedRebootInit()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtClose<P0>(handle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtClose ( handle : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: NTSTATUS );
    NtClose(handle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtDeviceIoControlFile<P0, P1>(filehandle: P0, event: P1, apcroutine: PIO_APC_ROUTINE, apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtDeviceIoControlFile ( filehandle : super::super::Foundation:: HANDLE , event : super::super::Foundation:: HANDLE , apcroutine : PIO_APC_ROUTINE , apccontext : *mut ::core::ffi::c_void , iostatusblock : *mut IO_STATUS_BLOCK , iocontrolcode : u32 , inputbuffer : *mut ::core::ffi::c_void , inputbufferlength : u32 , outputbuffer : *mut ::core::ffi::c_void , outputbufferlength : u32 ) -> super::super::Foundation:: NTSTATUS );
    NtDeviceIoControlFile(filehandle.into_param().abi(), event.into_param().abi(), apcroutine, apccontext, iostatusblock, iocontrolcode, inputbuffer, inputbufferlength, outputbuffer, outputbufferlength).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys<P0, P1, P2, P3>(masterkeyhandle: P0, subordinateobjects: ::core::option::Option<&[OBJECT_ATTRIBUTES]>, event: P1, apcroutine: PIO_APC_ROUTINE, apccontext: ::core::option::Option<*const ::core::ffi::c_void>, iostatusblock: *mut IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtNotifyChangeMultipleKeys ( masterkeyhandle : super::super::Foundation:: HANDLE , count : u32 , subordinateobjects : *const OBJECT_ATTRIBUTES , event : super::super::Foundation:: HANDLE , apcroutine : PIO_APC_ROUTINE , apccontext : *const ::core::ffi::c_void , iostatusblock : *mut IO_STATUS_BLOCK , completionfilter : u32 , watchtree : super::super::Foundation:: BOOLEAN , buffer : *mut ::core::ffi::c_void , buffersize : u32 , asynchronous : super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: NTSTATUS );
    NtNotifyChangeMultipleKeys(
        masterkeyhandle.into_param().abi(),
        subordinateobjects.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(subordinateobjects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        event.into_param().abi(),
        apcroutine,
        ::core::mem::transmute(apccontext.unwrap_or(::std::ptr::null())),
        iostatusblock,
        completionfilter,
        watchtree.into_param().abi(),
        ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())),
        buffersize,
        asynchronous.into_param().abi(),
    )
    .ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtOpenFile ( filehandle : *mut super::super::Foundation:: HANDLE , desiredaccess : u32 , objectattributes : *mut OBJECT_ATTRIBUTES , iostatusblock : *mut IO_STATUS_BLOCK , shareaccess : u32 , openoptions : u32 ) -> super::super::Foundation:: NTSTATUS );
    NtOpenFile(filehandle, desiredaccess, objectattributes, iostatusblock, shareaccess, openoptions).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryMultipleValueKey<P0>(keyhandle: P0, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtQueryMultipleValueKey ( keyhandle : super::super::Foundation:: HANDLE , valueentries : *mut KEY_VALUE_ENTRY , entrycount : u32 , valuebuffer : *mut ::core::ffi::c_void , bufferlength : *mut u32 , requiredbufferlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQueryMultipleValueKey(keyhandle.into_param().abi(), ::core::mem::transmute(valueentries.as_ptr()), valueentries.len() as _, valuebuffer, bufferlength, ::core::mem::transmute(requiredbufferlength.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryObject<P0>(handle: P0, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: ::core::option::Option<*mut ::core::ffi::c_void>, objectinformationlength: u32, returnlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtQueryObject ( handle : super::super::Foundation:: HANDLE , objectinformationclass : OBJECT_INFORMATION_CLASS , objectinformation : *mut ::core::ffi::c_void , objectinformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQueryObject(handle.into_param().abi(), objectinformationclass, ::core::mem::transmute(objectinformation.unwrap_or(::std::ptr::null_mut())), objectinformationlength, ::core::mem::transmute(returnlength.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtQuerySystemInformation ( systeminformationclass : SYSTEM_INFORMATION_CLASS , systeminformation : *mut ::core::ffi::c_void , systeminformationlength : u32 , returnlength : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQuerySystemInformation(systeminformationclass, systeminformation, systeminformationlength, returnlength).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemTime(systemtime: *mut i64) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtQuerySystemTime ( systemtime : *mut i64 ) -> super::super::Foundation:: NTSTATUS );
    NtQuerySystemTime(systemtime).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtQueryTimerResolution ( maximumtime : *mut u32 , minimumtime : *mut u32 , currenttime : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    NtQueryTimerResolution(maximumtime, minimumtime, currenttime).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtRenameKey<P0>(keyhandle: P0, newname: *const super::super::Foundation::UNICODE_STRING) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtRenameKey ( keyhandle : super::super::Foundation:: HANDLE , newname : *const super::super::Foundation:: UNICODE_STRING ) -> super::super::Foundation:: NTSTATUS );
    NtRenameKey(keyhandle.into_param().abi(), newname).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtSetInformationKey ( keyhandle : super::super::Foundation:: HANDLE , keysetinformationclass : KEY_SET_INFORMATION_CLASS , keysetinformation : *const ::core::ffi::c_void , keysetinformationlength : u32 ) -> super::super::Foundation:: NTSTATUS );
    NtSetInformationKey(keyhandle.into_param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtWaitForSingleObject<P0, P1>(handle: P0, alertable: P1, timeout: *mut i64) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn NtWaitForSingleObject ( handle : super::super::Foundation:: HANDLE , alertable : super::super::Foundation:: BOOLEAN , timeout : *mut i64 ) -> super::super::Foundation:: NTSTATUS );
    NtWaitForSingleObject(handle.into_param().abi(), alertable.into_param().abi(), timeout).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn OpenINFEngineA<P0, P1>(pszinffilename: P0, pszinstallsection: P1, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn OpenINFEngineA ( pszinffilename : ::windows::core::PCSTR , pszinstallsection : ::windows::core::PCSTR , dwflags : u32 , phinf : *mut *mut ::core::ffi::c_void , pvreserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    OpenINFEngineA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), dwflags, phinf, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn OpenINFEngineW<P0, P1>(pszinffilename: P0, pszinstallsection: P1, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn OpenINFEngineW ( pszinffilename : ::windows::core::PCWSTR , pszinstallsection : ::windows::core::PCWSTR , dwflags : u32 , phinf : *mut *mut ::core::ffi::c_void , pvreserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    OpenINFEngineW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), dwflags, phinf, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenMutexA<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenMutexA ( dwdesiredaccess : u32 , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    OpenMutexA(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenSemaphoreA<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenSemaphoreA ( dwdesiredaccess : u32 , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    OpenSemaphoreA(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWaitableTimerA<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lptimername: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenWaitableTimerA ( dwdesiredaccess : u32 , binherithandle : super::super::Foundation:: BOOL , lptimername : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    OpenWaitableTimerA(dwdesiredaccess, binherithandle.into_param().abi(), lptimername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency() -> ::windows::core::Result<u64> {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-2.dll""system" fn QueryAuxiliaryCounterFrequency ( lpauxiliarycounterfrequency : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    QueryAuxiliaryCounterFrequency(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: ::core::option::Option<*mut u64>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryIdleProcessorCycleTime ( bufferlength : *mut u32 , processoridlecycletime : *mut u64 ) -> super::super::Foundation:: BOOL );
    QueryIdleProcessorCycleTime(bufferlength, ::core::mem::transmute(processoridlecycletime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: ::core::option::Option<*mut u64>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryIdleProcessorCycleTimeEx ( group : u16 , bufferlength : *mut u32 , processoridlecycletime : *mut u64 ) -> super::super::Foundation:: BOOL );
    QueryIdleProcessorCycleTimeEx(group, bufferlength, ::core::mem::transmute(processoridlecycletime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryInterruptTime() -> u64 {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-1.dll""system" fn QueryInterruptTime ( lpinterrupttime : *mut u64 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u64>();
    QueryInterruptTime(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryInterruptTimePrecise() -> u64 {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-1.dll""system" fn QueryInterruptTimePrecise ( lpinterrupttimeprecise : *mut u64 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u64>();
    QueryInterruptTimePrecise(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProcessCycleTime<P0>(processhandle: P0, cycletime: *mut u64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryProcessCycleTime ( processhandle : super::super::Foundation:: HANDLE , cycletime : *mut u64 ) -> super::super::Foundation:: BOOL );
    QueryProcessCycleTime(processhandle.into_param().abi(), cycletime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadCycleTime<P0>(threadhandle: P0, cycletime: *mut u64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryThreadCycleTime ( threadhandle : super::super::Foundation:: HANDLE , cycletime : *mut u64 ) -> super::super::Foundation:: BOOL );
    QueryThreadCycleTime(threadhandle.into_param().abi(), cycletime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryUnbiasedInterruptTime ( unbiasedtime : *mut u64 ) -> super::super::Foundation:: BOOL );
    QueryUnbiasedInterruptTime(unbiasedtime)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise() -> u64 {
    ::windows_targets::link ! ( "api-ms-win-core-realtime-l1-1-1.dll""system" fn QueryUnbiasedInterruptTimePrecise ( lpunbiasedinterrupttimeprecise : *mut u64 ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<u64>();
    QueryUnbiasedInterruptTimePrecise(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    ::windows_targets::link ! ( "api-ms-win-core-backgroundtask-l1-1-0.dll""system" fn RaiseCustomSystemEventTrigger ( customsystemeventtriggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG ) -> u32 );
    RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallA<P0, P1, P2>(hwnd: P0, pszinf: P1, pszsec: P2, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RebootCheckOnInstallA ( hwnd : super::super::Foundation:: HWND , pszinf : ::windows::core::PCSTR , pszsec : ::windows::core::PCSTR , dwreserved : u32 ) -> ::windows::core::HRESULT );
    RebootCheckOnInstallA(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallW<P0, P1, P2>(hwnd: P0, pszinf: P1, pszsec: P2, dwreserved: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RebootCheckOnInstallW ( hwnd : super::super::Foundation:: HWND , pszinf : ::windows::core::PCWSTR , pszsec : ::windows::core::PCWSTR , dwreserved : u32 ) -> ::windows::core::HRESULT );
    RebootCheckOnInstallW(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), dwreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR) {
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-0.dll""system" fn RecordFeatureError ( featureid : u32 , error : *const FEATURE_ERROR ) -> ( ) );
    RecordFeatureError(featureid, error)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RecordFeatureUsage<P0>(featureid: u32, kind: u32, addend: u32, originname: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-0.dll""system" fn RecordFeatureUsage ( featureid : u32 , kind : u32 , addend : u32 , originname : ::windows::core::PCSTR ) -> ( ) );
    RecordFeatureUsage(featureid, kind, addend, originname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallA<P0, P1>(hmod: P0, pszsection: P1, psttable: *const STRTABLEA) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegInstallA ( hmod : super::super::Foundation:: HMODULE , pszsection : ::windows::core::PCSTR , psttable : *const STRTABLEA ) -> ::windows::core::HRESULT );
    RegInstallA(hmod.into_param().abi(), pszsection.into_param().abi(), psttable).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallW<P0, P1>(hmod: P0, pszsection: P1, psttable: *const STRTABLEW) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegInstallW ( hmod : super::super::Foundation:: HMODULE , pszsection : ::windows::core::PCWSTR , psttable : *const STRTABLEW ) -> ::windows::core::HRESULT );
    RegInstallW(hmod.into_param().abi(), pszsection.into_param().abi(), psttable).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllA<P0, P1, P2>(hwnd: P0, psztitlestring: P1, hkbckupkey: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::Registry::HKEY>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegRestoreAllA ( hwnd : super::super::Foundation:: HWND , psztitlestring : ::windows::core::PCSTR , hkbckupkey : super::Registry:: HKEY ) -> ::windows::core::HRESULT );
    RegRestoreAllA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllW<P0, P1, P2>(hwnd: P0, psztitlestring: P1, hkbckupkey: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::Registry::HKEY>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegRestoreAllW ( hwnd : super::super::Foundation:: HWND , psztitlestring : ::windows::core::PCWSTR , hkbckupkey : super::Registry:: HKEY ) -> ::windows::core::HRESULT );
    RegRestoreAllW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreA<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitlestring: P1, hkbckupkey: P2, pcszrootkey: P3, pcszsubkey: P4, pcszvaluename: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::Registry::HKEY>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegSaveRestoreA ( hwnd : super::super::Foundation:: HWND , psztitlestring : ::windows::core::PCSTR , hkbckupkey : super::Registry:: HKEY , pcszrootkey : ::windows::core::PCSTR , pcszsubkey : ::windows::core::PCSTR , pcszvaluename : ::windows::core::PCSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    RegSaveRestoreA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFA<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitle: P1, pszinf: P2, pszsection: P3, hhklmbackkey: P4, hhkcubackkey: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<super::Registry::HKEY>,
    P5: ::windows::core::IntoParam<super::Registry::HKEY>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegSaveRestoreOnINFA ( hwnd : super::super::Foundation:: HWND , psztitle : ::windows::core::PCSTR , pszinf : ::windows::core::PCSTR , pszsection : ::windows::core::PCSTR , hhklmbackkey : super::Registry:: HKEY , hhkcubackkey : super::Registry:: HKEY , dwflags : u32 ) -> ::windows::core::HRESULT );
    RegSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFW<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitle: P1, pszinf: P2, pszsection: P3, hhklmbackkey: P4, hhkcubackkey: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<super::Registry::HKEY>,
    P5: ::windows::core::IntoParam<super::Registry::HKEY>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegSaveRestoreOnINFW ( hwnd : super::super::Foundation:: HWND , psztitle : ::windows::core::PCWSTR , pszinf : ::windows::core::PCWSTR , pszsection : ::windows::core::PCWSTR , hhklmbackkey : super::Registry:: HKEY , hhkcubackkey : super::Registry:: HKEY , dwflags : u32 ) -> ::windows::core::HRESULT );
    RegSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreW<P0, P1, P2, P3, P4, P5>(hwnd: P0, psztitlestring: P1, hkbckupkey: P2, pcszrootkey: P3, pcszsubkey: P4, pcszvaluename: P5, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::Registry::HKEY>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RegSaveRestoreW ( hwnd : super::super::Foundation:: HWND , psztitlestring : ::windows::core::PCWSTR , hkbckupkey : super::Registry:: HKEY , pcszrootkey : ::windows::core::PCWSTR , pcszsubkey : ::windows::core::PCWSTR , pcszvaluename : ::windows::core::PCWSTR , dwflags : u32 ) -> ::windows::core::HRESULT );
    RegSaveRestoreW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplacePartitionUnit<P0, P1>(targetpartition: P0, sparepartition: P1, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReplacePartitionUnit ( targetpartition : ::windows::core::PCWSTR , sparepartition : ::windows::core::PCWSTR , flags : u32 ) -> super::super::Foundation:: BOOL );
    ReplacePartitionUnit(targetpartition.into_param().abi(), sparepartition.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RequestDeviceWakeup<P0>(hdevice: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RequestDeviceWakeup ( hdevice : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    RequestDeviceWakeup(hdevice.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlAnsiStringToUnicodeString<P0>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlAnsiStringToUnicodeString ( destinationstring : *mut super::super::Foundation:: UNICODE_STRING , sourcestring : *mut super::Kernel:: STRING , allocatedestinationstring : super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: NTSTATUS );
    RtlAnsiStringToUnicodeString(destinationstring, sourcestring, allocatedestinationstring.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlCharToInteger ( string : *mut i8 , base : u32 , value : *mut u32 ) -> super::super::Foundation:: NTSTATUS );
    RtlCharToInteger(string, base, value).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlFreeAnsiString ( ansistring : *mut super::Kernel:: STRING ) -> ( ) );
    RtlFreeAnsiString(ansistring)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlFreeOemString ( oemstring : *mut super::Kernel:: STRING ) -> ( ) );
    RtlFreeOemString(oemstring)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlFreeUnicodeString ( unicodestring : *mut super::super::Foundation:: UNICODE_STRING ) -> ( ) );
    RtlFreeUnicodeString(unicodestring)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlGetReturnAddressHijackTarget() -> usize {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlGetReturnAddressHijackTarget ( ) -> usize );
    RtlGetReturnAddressHijackTarget()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlInitAnsiString ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut i8 ) -> ( ) );
    RtlInitAnsiString(destinationstring, sourcestring)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlInitAnsiStringEx ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut i8 ) -> super::super::Foundation:: NTSTATUS );
    RtlInitAnsiStringEx(destinationstring, sourcestring).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlInitString ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut i8 ) -> ( ) );
    RtlInitString(destinationstring, sourcestring)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlInitStringEx ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut i8 ) -> super::super::Foundation:: NTSTATUS );
    RtlInitStringEx(destinationstring, sourcestring).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInitUnicodeString<P0>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlInitUnicodeString ( destinationstring : *mut super::super::Foundation:: UNICODE_STRING , sourcestring : ::windows::core::PCWSTR ) -> ( ) );
    RtlInitUnicodeString(destinationstring, sourcestring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlIsNameLegalDOS8Dot3 ( name : *mut super::super::Foundation:: UNICODE_STRING , oemname : *mut super::Kernel:: STRING , namecontainsspaces : *mut super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: BOOLEAN );
    RtlIsNameLegalDOS8Dot3(name, oemname, namecontainsspaces)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlLocalTimeToSystemTime ( localtime : *mut i64 , systemtime : *mut i64 ) -> super::super::Foundation:: NTSTATUS );
    RtlLocalTimeToSystemTime(localtime, systemtime).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlRaiseCustomSystemEventTrigger ( triggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG ) -> u32 );
    RtlRaiseCustomSystemEventTrigger(triggerconfig)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlTimeToSecondsSince1970 ( time : *mut i64 , elapsedseconds : *mut u32 ) -> super::super::Foundation:: BOOLEAN );
    RtlTimeToSecondsSince1970(time, elapsedseconds)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToAnsiString<P0>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlUnicodeStringToAnsiString ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut super::super::Foundation:: UNICODE_STRING , allocatedestinationstring : super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: NTSTATUS );
    RtlUnicodeStringToAnsiString(destinationstring, sourcestring, allocatedestinationstring.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToOemString<P0>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlUnicodeStringToOemString ( destinationstring : *mut super::Kernel:: STRING , sourcestring : *mut super::super::Foundation:: UNICODE_STRING , allocatedestinationstring : super::super::Foundation:: BOOLEAN ) -> super::super::Foundation:: NTSTATUS );
    RtlUnicodeStringToOemString(destinationstring, sourcestring, allocatedestinationstring.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteSize<P0>(bytesinmultibytestring: *mut u32, unicodestring: P0, bytesinunicodestring: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlUnicodeToMultiByteSize ( bytesinmultibytestring : *mut u32 , unicodestring : ::windows::core::PCWSTR , bytesinunicodestring : u32 ) -> super::super::Foundation:: NTSTATUS );
    RtlUnicodeToMultiByteSize(bytesinmultibytestring, unicodestring.into_param().abi(), bytesinunicodestring).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn RtlUniform(seed: *mut u32) -> u32 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlUniform ( seed : *mut u32 ) -> u32 );
    RtlUniform(seed)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandA<P0, P1, P2, P3, P4>(hwnd: P0, szcmdname: P1, szinfsection: P2, szdir: P3, lpsztitle: P4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RunSetupCommandA ( hwnd : super::super::Foundation:: HWND , szcmdname : ::windows::core::PCSTR , szinfsection : ::windows::core::PCSTR , szdir : ::windows::core::PCSTR , lpsztitle : ::windows::core::PCSTR , phexe : *mut super::super::Foundation:: HANDLE , dwflags : u32 , pvreserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RunSetupCommandA(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), phexe, dwflags, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandW<P0, P1, P2, P3, P4>(hwnd: P0, szcmdname: P1, szinfsection: P2, szdir: P3, lpsztitle: P4, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn RunSetupCommandW ( hwnd : super::super::Foundation:: HWND , szcmdname : ::windows::core::PCWSTR , szinfsection : ::windows::core::PCWSTR , szdir : ::windows::core::PCWSTR , lpsztitle : ::windows::core::PCWSTR , phexe : *mut super::super::Foundation:: HANDLE , dwflags : u32 , pvreserved : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    RunSetupCommandW(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), phexe, dwflags, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExA<P0, P1>(param0: P0, param1: P1) -> super::super::Foundation::LRESULT
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn SendIMEMessageExA ( param0 : super::super::Foundation:: HWND , param1 : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: LRESULT );
    SendIMEMessageExA(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExW<P0, P1>(param0: P0, param1: P1) -> super::super::Foundation::LRESULT
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn SendIMEMessageExW ( param0 : super::super::Foundation:: HWND , param1 : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: LRESULT );
    SendIMEMessageExW(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentStringsA<P0>(newenvironment: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetEnvironmentStringsA ( newenvironment : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetEnvironmentStringsA(newenvironment.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pvalue: ::core::option::Option<*const ::core::ffi::c_void>, nsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFirmwareEnvironmentVariableA ( lpname : ::windows::core::PCSTR , lpguid : ::windows::core::PCSTR , pvalue : *const ::core::ffi::c_void , nsize : u32 ) -> super::super::Foundation:: BOOL );
    SetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null())), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pvalue: ::core::option::Option<*const ::core::ffi::c_void>, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFirmwareEnvironmentVariableExA ( lpname : ::windows::core::PCSTR , lpguid : ::windows::core::PCSTR , pvalue : *const ::core::ffi::c_void , nsize : u32 , dwattributes : u32 ) -> super::super::Foundation:: BOOL );
    SetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null())), nsize, dwattributes)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pvalue: ::core::option::Option<*const ::core::ffi::c_void>, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFirmwareEnvironmentVariableExW ( lpname : ::windows::core::PCWSTR , lpguid : ::windows::core::PCWSTR , pvalue : *const ::core::ffi::c_void , nsize : u32 , dwattributes : u32 ) -> super::super::Foundation:: BOOL );
    SetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null())), nsize, dwattributes)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pvalue: ::core::option::Option<*const ::core::ffi::c_void>, nsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetFirmwareEnvironmentVariableW ( lpname : ::windows::core::PCWSTR , lpguid : ::windows::core::PCWSTR , pvalue : *const ::core::ffi::c_void , nsize : u32 ) -> super::super::Foundation:: BOOL );
    SetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null())), nsize)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn SetHandleCount(unumber: u32) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetHandleCount ( unumber : u32 ) -> u32 );
    SetHandleCount(unumber)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMessageWaitingIndicator<P0>(hmsgindicator: P0, ulmsgcount: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetMessageWaitingIndicator ( hmsgindicator : super::super::Foundation:: HANDLE , ulmsgcount : u32 ) -> super::super::Foundation:: BOOL );
    SetMessageWaitingIndicator(hmsgindicator.into_param().abi(), ulmsgcount)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "advpack.dll""system" fn SetPerUserSecValuesA ( pperuser : *mut PERUSERSECTIONA ) -> ::windows::core::HRESULT );
    SetPerUserSecValuesA(pperuser).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "advpack.dll""system" fn SetPerUserSecValuesW ( pperuser : *mut PERUSERSECTIONW ) -> ::windows::core::HRESULT );
    SetPerUserSecValuesW(pperuser).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SignalObjectAndWait<P0, P1, P2>(hobjecttosignal: P0, hobjecttowaiton: P1, dwmilliseconds: u32, balertable: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SignalObjectAndWait ( hobjecttosignal : super::super::Foundation:: HANDLE , hobjecttowaiton : super::super::Foundation:: HANDLE , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: WIN32_ERROR );
    SignalObjectAndWait(hobjecttosignal.into_param().abi(), hobjecttowaiton.into_param().abi(), dwmilliseconds, balertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: PFEATURE_STATE_CHANGE_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-0.dll""system" fn SubscribeFeatureStateChangeNotification ( subscription : *mut FEATURE_STATE_CHANGE_SUBSCRIPTION , callback : PFEATURE_STATE_CHANGE_CALLBACK , context : *const ::core::ffi::c_void ) -> ( ) );
    SubscribeFeatureStateChangeNotification(subscription, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringA<P0, P1, P2, P3>(pszinffilename: P0, pszinstallsection: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: ::core::option::Option<&mut [u8]>, pdwrequiredsize: *mut u32, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn TranslateInfStringA ( pszinffilename : ::windows::core::PCSTR , pszinstallsection : ::windows::core::PCSTR , psztranslatesection : ::windows::core::PCSTR , psztranslatekey : ::windows::core::PCSTR , pszbuffer : ::windows::core::PSTR , cchbuffer : u32 , pdwrequiredsize : *mut u32 , pvreserved : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    TranslateInfStringA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(pszbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszbuffer.as_deref().map_or(0, |slice| slice.len() as _), pdwrequiredsize, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringExA<P0, P1, P2>(hinf: *mut ::core::ffi::c_void, pszinffilename: P0, psztranslatesection: P1, psztranslatekey: P2, pszbuffer: &mut [u8], pdwrequiredsize: *mut u32, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn TranslateInfStringExA ( hinf : *mut ::core::ffi::c_void , pszinffilename : ::windows::core::PCSTR , psztranslatesection : ::windows::core::PCSTR , psztranslatekey : ::windows::core::PCSTR , pszbuffer : ::windows::core::PSTR , dwbuffersize : u32 , pdwrequiredsize : *mut u32 , pvreserved : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    TranslateInfStringExA(hinf, pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, pdwrequiredsize, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringExW<P0, P1, P2>(hinf: *mut ::core::ffi::c_void, pszinffilename: P0, psztranslatesection: P1, psztranslatekey: P2, pszbuffer: &mut [u16], pdwrequiredsize: *mut u32, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn TranslateInfStringExW ( hinf : *mut ::core::ffi::c_void , pszinffilename : ::windows::core::PCWSTR , psztranslatesection : ::windows::core::PCWSTR , psztranslatekey : ::windows::core::PCWSTR , pszbuffer : ::windows::core::PWSTR , dwbuffersize : u32 , pdwrequiredsize : *mut u32 , pvreserved : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    TranslateInfStringExW(hinf, pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, pdwrequiredsize, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn TranslateInfStringW<P0, P1, P2, P3>(pszinffilename: P0, pszinstallsection: P1, psztranslatesection: P2, psztranslatekey: P3, pszbuffer: ::core::option::Option<&mut [u16]>, pdwrequiredsize: *mut u32, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn TranslateInfStringW ( pszinffilename : ::windows::core::PCWSTR , pszinstallsection : ::windows::core::PCWSTR , psztranslatesection : ::windows::core::PCWSTR , psztranslatekey : ::windows::core::PCWSTR , pszbuffer : ::windows::core::PWSTR , cchbuffer : u32 , pdwrequiredsize : *mut u32 , pvreserved : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    TranslateInfStringW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::core::mem::transmute(pszbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszbuffer.as_deref().map_or(0, |slice| slice.len() as _), pdwrequiredsize, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn UnsubscribeFeatureStateChangeNotification<P0>(subscription: P0)
where
    P0: ::windows::core::IntoParam<FEATURE_STATE_CHANGE_SUBSCRIPTION>,
{
    ::windows_targets::link ! ( "api-ms-win-core-featurestaging-l1-1-0.dll""system" fn UnsubscribeFeatureStateChangeNotification ( subscription : FEATURE_STATE_CHANGE_SUBSCRIPTION ) -> ( ) );
    UnsubscribeFeatureStateChangeNotification(subscription.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperA<P0, P1, P2>(hwnd: P0, hinstance: P1, pszparms: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn UserInstStubWrapperA ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PCSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    UserInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperW<P0, P1, P2>(hwnd: P0, hinstance: P1, pszparms: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn UserInstStubWrapperW ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PCWSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    UserInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperA<P0, P1, P2>(hwnd: P0, hinstance: P1, pszparms: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn UserUnInstStubWrapperA ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PCSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    UserUnInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperW<P0, P1, P2>(hwnd: P0, hinstance: P1, pszparms: P2, nshow: i32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "advpack.dll""system" fn UserUnInstStubWrapperW ( hwnd : super::super::Foundation:: HWND , hinstance : super::super::Foundation:: HMODULE , pszparms : ::windows::core::PCWSTR , nshow : i32 ) -> ::windows::core::HRESULT );
    UserUnInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), nshow).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSEnableIME<P0, P1>(param0: P0, param1: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn WINNLSEnableIME ( param0 : super::super::Foundation:: HWND , param1 : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    WINNLSEnableIME(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetEnableStatus<P0>(param0: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn WINNLSGetEnableStatus ( param0 : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    WINNLSGetEnableStatus(param0.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetIMEHotkey<P0>(param0: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "user32.dll""system" fn WINNLSGetIMEHotkey ( param0 : super::super::Foundation:: HWND ) -> u32 );
    WINNLSGetIMEHotkey(param0.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn WinWatchClose<P0>(hww: P0)
where
    P0: ::windows::core::IntoParam<HWINWATCH>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn WinWatchClose ( hww : HWINWATCH ) -> ( ) );
    WinWatchClose(hww.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchDidStatusChange<P0>(hww: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HWINWATCH>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn WinWatchDidStatusChange ( hww : HWINWATCH ) -> super::super::Foundation:: BOOL );
    WinWatchDidStatusChange(hww.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn WinWatchGetClipList<P0>(hww: P0, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32
where
    P0: ::windows::core::IntoParam<HWINWATCH>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn WinWatchGetClipList ( hww : HWINWATCH , prc : *mut super::super::Foundation:: RECT , size : u32 , prd : *mut super::super::Graphics::Gdi:: RGNDATA ) -> u32 );
    WinWatchGetClipList(hww.into_param().abi(), prc, size, prd)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchNotify<P0, P1>(hww: P0, notifycallback: WINWATCHNOTIFYPROC, notifyparam: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HWINWATCH>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn WinWatchNotify ( hww : HWINWATCH , notifycallback : WINWATCHNOTIFYPROC , notifyparam : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    WinWatchNotify(hww.into_param().abi(), notifycallback, notifyparam.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchOpen<P0>(hwnd: P0) -> HWINWATCH
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "dciman32.dll""system" fn WinWatchOpen ( hwnd : super::super::Foundation:: HWND ) -> HWINWATCH );
    WinWatchOpen(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpGetLockdownPolicy(hostinformation: ::core::option::Option<*const WLDP_HOST_INFORMATION>, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpGetLockdownPolicy ( hostinformation : *const WLDP_HOST_INFORMATION , lockdownstate : *mut u32 , lockdownflags : u32 ) -> ::windows::core::HRESULT );
    WldpGetLockdownPolicy(::core::mem::transmute(hostinformation.unwrap_or(::std::ptr::null())), lockdownstate, lockdownflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsClassInApprovedList(classid: *const ::windows::core::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpIsClassInApprovedList ( classid : *const ::windows::core::GUID , hostinformation : *const WLDP_HOST_INFORMATION , isapproved : *mut super::super::Foundation:: BOOL , optionalflags : u32 ) -> ::windows::core::HRESULT );
    WldpIsClassInApprovedList(classid, hostinformation, isapproved, optionalflags).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsDynamicCodePolicyEnabled() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpIsDynamicCodePolicyEnabled ( isenabled : *mut super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    WldpIsDynamicCodePolicyEnabled(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn WldpQueryDeviceSecurityInformation(information: ::core::option::Option<&mut [WLDP_DEVICE_SECURITY_INFORMATION]>, returnlength: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpQueryDeviceSecurityInformation ( information : *mut WLDP_DEVICE_SECURITY_INFORMATION , informationlength : u32 , returnlength : *mut u32 ) -> ::windows::core::HRESULT );
    WldpQueryDeviceSecurityInformation(::core::mem::transmute(information.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), information.as_deref().map_or(0, |slice| slice.len() as _), returnlength).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpQueryDynamicCodeTrust<P0>(filehandle: P0, baseimage: ::core::option::Option<*const ::core::ffi::c_void>, imagesize: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpQueryDynamicCodeTrust ( filehandle : super::super::Foundation:: HANDLE , baseimage : *const ::core::ffi::c_void , imagesize : u32 ) -> ::windows::core::HRESULT );
    WldpQueryDynamicCodeTrust(filehandle.into_param().abi(), ::core::mem::transmute(baseimage.unwrap_or(::std::ptr::null())), imagesize).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpSetDynamicCodeTrust<P0>(filehandle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wldp.dll""system" fn WldpSetDynamicCodeTrust ( filehandle : super::super::Foundation:: HANDLE ) -> ::windows::core::HRESULT );
    WldpSetDynamicCodeTrust(filehandle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionA<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileSectionA ( lpappname : ::windows::core::PCSTR , lpstring : ::windows::core::PCSTR , lpfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionW<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileSectionW ( lpappname : ::windows::core::PCWSTR , lpstring : ::windows::core::PCWSTR , lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringA<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileStringA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , lpstring : ::windows::core::PCSTR , lpfilename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringW<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileStringW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , lpstring : ::windows::core::PCWSTR , lpfilename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructA<P0, P1, P2>(lpszsection: P0, lpszkey: P1, lpstruct: ::core::option::Option<*const ::core::ffi::c_void>, usizestruct: u32, szfile: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileStructA ( lpszsection : ::windows::core::PCSTR , lpszkey : ::windows::core::PCSTR , lpstruct : *const ::core::ffi::c_void , usizestruct : u32 , szfile : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct.unwrap_or(::std::ptr::null())), usizestruct, szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructW<P0, P1, P2>(lpszsection: P0, lpszkey: P1, lpstruct: ::core::option::Option<*const ::core::ffi::c_void>, usizestruct: u32, szfile: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WritePrivateProfileStructW ( lpszsection : ::windows::core::PCWSTR , lpszkey : ::windows::core::PCWSTR , lpstruct : *const ::core::ffi::c_void , usizestruct : u32 , szfile : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WritePrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::core::mem::transmute(lpstruct.unwrap_or(::std::ptr::null())), usizestruct, szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionA<P0, P1>(lpappname: P0, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteProfileSectionA ( lpappname : ::windows::core::PCSTR , lpstring : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    WriteProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionW<P0, P1>(lpappname: P0, lpstring: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteProfileSectionW ( lpappname : ::windows::core::PCWSTR , lpstring : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WriteProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteProfileStringA ( lpappname : ::windows::core::PCSTR , lpkeyname : ::windows::core::PCSTR , lpstring : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    WriteProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteProfileStringW ( lpappname : ::windows::core::PCWSTR , lpkeyname : ::windows::core::PCWSTR , lpstring : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WriteProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _hread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, lbytes: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _hread ( hfile : i32 , lpbuffer : *mut ::core::ffi::c_void , lbytes : i32 ) -> i32 );
    _hread(hfile, lpbuffer, lbytes)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _hwrite(hfile: i32, lpbuffer: &[u8]) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _hwrite ( hfile : i32 , lpbuffer : ::windows::core::PCSTR , lbytes : i32 ) -> i32 );
    _hwrite(hfile, ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lclose(hfile: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _lclose ( hfile : i32 ) -> i32 );
    _lclose(hfile)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lcreat<P0>(lppathname: P0, iattribute: i32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn _lcreat ( lppathname : ::windows::core::PCSTR , iattribute : i32 ) -> i32 );
    _lcreat(lppathname.into_param().abi(), iattribute)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _llseek ( hfile : i32 , loffset : i32 , iorigin : i32 ) -> i32 );
    _llseek(hfile, loffset, iorigin)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lopen<P0>(lppathname: P0, ireadwrite: i32) -> i32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn _lopen ( lppathname : ::windows::core::PCSTR , ireadwrite : i32 ) -> i32 );
    _lopen(lppathname.into_param().abi(), ireadwrite)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, ubytes: u32) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _lread ( hfile : i32 , lpbuffer : *mut ::core::ffi::c_void , ubytes : u32 ) -> u32 );
    _lread(hfile, lpbuffer, ubytes)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[inline]
pub unsafe fn _lwrite(hfile: i32, lpbuffer: &[u8]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn _lwrite ( hfile : i32 , lpbuffer : ::windows::core::PCSTR , ubytes : u32 ) -> u32 );
    _lwrite(hfile, ::core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_lstrcmpW ( string1 : *const u16 , string2 : *const u16 ) -> i32 );
    uaw_lstrcmpW(string1, string2)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_lstrcmpiW ( string1 : *const u16 , string2 : *const u16 ) -> i32 );
    uaw_lstrcmpiW(string1, string2)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_lstrlenW(string: *const u16) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_lstrlenW ( string : *const u16 ) -> i32 );
    uaw_lstrlenW(string)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_wcschr ( string : *const u16 , character : u16 ) -> *mut u16 );
    uaw_wcschr(string, character)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_wcscpy ( destination : *mut u16 , source : *const u16 ) -> *mut u16 );
    uaw_wcscpy(destination, source)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_wcsicmp ( string1 : *const u16 , string2 : *const u16 ) -> i32 );
    uaw_wcsicmp(string1, string2)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcslen(string: *const u16) -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_wcslen ( string : *const u16 ) -> usize );
    uaw_wcslen(string)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn uaw_wcsrchr ( string : *const u16 , character : u16 ) -> *mut u16 );
    uaw_wcsrchr(string, character)
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct ICameraUIControl(::windows::core::IUnknown);
impl ICameraUIControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0, P1, P2>(&self, pwindow: P0, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: P1, peventcallback: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ICameraUIControlEventCallback>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), pwindow.into_param().abi(), mode, selectionmode, capturemode, photoformat, videoformat, bhasclosebutton.into_param().abi(), peventcallback.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Suspend)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrentViewType(&self) -> ::windows::core::Result<CameraUIControlViewType> {
        let mut result__ = ::windows::core::zeroed::<CameraUIControlViewType>();
        (::windows::core::Interface::vtable(self).GetCurrentViewType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveItem(&self, pbstractiveitempath: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActiveItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstractiveitempath.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedItems(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetSelectedItems)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveCapturedItem<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveCapturedItem)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICameraUIControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICameraUIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControl {}
impl ::core::fmt::Debug for ICameraUIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICameraUIControl {
    type Vtable = ICameraUIControl_Vtbl;
}
impl ::core::clone::Clone for ICameraUIControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICameraUIControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8733adf_3d68_4b8f_bb08_e28a0bed0376);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Suspend: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentViewType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows::core::HRESULT,
    pub GetActiveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelectedItems: usize,
    pub RemoveCapturedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct ICameraUIControlEventCallback(::windows::core::IUnknown);
impl ICameraUIControlEventCallback {
    pub unsafe fn OnStartupComplete(&self) {
        (::windows::core::Interface::vtable(self).OnStartupComplete)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn OnSuspendComplete(&self) {
        (::windows::core::Interface::vtable(self).OnSuspendComplete)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn OnItemCaptured<P0>(&self, pszpath: P0)
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnItemCaptured)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi())
    }
    pub unsafe fn OnItemDeleted<P0>(&self, pszpath: P0)
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnItemDeleted)(::windows::core::Interface::as_raw(self), pszpath.into_param().abi())
    }
    pub unsafe fn OnClosed(&self) {
        (::windows::core::Interface::vtable(self).OnClosed)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(ICameraUIControlEventCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICameraUIControlEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraUIControlEventCallback {}
impl ::core::fmt::Debug for ICameraUIControlEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraUIControlEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICameraUIControlEventCallback {
    type Vtable = ICameraUIControlEventCallback_Vtbl;
}
impl ::core::clone::Clone for ICameraUIControlEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICameraUIControlEventCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bfa0c2c_fbcd_4776_bda4_88bf974e74f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControlEventCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStartupComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnSuspendComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnItemCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR),
    pub OnItemDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR),
    pub OnClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IClipServiceNotificationHelper(::windows::core::IUnknown);
impl IClipServiceNotificationHelper {
    pub unsafe fn ShowToast<P0, P1, P2, P3, P4>(&self, titletext: P0, bodytext: P1, packagename: P2, appid: P3, launchcommand: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ShowToast)(::windows::core::Interface::as_raw(self), titletext.into_param().abi(), bodytext.into_param().abi(), packagename.into_param().abi(), appid.into_param().abi(), launchcommand.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IClipServiceNotificationHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IClipServiceNotificationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClipServiceNotificationHelper {}
impl ::core::fmt::Debug for IClipServiceNotificationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClipServiceNotificationHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClipServiceNotificationHelper {
    type Vtable = IClipServiceNotificationHelper_Vtbl;
}
impl ::core::clone::Clone for IClipServiceNotificationHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClipServiceNotificationHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc39948f0_6142_44fd_98ca_e1681a8d68b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipServiceNotificationHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ShowToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titletext: ::std::mem::MaybeUninit<::windows::core::BSTR>, bodytext: ::std::mem::MaybeUninit<::windows::core::BSTR>, packagename: ::std::mem::MaybeUninit<::windows::core::BSTR>, appid: ::std::mem::MaybeUninit<::windows::core::BSTR>, launchcommand: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IContainerActivationHelper(::windows::core::IUnknown);
impl IContainerActivationHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanActivateClientVM(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).CanActivateClientVM)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IContainerActivationHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IContainerActivationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContainerActivationHelper {}
impl ::core::fmt::Debug for IContainerActivationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContainerActivationHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContainerActivationHelper {
    type Vtable = IContainerActivationHelper_Vtbl;
}
impl ::core::clone::Clone for IContainerActivationHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContainerActivationHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb524f93f_80d5_4ec7_ae9e_d66e93ade1fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerActivationHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CanActivateClientVM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanActivateClientVM: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IDefaultBrowserSyncSettings(::windows::core::IUnknown);
impl IDefaultBrowserSyncSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(IDefaultBrowserSyncSettings, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDefaultBrowserSyncSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultBrowserSyncSettings {}
impl ::core::fmt::Debug for IDefaultBrowserSyncSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultBrowserSyncSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDefaultBrowserSyncSettings {
    type Vtable = IDefaultBrowserSyncSettings_Vtbl;
}
impl ::core::clone::Clone for IDefaultBrowserSyncSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDefaultBrowserSyncSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a27faad_5ae6_4255_9030_c530936292e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultBrowserSyncSettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IDeleteBrowsingHistory(::windows::core::IUnknown);
impl IDeleteBrowsingHistory {
    pub unsafe fn DeleteBrowsingHistory(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteBrowsingHistory)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IDeleteBrowsingHistory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDeleteBrowsingHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeleteBrowsingHistory {}
impl ::core::fmt::Debug for IDeleteBrowsingHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeleteBrowsingHistory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeleteBrowsingHistory {
    type Vtable = IDeleteBrowsingHistory_Vtbl;
}
impl ::core::clone::Clone for IDeleteBrowsingHistory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeleteBrowsingHistory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf38ed4b_2be7_4461_8b5e_9a466dc82ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteBrowsingHistory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DeleteBrowsingHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IEditionUpgradeBroker(::windows::core::IUnknown);
impl IEditionUpgradeBroker {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn InitializeParentWindow<P0>(&self, parenthandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Ole::OLE_HANDLE>,
    {
        (::windows::core::Interface::vtable(self).InitializeParentWindow)(::windows::core::Interface::as_raw(self), parenthandle.into_param().abi()).ok()
    }
    pub unsafe fn UpdateOperatingSystem<P0>(&self, parameter: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).UpdateOperatingSystem)(::windows::core::Interface::as_raw(self), parameter.into_param().abi()).ok()
    }
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowProductKeyUI)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanUpgrade(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CanUpgrade)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IEditionUpgradeBroker, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEditionUpgradeBroker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeBroker {}
impl ::core::fmt::Debug for IEditionUpgradeBroker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeBroker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEditionUpgradeBroker {
    type Vtable = IEditionUpgradeBroker_Vtbl;
}
impl ::core::clone::Clone for IEditionUpgradeBroker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEditionUpgradeBroker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff19cbcf_9455_4937_b872_6b7929a460af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeBroker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub InitializeParentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parenthandle: super::Ole::OLE_HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    InitializeParentWindow: usize,
    pub UpdateOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ShowProductKeyUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IEditionUpgradeHelper(::windows::core::IUnknown);
impl IEditionUpgradeHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanUpgrade(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanUpgrade)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateOperatingSystem<P0>(&self, contentid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UpdateOperatingSystem)(::windows::core::Interface::as_raw(self), contentid.into_param().abi()).ok()
    }
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowProductKeyUI)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetOsProductContentId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetOsProductContentId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGenuineLocalStatus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetGenuineLocalStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEditionUpgradeHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEditionUpgradeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEditionUpgradeHelper {}
impl ::core::fmt::Debug for IEditionUpgradeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEditionUpgradeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEditionUpgradeHelper {
    type Vtable = IEditionUpgradeHelper_Vtbl;
}
impl ::core::clone::Clone for IEditionUpgradeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEditionUpgradeHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e9e342_5deb_43b6_849e_6913b85d503a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CanUpgrade: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanUpgrade: usize,
    pub UpdateOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ShowProductKeyUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOsProductContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGenuineLocalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGenuineLocalStatus: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IFClipNotificationHelper(::windows::core::IUnknown);
impl IFClipNotificationHelper {
    pub unsafe fn ShowSystemDialog<P0, P1>(&self, titletext: P0, bodytext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ShowSystemDialog)(::windows::core::Interface::as_raw(self), titletext.into_param().abi(), bodytext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IFClipNotificationHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IFClipNotificationHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFClipNotificationHelper {}
impl ::core::fmt::Debug for IFClipNotificationHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFClipNotificationHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFClipNotificationHelper {
    type Vtable = IFClipNotificationHelper_Vtbl;
}
impl ::core::clone::Clone for IFClipNotificationHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFClipNotificationHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d5e3d21_bd41_4c2a_a669_b17ce87fb50b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFClipNotificationHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ShowSystemDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titletext: ::std::mem::MaybeUninit<::windows::core::BSTR>, bodytext: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
pub struct IWindowsLockModeHelper(::windows::core::IUnknown);
impl IWindowsLockModeHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetSMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWindowsLockModeHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWindowsLockModeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsLockModeHelper {}
impl ::core::fmt::Debug for IWindowsLockModeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsLockModeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWindowsLockModeHelper {
    type Vtable = IWindowsLockModeHelper_Vtbl;
}
impl ::core::clone::Clone for IWindowsLockModeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWindowsLockModeHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf342d19e_cc22_4648_bb5d_03ccf75b47c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsLockModeHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSMode: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AADBE_ADD_ENTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AADBE_DEL_ENTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_OFFLINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_ONLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AC_LINE_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_BACKNEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NODELETENEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_NOPROGRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_UPDREFCNT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AFSR_USEREFCNT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOOVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_QUIET: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_REPLACEONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AIF_WARNIFSKIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_BKINSTALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_CHECKBKDATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_NGCONV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_QUIET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_ROLLBACK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_REGSECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ARSR_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_ARP: u32 = 640u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_NULL: u32 = 642u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BACKUP_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_LOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_075: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_110: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_115200: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_1200: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_128K: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_134_5: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_14400: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_150: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_1800: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_19200: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_2400: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_300: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_38400: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_4800: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_56K: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_57600: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_600: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_7200: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_9600: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const BAUD_USER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CATID_DeleteBrowsingHistory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31caf6e4_d6aa_4090_a050_a5ac8972e9ef);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_110: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_115200: u32 = 115200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_1200: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_128000: u32 = 128000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_14400: u32 = 14400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_19200: u32 = 19200u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_2400: u32 = 2400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_256000: u32 = 256000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_300: u32 = 300u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_38400: u32 = 38400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_4800: u32 = 4800u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_56000: u32 = 56000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_57600: u32 = 57600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_600: u32 = 600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CBR_9600: u32 = 9600u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_DNS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_IOE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_MODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_OOP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_PTO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CE_TXFULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_IP: u32 = 771u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_IPX: u32 = 769u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_NBF: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_UDP: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COMMPROP_INITIALIZED: u32 = 3879531822u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CONTEXT_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_IO_RATE_MIN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE2_V2_DONT_COPY_JUNCTIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE2_V2_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DIRECTORY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_ENABLE_SPARSE_COPY: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_NO_BUFFERING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_RESTARTABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_NBF: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_SPP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_SPX: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_TCP: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_DIRECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_HWND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_LEVEL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CP_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CREATE_FOR_DIR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CREATE_FOR_IMPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CameraUIControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16d5a2be_b1c5_47b3_8eae_ccbcf452c7e8);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCIENUMSURFACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCIESCAPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_1632_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ASYNC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CANOVERLAY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_CHROMAKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_DWORDALIGN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_DWORDSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_XALIGN: i32 = -17i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_XYALIGN: i32 = -19i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_ERR_YALIGN: i32 = -18i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_GENERIC: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OFFSCREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_OVERLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_PRIMARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_SURFACE_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_VISIBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DCI_WRITEONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_DOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_CDROM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_FIXED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_RAMDISK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DRIVE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DefaultBrowserSyncSettings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ac83423_3112_4aa6_9b5b_1feb23d0c5f9);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ENTITY_LIST_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ENTITY_TYPE_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ER_ICMP: u32 = 896u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EditionUpgradeBroker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4270827_4f39_45df_9288_12ff6b85a921);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EditionUpgradeHelper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01776df3_b9af_4e50_9b1c_56e93116d704);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_CREATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DELETE_ON_CLOSE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DIRECTORY_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_ENCRYPTABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_EXISTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NON_DIRECTORY_FILE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPENED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_OVERWRITTEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RANDOM_ACCESS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_READ_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_RESERVE_OPFILTER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_ROOT_DIR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SEQUENTIAL_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SUPERSEDED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_DIR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_UNKNOWN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_USER_DISALLOWED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_WRITE_THROUGH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_CASE_SENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_FILE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: ::windows::core::PCSTR = ::windows::core::s!("GetSystemWow64DirectoryA");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryA");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryA");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: ::windows::core::PCSTR = ::windows::core::s!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: ::windows::core::PCWSTR = ::windows::core::w!("GetSystemWow64DirectoryW");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DDESHARE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DISCARDABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_DISCARDED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_LOCKCOUNT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_LOWER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_MODIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOCOMPACT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NODISCARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOTIFY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_NOT_BANKED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_SHARE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HANJA_WINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HINSTANCE_ERROR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_BACKNEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_FRDOALL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NODELETENEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOENUMKEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NOPROGRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_REGSECTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_RESTORE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_UPDREFCNT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE4_USEREFCNT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BADID: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BAUDRATE: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_BYTESIZE: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_DEFAULT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_HARDWARE: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_MEMORY: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_NOPEN: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IE_OPEN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_GENERIC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_MIB: u32 = 514u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IGNORE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_INIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IMEA_PREV: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_BANJAtoJUNJA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_ENABLE_CONVERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETIMECAPS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETOPEN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_GETVERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_JOHABtoKS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_JUNJAtoBANJA: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_KStoJOHAB: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MAXPROCESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_CODEINPUT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_HIRAGANA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_KATAKANA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_NOROMAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_ROMAN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_REQUEST_CONVERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_DISKERROR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_ILLEGAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_INVALID: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NEST: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOROOM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_NOTFOUND: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_RS_TOOLONG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SENDVKEY: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SETOPEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IME_SET_MODE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_GENERIC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INTERIM_WINDOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_CHANGECONVERT: u32 = 289u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_CLOSECONVERT: u32 = 290u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_DBCSCHAR: u32 = 352u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_FULLCONVERT: u32 = 291u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_IMESELECT: u32 = 304u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_MODEINFO: u32 = 400u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_OPENCONVERT: u32 = 288u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRING: u32 = 320u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGEND: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGEX: u32 = 384u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_STRINGSTART: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IR_UNDETERMINE: u32 = 368u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LIS_NOGRPCONV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LIS_QUIET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const LPTx: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAXINTATOM: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_HIDDEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_RECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_SCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_VERTICAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MCW_WINDOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MODE_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OFS_MAXPATHNAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OPERATION_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const OVERWRITE_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_16BITMODE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_DTRDSR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_INTTIMEOUTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_PARITY_CHECK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_RLSD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_RTSCTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_SETXCHAR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_SPECIALCHARS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_TOTALTIMEOUTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PCF_XONXOFF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_CANCEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_CONTINUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_QUIET: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROGRESS_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_FAX: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_LAT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_MODEM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_NETWORK_BRIDGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_PARALLELPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_RS232: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_RS422: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_RS423: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_RS449: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_SCANNER: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_TCPIP_TELNET: u32 = 258u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const PST_X25: u32 = 259u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REG_RESTORE_LOG_KEY: ::windows::core::PCWSTR = ::windows::core::w!("RegRestoreLogFile");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REG_SAVE_LOG_KEY: ::windows::core::PCWSTR = ::windows::core::w!("RegSaveLogFile");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RESETDEV: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_INF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_NGCONV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_QUIET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_NOWAIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const RUNCMDS_QUIET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_32BIT_BINARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_64BIT_BINARY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_DOS_BINARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_OS216_BINARY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_PIF_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_POSIX_BINARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SCS_WOW_BINARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SHUTDOWN_NORETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_BAUD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_DATABITS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_HANDSHAKING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_PARITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_PARITY_CHECK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_RLSD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_SERIALCOMM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SP_STOPBITS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_ALLTHRESHOLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_LEGATO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD1024: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD2048: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIOD512: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_PERIODVOICE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_QUEUEEMPTY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERBDNT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDCC: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDDR: i32 = -14i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDFQ: i32 = -13i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDLN: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDMD: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDPT: i32 = -12i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDSH: i32 = -11i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDSR: i32 = -15i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDST: i32 = -16i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDTP: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDVL: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERDVNA: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERMACT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SEROFM: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_SERQFUL: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_STACCATO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE1024: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE2048: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITE512: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const S_WHITEVOICE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_GP_TRAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_HARDERR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const TC_SIGNAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const UMS_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_DOS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_GUID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_NONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VOLUME_NAME_NT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_CANEXECUTEBUFFER_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpCanExecuteBuffer");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_CANEXECUTEFILE_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpCanExecuteFile");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_DLL: ::windows::core::PCWSTR = ::windows::core::w!("WLDP.DLL");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_GETLOCKDOWNPOLICY_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpGetLockdownPolicy");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_CMD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5baea1d6_6f1c_488e_8490_347fa5c5067f);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a71b6_fe56_48d6_9543_2dff0ecded66);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_JAVASCRIPT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5629f0d5_1cca_4fed_a1a3_36a8c18d74c0);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_MSI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x624eb611_6e7e_4eec_9bfe_f0ecdbfcf390);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_OTHER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x626cbec3_e1fa_4227_9800_ed210274cf7c);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_POWERSHELL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e9aaa7c_198b_4879_ae41_a50d47ad6458);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_PYTHON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfd557ef_2448_42ec_810b_0d9f09352d4a);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_WINDOWS_SCRIPT_HOST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd30b84c5_29ce_4ff3_86ec_a30007a82e49);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_XML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5594be58_c6bf_4295_82f4_d494d20e3a36);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISAPPAPPROVEDBYPOLICY_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpIsAppApprovedByPolicy");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISCLASSINAPPROVEDLIST_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpIsClassInApprovedList");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISDYNAMICCODEPOLICYENABLED_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpIsDynamicCodePolicyEnabled");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISPRODUCTIONCONFIGURATION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpIsProductionConfiguration");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_ISWCOSPRODUCTIONCONFIGURATION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpIsWcosProductionConfiguration");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDANAMICCODETRUST_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryDynamicCodeTrust");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDEVICESECURITYINFORMATION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryDeviceSecurityInformation");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYDYNAMICCODETRUST_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryDynamicCodeTrust");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYPOLICYSETTINGENABLED2_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryPolicySettingEnabled2");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYPOLICYSETTINGENABLED_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryPolicySettingEnabled");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYWINDOWSLOCKDOWNMODE_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryWindowsLockdownMode");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpQueryWindowsLockdownRestriction");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_RESETPRODUCTIONCONFIGURATION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpResetProductionConfiguration");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_RESETWCOSPRODUCTIONCONFIGURATION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpResetWcosProductionConfiguration");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_SETDYNAMICCODETRUST_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpSetDynamicCodeTrust");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_SETWINDOWSLOCKDOWNRESTRICTION_FN: ::windows::core::PCSTR = ::windows::core::s!("WldpSetWindowsLockdownRestriction");
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_CONVERTREQUEST: u32 = 266u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_CONVERTRESULT: u32 = 267u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IMEKEYDOWN: u32 = 656u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IMEKEYUP: u32 = 657u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_IME_REPORT: u32 = 640u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_INTERIM: u32 = 268u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlCaptureMode {}
impl ::core::clone::Clone for CameraUIControlCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlCaptureMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlCaptureMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlLinearSelectionMode {}
impl ::core::clone::Clone for CameraUIControlLinearSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlLinearSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlLinearSelectionMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlLinearSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlLinearSelectionMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlMode {}
impl ::core::clone::Clone for CameraUIControlMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlPhotoFormat {}
impl ::core::clone::Clone for CameraUIControlPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlPhotoFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlPhotoFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlVideoFormat {}
impl ::core::clone::Clone for CameraUIControlVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlVideoFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlVideoFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlViewType {}
impl ::core::clone::Clone for CameraUIControlViewType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraUIControlViewType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CameraUIControlViewType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CameraUIControlViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraUIControlViewType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DECISION_LOCATION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = DECISION_LOCATION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = DECISION_LOCATION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = DECISION_LOCATION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = DECISION_LOCATION(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = DECISION_LOCATION(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(6i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = DECISION_LOCATION(7i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = DECISION_LOCATION(8i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = DECISION_LOCATION(9i32);
impl ::core::marker::Copy for DECISION_LOCATION {}
impl ::core::clone::Clone for DECISION_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DECISION_LOCATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DECISION_LOCATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DECISION_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DECISION_LOCATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEATURE_CHANGE_TIME(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(3i32);
impl ::core::marker::Copy for FEATURE_CHANGE_TIME {}
impl ::core::clone::Clone for FEATURE_CHANGE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_CHANGE_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FEATURE_CHANGE_TIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FEATURE_CHANGE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_CHANGE_TIME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEATURE_ENABLED_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(2i32);
impl ::core::marker::Copy for FEATURE_ENABLED_STATE {}
impl ::core::clone::Clone for FEATURE_ENABLED_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEATURE_ENABLED_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FEATURE_ENABLED_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FEATURE_ENABLED_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_ENABLED_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_FLUSH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLUSH_DEFAULT: FILE_FLUSH_MODE = FILE_FLUSH_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLUSH_DATA: FILE_FLUSH_MODE = FILE_FLUSH_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLUSH_MIN_METADATA: FILE_FLUSH_MODE = FILE_FLUSH_MODE(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_FLUSH_NO_SYNC: FILE_FLUSH_MODE = FILE_FLUSH_MODE(3i32);
impl ::core::marker::Copy for FILE_FLUSH_MODE {}
impl ::core::clone::Clone for FILE_FLUSH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_FLUSH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_FLUSH_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_FLUSH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_FLUSH_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for FILE_INFORMATION_CLASS {}
impl ::core::clone::Clone for FILE_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILE_WRITE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_WRITE_FLAGS_NONE: FILE_WRITE_FLAGS = FILE_WRITE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const FILE_WRITE_FLAGS_WRITE_THROUGH: FILE_WRITE_FLAGS = FILE_WRITE_FLAGS(1i32);
impl ::core::marker::Copy for FILE_WRITE_FLAGS {}
impl ::core::clone::Clone for FILE_WRITE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FILE_WRITE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_WRITE_FLAGS").field(&self.0).finish()
    }
}
impl FILE_WRITE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FILE_WRITE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_WRITE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_WRITE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_WRITE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_WRITE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
impl ::core::marker::Copy for KEY_SET_INFORMATION_CLASS {}
impl ::core::clone::Clone for KEY_SET_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEY_SET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for KEY_SET_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECT_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for OBJECT_INFORMATION_CLASS {}
impl ::core::clone::Clone for OBJECT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OBJECT_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OBJECT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(23i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(33i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(37i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(45i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(103i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(134i32);
impl ::core::marker::Copy for SYSTEM_INFORMATION_CLASS {}
impl ::core::clone::Clone for SYSTEM_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSTEM_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYSTEM_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYSTEM_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TDIENTITY_ENTITY_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(640u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(769u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(768u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1025u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1024u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(896u32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(512u32);
impl ::core::marker::Copy for TDIENTITY_ENTITY_TYPE {}
impl ::core::clone::Clone for TDIENTITY_ENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDIENTITY_ENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TDIENTITY_ENTITY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TDIENTITY_ENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDIENTITY_ENTITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TDI_TL_IO_CONTROL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(3i32);
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_TYPE {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TDI_TL_IO_CONTROL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TDI_TL_IO_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TDI_TL_IO_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VALUENAME(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_UNKNOWN: VALUENAME = VALUENAME(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = VALUENAME(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = VALUENAME(2i32);
impl ::core::marker::Copy for VALUENAME {}
impl ::core::clone::Clone for VALUENAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VALUENAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VALUENAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VALUENAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALUENAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WINSTATIONINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WinStationInformation: WINSTATIONINFOCLASS = WINSTATIONINFOCLASS(8i32);
impl ::core::marker::Copy for WINSTATIONINFOCLASS {}
impl ::core::clone::Clone for WINSTATIONINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINSTATIONINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WINSTATIONINFOCLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WINSTATIONINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINSTATIONINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_EXECUTION_EVALUATION_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_EXECUTION_EVALUATION_OPTION_NONE: WLDP_EXECUTION_EVALUATION_OPTIONS = WLDP_EXECUTION_EVALUATION_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_EXECUTION_EVALUATION_OPTION_EXECUTE_IN_INTERACTIVE_SESSION: WLDP_EXECUTION_EVALUATION_OPTIONS = WLDP_EXECUTION_EVALUATION_OPTIONS(1i32);
impl ::core::marker::Copy for WLDP_EXECUTION_EVALUATION_OPTIONS {}
impl ::core::clone::Clone for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_EXECUTION_EVALUATION_OPTIONS").field(&self.0).finish()
    }
}
impl WLDP_EXECUTION_EVALUATION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WLDP_EXECUTION_EVALUATION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WLDP_EXECUTION_EVALUATION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_EXECUTION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_EXECUTION_POLICY_BLOCKED: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_EXECUTION_POLICY_ALLOWED: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_EXECUTION_POLICY_REQUIRE_SANDBOX: WLDP_EXECUTION_POLICY = WLDP_EXECUTION_POLICY(2i32);
impl ::core::marker::Copy for WLDP_EXECUTION_POLICY {}
impl ::core::clone::Clone for WLDP_EXECUTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_EXECUTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_EXECUTION_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_EXECUTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_EXECUTION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_HOST(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = WLDP_HOST(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_SVCHOST: WLDP_HOST = WLDP_HOST(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_MAX: WLDP_HOST = WLDP_HOST(2i32);
impl ::core::marker::Copy for WLDP_HOST {}
impl ::core::clone::Clone for WLDP_HOST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_HOST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_HOST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_HOST_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = WLDP_HOST_ID(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = WLDP_HOST_ID(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = WLDP_HOST_ID(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = WLDP_HOST_ID(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = WLDP_HOST_ID(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = WLDP_HOST_ID(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = WLDP_HOST_ID(6i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = WLDP_HOST_ID(7i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = WLDP_HOST_ID(8i32);
impl ::core::marker::Copy for WLDP_HOST_ID {}
impl ::core::clone::Clone for WLDP_HOST_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_HOST_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_HOST_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_HOST_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_HOST_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_KEY(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_UNKNOWN: WLDP_KEY = WLDP_KEY(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_OVERRIDE: WLDP_KEY = WLDP_KEY(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const KEY_ALL_KEYS: WLDP_KEY = WLDP_KEY(2i32);
impl ::core::marker::Copy for WLDP_KEY {}
impl ::core::clone::Clone for WLDP_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_KEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_KEY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_POLICY_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = WLDP_POLICY_SETTING(1000i32);
impl ::core::marker::Copy for WLDP_POLICY_SETTING {}
impl ::core::clone::Clone for WLDP_POLICY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_POLICY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_POLICY_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_POLICY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_POLICY_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_MODE {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_WINDOWS_LOCKDOWN_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLDP_WINDOWS_LOCKDOWN_RESTRICTION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(3i32);
impl ::core::marker::Copy for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {}
impl ::core::clone::Clone for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLDP_WINDOWS_LOCKDOWN_RESTRICTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_2600")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACTCTX_SECTION_KEYED_DATA_2600 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulDataFormatVersion == other.ulDataFormatVersion && self.lpData == other.lpData && self.ulLength == other.ulLength && self.lpSectionGlobalData == other.lpSectionGlobalData && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength && self.lpSectionBase == other.lpSectionBase && self.ulSectionTotalLength == other.ulSectionTotalLength && self.hActCtx == other.hActCtx && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut ::core::ffi::c_void,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA").field("lpInformation", &self.lpInformation).field("lpSectionBase", &self.lpSectionBase).field("ulSectionLength", &self.ulSectionLength).field("lpSectionGlobalDataBase", &self.lpSectionGlobalDataBase).field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength).finish()
    }
}
impl ::windows::core::TypeKind for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpInformation == other.lpInformation && self.lpSectionBase == other.lpSectionBase && self.ulSectionLength == other.ulSectionLength && self.lpSectionGlobalDataBase == other.lpSectionGlobalDataBase && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
    }
}
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_BASIC_INFORMATION").field("hActCtx", &self.hActCtx).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hActCtx == other.hActCtx && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct CABINFOA {
    pub pszCab: ::windows::core::PSTR,
    pub pszInf: ::windows::core::PSTR,
    pub pszSection: ::windows::core::PSTR,
    pub szSrcPath: [u8; 260],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CABINFOA {}
impl ::core::clone::Clone for CABINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOA").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for CABINFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CABINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CABINFOA {}
impl ::core::default::Default for CABINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct CABINFOW {
    pub pszCab: ::windows::core::PWSTR,
    pub pszInf: ::windows::core::PWSTR,
    pub pszSection: ::windows::core::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CABINFOW {}
impl ::core::clone::Clone for CABINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABINFOW").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for CABINFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CABINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CABINFOW {}
impl ::core::default::Default for CABINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_ID").field("UniqueProcess", &self.UniqueProcess).field("UniqueThread", &self.UniqueThread).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CLIENT_ID {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueProcess == other.UniqueProcess && self.UniqueThread == other.UniqueThread
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::clone::Clone for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG").field("Size", &self.Size).field("TriggerId", &self.TriggerId).finish()
    }
}
impl ::windows::core::TypeKind for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TriggerId == other.TriggerId
    }
}
impl ::core::cmp::Eq for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
impl ::core::default::Default for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
impl ::core::marker::Copy for DATETIME {}
impl ::core::clone::Clone for DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIME").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).finish()
    }
}
impl ::windows::core::TypeKind for DATETIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.min == other.min && self.sec == other.sec
    }
}
impl ::core::cmp::Eq for DATETIME {}
impl ::core::default::Default for DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DCICMD {}
impl ::core::clone::Clone for DCICMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICMD").field("dwCommand", &self.dwCommand).field("dwParam1", &self.dwParam1).field("dwParam2", &self.dwParam2).field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows::core::TypeKind for DCICMD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCICMD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommand == other.dwCommand && self.dwParam1 == other.dwParam1 && self.dwParam2 == other.dwParam2 && self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DCICMD {}
impl ::core::default::Default for DCICMD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DCICREATEINPUT {}
impl ::core::clone::Clone for DCICREATEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCICREATEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCICREATEINPUT").field("cmd", &self.cmd).field("dwCompression", &self.dwCompression).field("dwMask", &self.dwMask).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwDCICaps", &self.dwDCICaps).field("dwBitCount", &self.dwBitCount).field("lpSurface", &self.lpSurface).finish()
    }
}
impl ::windows::core::TypeKind for DCICREATEINPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCICREATEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.dwCompression == other.dwCompression && self.dwMask == other.dwMask && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwDCICaps == other.dwDCICaps && self.dwBitCount == other.dwBitCount && self.lpSurface == other.lpSurface
    }
}
impl ::core::cmp::Eq for DCICREATEINPUT {}
impl ::core::default::Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DCIENUMINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DCIENUMINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIENUMINPUT").field("cmd", &self.cmd).field("rSrc", &self.rSrc).field("rDst", &self.rDst).field("EnumCallback", &self.EnumCallback).field("lpContext", &self.lpContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DCIENUMINPUT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DCIENUMINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.rSrc == other.rSrc && self.rDst == other.rDst && self.EnumCallback == other.EnumCallback && self.lpContext == other.lpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
impl ::core::marker::Copy for DCIOFFSCREEN {}
impl ::core::clone::Clone for DCIOFFSCREEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOFFSCREEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOFFSCREEN").field("dciInfo", &self.dciInfo).field("Draw", &self.Draw).field("SetClipList", &self.SetClipList).field("SetDestination", &self.SetDestination).finish()
    }
}
impl ::windows::core::TypeKind for DCIOFFSCREEN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCIOFFSCREEN {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.Draw == other.Draw && self.SetClipList == other.SetClipList && self.SetDestination == other.SetDestination
    }
}
impl ::core::cmp::Eq for DCIOFFSCREEN {}
impl ::core::default::Default for DCIOFFSCREEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
impl ::core::marker::Copy for DCIOVERLAY {}
impl ::core::clone::Clone for DCIOVERLAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCIOVERLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCIOVERLAY").field("dciInfo", &self.dciInfo).field("dwChromakeyValue", &self.dwChromakeyValue).field("dwChromakeyMask", &self.dwChromakeyMask).finish()
    }
}
impl ::windows::core::TypeKind for DCIOVERLAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCIOVERLAY {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.dwChromakeyValue == other.dwChromakeyValue && self.dwChromakeyMask == other.dwChromakeyMask
    }
}
impl ::core::cmp::Eq for DCIOVERLAY {}
impl ::core::default::Default for DCIOVERLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl ::core::marker::Copy for DCISURFACEINFO {}
impl ::core::clone::Clone for DCISURFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCISURFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCISURFACEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lStride", &self.lStride)
            .field("dwBitCount", &self.dwBitCount)
            .field("dwOffSurface", &self.dwOffSurface)
            .field("wSelSurface", &self.wSelSurface)
            .field("wReserved", &self.wReserved)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("BeginAccess", &self.BeginAccess)
            .field("EndAccess", &self.EndAccess)
            .field("DestroySurface", &self.DestroySurface)
            .finish()
    }
}
impl ::windows::core::TypeKind for DCISURFACEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCISURFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwDCICaps == other.dwDCICaps && self.dwCompression == other.dwCompression && self.dwMask == other.dwMask && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.lStride == other.lStride && self.dwBitCount == other.dwBitCount && self.dwOffSurface == other.dwOffSurface && self.wSelSurface == other.wSelSurface && self.wReserved == other.wReserved && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3 && self.BeginAccess == other.BeginAccess && self.EndAccess == other.EndAccess && self.DestroySurface == other.DestroySurface
    }
}
impl ::core::cmp::Eq for DCISURFACEINFO {}
impl ::core::default::Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: ::windows::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for DELAYLOAD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(target_arch = "x86")]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: ::windows::core::PCSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for DELAYLOAD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DELAYLOAD_PROC_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: ::windows::core::PCSTR,
    pub Ordinal: u32,
}
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR_0 {}
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DELAYLOAD_PROC_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FEATURE_ERROR {
    pub hr: ::windows::core::HRESULT,
    pub lineNumber: u16,
    pub file: ::windows::core::PCSTR,
    pub process: ::windows::core::PCSTR,
    pub module: ::windows::core::PCSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: ::windows::core::PCSTR,
    pub message: ::windows::core::PCSTR,
    pub originLineNumber: u16,
    pub originFile: ::windows::core::PCSTR,
    pub originModule: ::windows::core::PCSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: ::windows::core::PCSTR,
    pub originName: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for FEATURE_ERROR {}
impl ::core::clone::Clone for FEATURE_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FEATURE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FEATURE_ERROR")
            .field("hr", &self.hr)
            .field("lineNumber", &self.lineNumber)
            .field("file", &self.file)
            .field("process", &self.process)
            .field("module", &self.module)
            .field("callerReturnAddressOffset", &self.callerReturnAddressOffset)
            .field("callerModule", &self.callerModule)
            .field("message", &self.message)
            .field("originLineNumber", &self.originLineNumber)
            .field("originFile", &self.originFile)
            .field("originModule", &self.originModule)
            .field("originCallerReturnAddressOffset", &self.originCallerReturnAddressOffset)
            .field("originCallerModule", &self.originCallerModule)
            .field("originName", &self.originName)
            .finish()
    }
}
impl ::windows::core::TypeKind for FEATURE_ERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FEATURE_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.lineNumber == other.lineNumber && self.file == other.file && self.process == other.process && self.module == other.module && self.callerReturnAddressOffset == other.callerReturnAddressOffset && self.callerModule == other.callerModule && self.message == other.message && self.originLineNumber == other.originLineNumber && self.originFile == other.originFile && self.originModule == other.originModule && self.originCallerReturnAddressOffset == other.originCallerReturnAddressOffset && self.originCallerModule == other.originCallerModule && self.originName == other.originName
    }
}
impl ::core::cmp::Eq for FEATURE_ERROR {}
impl ::core::default::Default for FEATURE_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FEATURE_STATE_CHANGE_SUBSCRIPTION(pub isize);
impl FEATURE_STATE_CHANGE_SUBSCRIPTION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FEATURE_STATE_CHANGE_SUBSCRIPTION {}
impl ::core::fmt::Debug for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEATURE_STATE_CHANGE_SUBSCRIPTION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_SERVICE_PIPE_HANDLE(pub isize);
impl FH_SERVICE_PIPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FH_SERVICE_PIPE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FH_SERVICE_PIPE_HANDLE {}
impl ::core::fmt::Debug for FH_SERVICE_PIPE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_SERVICE_PIPE_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FH_SERVICE_PIPE_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_CASE_SENSITIVE_INFO {}
impl ::core::clone::Clone for FILE_CASE_SENSITIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_CASE_SENSITIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_CASE_SENSITIVE_INFO").field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for FILE_CASE_SENSITIVE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_CASE_SENSITIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_CASE_SENSITIVE_INFO {}
impl ::core::default::Default for FILE_CASE_SENSITIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFO_EX {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO_EX").field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for FILE_DISPOSITION_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO_EX {}
impl ::core::default::Default for FILE_DISPOSITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HWINWATCH(pub isize);
impl HWINWATCH {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HWINWATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWINWATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWINWATCH {}
impl ::core::fmt::Debug for HWINWATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWINWATCH").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HWINWATCH {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u8; 39],
    pub szHwProfileName: [u8; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOA {}
impl ::core::clone::Clone for HW_PROFILE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HW_PROFILE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOA").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
impl ::windows::core::TypeKind for HW_PROFILE_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOA {}
impl ::core::default::Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOW {}
impl ::core::clone::Clone for HW_PROFILE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HW_PROFILE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HW_PROFILE_INFOW").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
impl ::windows::core::TypeKind for HW_PROFILE_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HW_PROFILE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::core::cmp::Eq for HW_PROFILE_INFOW {}
impl ::core::default::Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_DELAYLOAD_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DELAYLOAD_DESCRIPTOR_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::default::Default for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_THUNK_DATA32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_THUNK_DATA32_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_THUNK_DATA64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_THUNK_DATA64_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROA").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMEPROA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROA {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEPROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEPROW").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMEPROW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEPROW {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEPROW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRUCT").field("fnc", &self.fnc).field("wParam", &self.wParam).field("wCount", &self.wCount).field("dchSource", &self.dchSource).field("dchDest", &self.dchDest).field("lParam1", &self.lParam1).field("lParam2", &self.lParam2).field("lParam3", &self.lParam3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMESTRUCT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fnc == other.fnc && self.wParam == other.wParam && self.wCount == other.wCount && self.dchSource == other.dchSource && self.dchDest == other.dchDest && self.lParam1 == other.lParam1 && self.lParam2 == other.lParam2 && self.lParam3 == other.lParam3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IO_STATUS_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IO_STATUS_BLOCK_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: super::super::Foundation::BOOL,
    pub fAllPermissions: super::super::Foundation::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: ::windows::core::PCWSTR,
    pub guidZone: ::windows::core::GUID,
    pub hVerify: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JAVA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JAVA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JAVA_TRUST")
            .field("cbSize", &self.cbSize)
            .field("flag", &self.flag)
            .field("fAllActiveXPermissions", &self.fAllActiveXPermissions)
            .field("fAllPermissions", &self.fAllPermissions)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("pbJavaPermissions", &self.pbJavaPermissions)
            .field("cbJavaPermissions", &self.cbJavaPermissions)
            .field("pbSigner", &self.pbSigner)
            .field("cbSigner", &self.cbSigner)
            .field("pwszZone", &self.pwszZone)
            .field("guidZone", &self.guidZone)
            .field("hVerify", &self.hVerify)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for JAVA_TRUST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JAVA_TRUST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flag == other.flag && self.fAllActiveXPermissions == other.fAllActiveXPermissions && self.fAllPermissions == other.fAllPermissions && self.dwEncodingType == other.dwEncodingType && self.pbJavaPermissions == other.pbJavaPermissions && self.cbJavaPermissions == other.cbJavaPermissions && self.pbSigner == other.pbSigner && self.cbSigner == other.cbSigner && self.pwszZone == other.pwszZone && self.guidZone == other.guidZone && self.hVerify == other.hVerify
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
impl ::core::marker::Copy for JIT_DEBUG_INFO {}
impl ::core::clone::Clone for JIT_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JIT_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JIT_DEBUG_INFO").field("dwSize", &self.dwSize).field("dwProcessorArchitecture", &self.dwProcessorArchitecture).field("dwThreadID", &self.dwThreadID).field("dwReserved0", &self.dwReserved0).field("lpExceptionAddress", &self.lpExceptionAddress).field("lpExceptionRecord", &self.lpExceptionRecord).field("lpContextRecord", &self.lpContextRecord).finish()
    }
}
impl ::windows::core::TypeKind for JIT_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for JIT_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwProcessorArchitecture == other.dwProcessorArchitecture && self.dwThreadID == other.dwThreadID && self.dwReserved0 == other.dwReserved0 && self.lpExceptionAddress == other.lpExceptionAddress && self.lpExceptionRecord == other.lpExceptionRecord && self.lpContextRecord == other.lpContextRecord
    }
}
impl ::core::cmp::Eq for JIT_DEBUG_INFO {}
impl ::core::default::Default for JIT_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_VALUE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_VALUE_ENTRY").field("ValueName", &self.ValueName).field("DataLength", &self.DataLength).field("DataOffset", &self.DataOffset).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for KEY_VALUE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ValueName == other.ValueName && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset && self.Type == other.Type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub DllBase: *mut ::core::ffi::c_void,
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut ::core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for LDR_DATA_TABLE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for LDR_DATA_TABLE_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub ObjectName: *mut super::super::Foundation::UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub SecurityQualityOfService: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OBJECT_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONA {
    pub szGUID: [u8; 59],
    pub szDispName: [u8; 128],
    pub szLocale: [u8; 10],
    pub szStub: [u8; 1040],
    pub szVersion: [u8; 32],
    pub szCompID: [u8; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONA").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PERUSERSECTIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERUSERSECTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERUSERSECTIONW").field("szGUID", &self.szGUID).field("szDispName", &self.szDispName).field("szLocale", &self.szLocale).field("szStub", &self.szStub).field("szVersion", &self.szVersion).field("szCompID", &self.szCompID).field("dwIsInstalled", &self.dwIsInstalled).field("bRollback", &self.bRollback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PERUSERSECTIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERUSERSECTIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION").field("Attributes", &self.Attributes).field("GrantedAccess", &self.GrantedAccess).field("HandleCount", &self.HandleCount).field("PointerCount", &self.PointerCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for PUBLIC_OBJECT_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes && self.GrantedAccess == other.GrantedAccess && self.HandleCount == other.HandleCount && self.PointerCount == other.PointerCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::default::Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION").field("TypeName", &self.TypeName).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PUBLIC_OBJECT_TYPE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TypeName == other.TypeName && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRENTRYA {
    pub pszName: ::windows::core::PSTR,
    pub pszValue: ::windows::core::PSTR,
}
impl ::core::marker::Copy for STRENTRYA {}
impl ::core::clone::Clone for STRENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYA").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
impl ::windows::core::TypeKind for STRENTRYA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYA {}
impl ::core::default::Default for STRENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRENTRYW {
    pub pszName: ::windows::core::PWSTR,
    pub pszValue: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for STRENTRYW {}
impl ::core::clone::Clone for STRENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRENTRYW").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
impl ::windows::core::TypeKind for STRENTRYW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
impl ::core::cmp::Eq for STRENTRYW {}
impl ::core::default::Default for STRENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for STRINGEXSTRUCT {}
impl ::core::clone::Clone for STRINGEXSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRINGEXSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRINGEXSTRUCT").field("dwSize", &self.dwSize).field("uDeterminePos", &self.uDeterminePos).field("uDetermineDelimPos", &self.uDetermineDelimPos).field("uYomiPos", &self.uYomiPos).field("uYomiDelimPos", &self.uYomiDelimPos).finish()
    }
}
impl ::windows::core::TypeKind for STRINGEXSTRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRINGEXSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uDeterminePos == other.uDeterminePos && self.uDetermineDelimPos == other.uDetermineDelimPos && self.uYomiPos == other.uYomiPos && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for STRINGEXSTRUCT {}
impl ::core::default::Default for STRINGEXSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
impl ::core::marker::Copy for STRTABLEA {}
impl ::core::clone::Clone for STRTABLEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEA").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
impl ::windows::core::TypeKind for STRTABLEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRTABLEA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEA {}
impl ::core::default::Default for STRTABLEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
impl ::core::marker::Copy for STRTABLEW {}
impl ::core::clone::Clone for STRTABLEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRTABLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRTABLEW").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
impl ::windows::core::TypeKind for STRTABLEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRTABLEW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
impl ::core::cmp::Eq for STRTABLEW {}
impl ::core::default::Default for STRTABLEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut ::core::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl ::core::marker::Copy for SYSTEM_BASIC_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("NumberOfProcessors", &self.NumberOfProcessors).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.NumberOfProcessors == other.NumberOfProcessors
    }
}
impl ::core::cmp::Eq for SYSTEM_BASIC_INFORMATION {}
impl ::core::default::Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
impl ::core::marker::Copy for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CODEINTEGRITY_INFORMATION").field("Length", &self.Length).field("CodeIntegrityOptions", &self.CodeIntegrityOptions).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_CODEINTEGRITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CodeIntegrityOptions == other.CodeIntegrityOptions
    }
}
impl ::core::cmp::Eq for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::default::Default for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl ::core::marker::Copy for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_EXCEPTION_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_EXCEPTION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::default::Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl ::core::marker::Copy for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_INTERRUPT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_INTERRUPT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INTERRUPT_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_INTERRUPT_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_INTERRUPT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::default::Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl ::core::marker::Copy for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOOKASIDE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_LOOKASIDE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOOKASIDE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_LOOKASIDE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_LOOKASIDE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::default::Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl ::core::marker::Copy for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PERFORMANCE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_PERFORMANCE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::default::Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl ::core::marker::Copy for SYSTEM_POLICY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POLICY_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_POLICY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_POLICY_INFORMATION {}
impl ::core::default::Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION").field("IdleTime", &self.IdleTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IdleTime == other.IdleTime && self.KernelTime == other.KernelTime && self.UserTime == other.UserTime && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::default::Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut ::core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESS_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("Reserved1", &self.Reserved1)
            .field("ImageName", &self.ImageName)
            .field("BasePriority", &self.BasePriority)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved2", &self.Reserved2)
            .field("HandleCount", &self.HandleCount)
            .field("SessionId", &self.SessionId)
            .field("Reserved3", &self.Reserved3)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("Reserved4", &self.Reserved4)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("Reserved5", &self.Reserved5)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("Reserved6", &self.Reserved6)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivatePageCount", &self.PrivatePageCount)
            .field("Reserved7", &self.Reserved7)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SYSTEM_PROCESS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.NumberOfThreads == other.NumberOfThreads
            && self.Reserved1 == other.Reserved1
            && self.ImageName == other.ImageName
            && self.BasePriority == other.BasePriority
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved2 == other.Reserved2
            && self.HandleCount == other.HandleCount
            && self.SessionId == other.SessionId
            && self.Reserved3 == other.Reserved3
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.Reserved4 == other.Reserved4
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.Reserved5 == other.Reserved5
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.Reserved6 == other.Reserved6
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivatePageCount == other.PrivatePageCount
            && self.Reserved7 == other.Reserved7
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_REGISTRY_QUOTA_INFORMATION").field("RegistryQuotaAllowed", &self.RegistryQuotaAllowed).field("RegistryQuotaUsed", &self.RegistryQuotaUsed).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegistryQuotaAllowed == other.RegistryQuotaAllowed && self.RegistryQuotaUsed == other.RegistryQuotaUsed && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::default::Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut ::core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_THREAD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_THREAD_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("StartAddress", &self.StartAddress).field("ClientId", &self.ClientId).field("Priority", &self.Priority).field("BasePriority", &self.BasePriority).field("Reserved3", &self.Reserved3).field("ThreadState", &self.ThreadState).field("WaitReason", &self.WaitReason).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SYSTEM_THREAD_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.StartAddress == other.StartAddress && self.ClientId == other.ClientId && self.Priority == other.Priority && self.BasePriority == other.BasePriority && self.Reserved3 == other.Reserved3 && self.ThreadState == other.ThreadState && self.WaitReason == other.WaitReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl ::core::marker::Copy for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_TIMEOFDAY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYSTEM_TIMEOFDAY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_TIMEOFDAY_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_TIMEOFDAY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_TIMEOFDAY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::default::Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX32_XP").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX32_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl ::core::marker::Copy for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {}
impl ::core::clone::Clone for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX_W2K").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::windows::core::TypeKind for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {}
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX_W2K {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    pub ID: TDIObjectID,
    pub Context: [usize; 4],
}
impl ::core::marker::Copy for TCP_REQUEST_QUERY_INFORMATION_EX_XP {}
impl ::core::clone::Clone for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_QUERY_INFORMATION_EX_XP").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::windows::core::TypeKind for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_QUERY_INFORMATION_EX_XP {}
impl ::core::default::Default for TCP_REQUEST_QUERY_INFORMATION_EX_XP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TCP_REQUEST_SET_INFORMATION_EX {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for TCP_REQUEST_SET_INFORMATION_EX {}
impl ::core::clone::Clone for TCP_REQUEST_SET_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCP_REQUEST_SET_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCP_REQUEST_SET_INFORMATION_EX").field("ID", &self.ID).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for TCP_REQUEST_SET_INFORMATION_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TCP_REQUEST_SET_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for TCP_REQUEST_SET_INFORMATION_EX {}
impl ::core::default::Default for TCP_REQUEST_SET_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
impl ::core::marker::Copy for TDIEntityID {}
impl ::core::clone::Clone for TDIEntityID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIEntityID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIEntityID").field("tei_entity", &self.tei_entity).field("tei_instance", &self.tei_instance).finish()
    }
}
impl ::windows::core::TypeKind for TDIEntityID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TDIEntityID {
    fn eq(&self, other: &Self) -> bool {
        self.tei_entity == other.tei_entity && self.tei_instance == other.tei_instance
    }
}
impl ::core::cmp::Eq for TDIEntityID {}
impl ::core::default::Default for TDIEntityID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
impl ::core::marker::Copy for TDIObjectID {}
impl ::core::clone::Clone for TDIObjectID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TDIObjectID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TDIObjectID").field("toi_entity", &self.toi_entity).field("toi_class", &self.toi_class).field("toi_type", &self.toi_type).field("toi_id", &self.toi_id).finish()
    }
}
impl ::windows::core::TypeKind for TDIObjectID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TDIObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.toi_entity == other.toi_entity && self.toi_class == other.toi_class && self.toi_type == other.toi_type && self.toi_id == other.toi_id
    }
}
impl ::core::cmp::Eq for TDIObjectID {}
impl ::core::default::Default for TDIObjectID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut ::core::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut ::core::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TDI_TL_IO_CONTROL_ENDPOINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for THREAD_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for THREAD_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_NAME_INFORMATION").field("ThreadName", &self.ThreadName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for THREAD_NAME_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for THREAD_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadName == other.ThreadName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for THREAD_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for UNDETERMINESTRUCT {}
impl ::core::clone::Clone for UNDETERMINESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNDETERMINESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNDETERMINESTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDefIMESize", &self.uDefIMESize)
            .field("uDefIMEPos", &self.uDefIMEPos)
            .field("uUndetTextLen", &self.uUndetTextLen)
            .field("uUndetTextPos", &self.uUndetTextPos)
            .field("uUndetAttrPos", &self.uUndetAttrPos)
            .field("uCursorPos", &self.uCursorPos)
            .field("uDeltaStart", &self.uDeltaStart)
            .field("uDetermineTextLen", &self.uDetermineTextLen)
            .field("uDetermineTextPos", &self.uDetermineTextPos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiTextLen", &self.uYomiTextLen)
            .field("uYomiTextPos", &self.uYomiTextPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
impl ::windows::core::TypeKind for UNDETERMINESTRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UNDETERMINESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uDefIMESize == other.uDefIMESize && self.uDefIMEPos == other.uDefIMEPos && self.uUndetTextLen == other.uUndetTextLen && self.uUndetTextPos == other.uUndetTextPos && self.uUndetAttrPos == other.uUndetAttrPos && self.uCursorPos == other.uCursorPos && self.uDeltaStart == other.uDeltaStart && self.uDetermineTextLen == other.uDetermineTextLen && self.uDetermineTextPos == other.uDetermineTextPos && self.uDetermineDelimPos == other.uDetermineDelimPos && self.uYomiTextLen == other.uYomiTextLen && self.uYomiTextPos == other.uYomiTextPos && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::core::cmp::Eq for UNDETERMINESTRUCT {}
impl ::core::default::Default for UNDETERMINESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl ::core::marker::Copy for WINSTATIONINFORMATIONW {}
impl ::core::clone::Clone for WINSTATIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINSTATIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINSTATIONINFORMATIONW").field("Reserved2", &self.Reserved2).field("LogonId", &self.LogonId).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::windows::core::TypeKind for WINSTATIONINFORMATIONW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WINSTATIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved2 == other.Reserved2 && self.LogonId == other.LogonId && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for WINSTATIONINFORMATIONW {}
impl ::core::default::Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::clone::Clone for WLDP_DEVICE_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLDP_DEVICE_SECURITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_DEVICE_SECURITY_INFORMATION").field("UnlockIdSize", &self.UnlockIdSize).field("UnlockId", &self.UnlockId).field("ManufacturerIDLength", &self.ManufacturerIDLength).field("ManufacturerID", &self.ManufacturerID).finish()
    }
}
impl ::windows::core::TypeKind for WLDP_DEVICE_SECURITY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLDP_DEVICE_SECURITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.UnlockIdSize == other.UnlockIdSize && self.UnlockId == other.UnlockId && self.ManufacturerIDLength == other.ManufacturerIDLength && self.ManufacturerID == other.ManufacturerID
    }
}
impl ::core::cmp::Eq for WLDP_DEVICE_SECURITY_INFORMATION {}
impl ::core::default::Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: ::windows::core::PCWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLDP_HOST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLDP_HOST_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLDP_HOST_INFORMATION").field("dwRevision", &self.dwRevision).field("dwHostId", &self.dwHostId).field("szSource", &self.szSource).field("hSource", &self.hSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLDP_HOST_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLDP_HOST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwRevision == other.dwRevision && self.dwHostId == other.dwHostId && self.szSource == other.szSource && self.hSource == other.hSource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLDP_HOST_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _D3DHAL_CALLBACKS(pub u8);
#[repr(C)]
pub struct _D3DHAL_GLOBALDRIVERDATA(pub u8);
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type APPLICATION_RECOVERY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type ENUM_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PFEATURE_STATE_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PFIBER_CALLOUT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpparameter: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIO_APC_ROUTINE = ::core::option::Option<unsafe extern "system" fn(apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PQUERYACTCTXW_FUNC = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINSTATIONQUERYINFORMATIONW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut ::core::ffi::c_void, param4: u32, param5: *mut u32) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_CANEXECUTEBUFFER_API = ::core::option::Option<unsafe extern "system" fn(host: *const ::windows::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, buffer: *const u8, buffersize: u32, auditinfo: ::windows::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_CANEXECUTEFILE_API = ::core::option::Option<unsafe extern "system" fn(host: *const ::windows::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, filehandle: super::super::Foundation::HANDLE, auditinfo: ::windows::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type PWLDP_CANEXECUTESTREAM_API = ::core::option::Option<unsafe extern "system" fn(host: *const ::windows::core::GUID, options: WLDP_EXECUTION_EVALUATION_OPTIONS, stream: ::core::option::Option<super::Com::IStream>, auditinfo: ::windows::core::PCWSTR, result: *mut WLDP_EXECUTION_POLICY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = ::core::option::Option<unsafe extern "system" fn(packagefamilyname: ::windows::core::PCWSTR, packageversion: u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = ::core::option::Option<unsafe extern "system" fn(pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = ::core::option::Option<unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDYNAMICODETRUST_API = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = ::core::option::Option<unsafe extern "system" fn(setting: ::windows::core::PCWSTR, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = ::core::option::Option<unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = ::core::option::Option<unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_SETDYNAMICCODETRUST_API = ::core::option::Option<unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`*"]
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type REGINSTALLA = ::core::option::Option<unsafe extern "system" fn(hm: super::super::Foundation::HMODULE, pszsection: ::windows::core::PCSTR, psttable: *const STRTABLEA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_WindowsProgramming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WINWATCHNOTIFYPROC = ::core::option::Option<unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
