#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmAssociateContext(param0: super::windef::HWND, param1: HIMC) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::windef::HWND, param1 : HIMC) -> HIMC);
    unsafe { ImmAssociateContext(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmAssociateContextEx(param0: super::windef::HWND, param1: HIMC, param2: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::windef::HWND, param1 : HIMC, param2 : u32) -> windows_core::BOOL);
    unsafe { ImmAssociateContextEx(param0, param1, param2) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmConfigureIMEA(param0: super::minwindef::HKL, param1: super::windef::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::minwindef::HKL, param1 : super::windef::HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImmConfigureIMEA(param0, param1, param2, param3 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmConfigureIMEW(param0: super::minwindef::HKL, param1: super::windef::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::minwindef::HKL, param1 : super::windef::HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImmConfigureIMEW(param0, param1, param2, param3 as _) }
}
#[inline]
pub unsafe fn ImmCreateContext() -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmCreateContext() -> HIMC);
    unsafe { ImmCreateContext() }
}
#[inline]
pub unsafe fn ImmDestroyContext(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmDestroyContext(param0) }
}
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableIME(param0 : u32) -> windows_core::BOOL);
    unsafe { ImmDisableIME(param0) }
}
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableLegacyIME() -> windows_core::BOOL);
    unsafe { ImmDisableLegacyIME() }
}
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableTextFrameService(idthread : u32) -> windows_core::BOOL);
    unsafe { ImmDisableTextFrameService(idthread) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEnumInputContext(idthread: u32, lpfn: IMCENUMPROC, lparam: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : u32, lpfn : IMCENUMPROC, lparam : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { ImmEnumInputContext(idthread, lpfn, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<P2, P4>(param0: super::minwindef::HKL, param1: REGISTERWORDENUMPROCA, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::minwindef::HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : windows_core::PCSTR, param3 : u32, lpszregister : windows_core::PCSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordA(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<P2, P4>(param0: super::minwindef::HKL, param1: REGISTERWORDENUMPROCW, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::minwindef::HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : windows_core::PCWSTR, param3 : u32, lpszregister : windows_core::PCWSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordW(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEscapeA(param0: super::minwindef::HKL, param1: HIMC, param2: u32, param3: *mut core::ffi::c_void) -> super::minwindef::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::minwindef::HKL, param1 : HIMC, param2 : u32, param3 : *mut core::ffi::c_void) -> super::minwindef::LRESULT);
    unsafe { ImmEscapeA(param0, param1, param2, param3 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEscapeW(param0: super::minwindef::HKL, param1: HIMC, param2: u32, param3: *mut core::ffi::c_void) -> super::minwindef::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::minwindef::HKL, param1 : HIMC, param2 : u32, param3 : *mut core::ffi::c_void) -> super::minwindef::LRESULT);
    unsafe { ImmEscapeW(param0, param1, param2, param3 as _) }
}
#[inline]
pub unsafe fn ImmGetCandidateListA(param0: HIMC, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListA(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    unsafe { ImmGetCandidateListA(param0, deindex, lpcandlist.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCandidateListCountA(param0: HIMC, lpdwlistcount: *mut u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListCountA(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    unsafe { ImmGetCandidateListCountA(param0, lpdwlistcount as _) }
}
#[inline]
pub unsafe fn ImmGetCandidateListCountW(param0: HIMC, lpdwlistcount: *mut u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListCountW(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    unsafe { ImmGetCandidateListCountW(param0, lpdwlistcount as _) }
}
#[inline]
pub unsafe fn ImmGetCandidateListW(param0: HIMC, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListW(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    unsafe { ImmGetCandidateListW(param0, deindex, lpcandlist.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetCandidateWindow(param0: HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateWindow(param0 : HIMC, param1 : u32, lpcandidate : *mut CANDIDATEFORM) -> windows_core::BOOL);
    unsafe { ImmGetCandidateWindow(param0, param1, lpcandidate as _) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontA(param0: HIMC, lplf: *mut super::wingdi::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : HIMC, lplf : *mut super::wingdi::LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmGetCompositionFontA(param0, lplf as _) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontW(param0: HIMC, lplf: *mut super::wingdi::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : HIMC, lplf : *mut super::wingdi::LOGFONTW) -> windows_core::BOOL);
    unsafe { ImmGetCompositionFontW(param0, lplf as _) }
}
#[inline]
pub unsafe fn ImmGetCompositionStringA(param0: HIMC, param1: u32, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionStringA(param0 : HIMC, param1 : u32, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    unsafe { ImmGetCompositionStringA(param0, param1, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCompositionStringW(param0: HIMC, param1: u32, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionStringW(param0 : HIMC, param1 : u32, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    unsafe { ImmGetCompositionStringW(param0, param1, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetCompositionWindow(param0: HIMC, lpcompform: *mut COMPOSITIONFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionWindow(param0 : HIMC, lpcompform : *mut COMPOSITIONFORM) -> windows_core::BOOL);
    unsafe { ImmGetCompositionWindow(param0, lpcompform as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetContext(param0: super::windef::HWND) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::windef::HWND) -> HIMC);
    unsafe { ImmGetContext(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetConversionListA<P2>(param0: super::minwindef::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::minwindef::HKL, param1 : HIMC, lpsrc : windows_core::PCSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : u32) -> u32);
    unsafe { ImmGetConversionListA(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetConversionListW<P2>(param0: super::minwindef::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::minwindef::HKL, param1 : HIMC, lpsrc : windows_core::PCWSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : u32) -> u32);
    unsafe { ImmGetConversionListW(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[inline]
pub unsafe fn ImmGetConversionStatus(param0: HIMC, lpfdwconversion: Option<*mut u32>, lpfdwsentence: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : HIMC, lpfdwconversion : *mut u32, lpfdwsentence : *mut u32) -> windows_core::BOOL);
    unsafe { ImmGetConversionStatus(param0, lpfdwconversion.unwrap_or(core::mem::zeroed()) as _, lpfdwsentence.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd(param0: super::windef::HWND) -> super::windef::HWND {
    windows_core::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::windef::HWND) -> super::windef::HWND);
    unsafe { ImmGetDefaultIMEWnd(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetDescriptionA(param0: super::minwindef::HKL, lpszdescription: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::minwindef::HKL, lpszdescription : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionA(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetDescriptionW(param0: super::minwindef::HKL, lpszdescription: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::minwindef::HKL, lpszdescription : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionW(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineA(param0: HIMC, dwindex: u32, lpbuf: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : HIMC, dwindex : u32, lpbuf : windows_core::PSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineA(param0, dwindex, core::mem::transmute(lpbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineW(param0: HIMC, dwindex: u32, lpbuf: Option<windows_core::PWSTR>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : HIMC, dwindex : u32, lpbuf : windows_core::PWSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineW(param0, dwindex, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA(param0: super::minwindef::HKL, lpszfilename: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::minwindef::HKL, lpszfilename : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameA(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW(param0: super::minwindef::HKL, lpszfilename: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::minwindef::HKL, lpszfilename : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameW(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA(param0: HIMC, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOA>, lpimemenu: Option<*mut IMEMENUITEMINFOA>, dwsize: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetImeMenuItemsA(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOA, lpimemenu : *mut IMEMENUITEMINFOA, dwsize : u32) -> u32);
    unsafe { ImmGetImeMenuItemsA(param0, param1, param2, lpimeparentmenu.unwrap_or(core::mem::zeroed()) as _, lpimemenu.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW(param0: HIMC, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOW>, lpimemenu: Option<*mut IMEMENUITEMINFOW>, dwsize: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetImeMenuItemsW(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOW, lpimemenu : *mut IMEMENUITEMINFOW, dwsize : u32) -> u32);
    unsafe { ImmGetImeMenuItemsW(param0, param1, param2, lpimeparentmenu.unwrap_or(core::mem::zeroed()) as _, lpimemenu.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[inline]
pub unsafe fn ImmGetOpenStatus(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetOpenStatus(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmGetOpenStatus(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetProperty(param0: super::minwindef::HKL, param1: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::minwindef::HKL, param1 : u32) -> u32);
    unsafe { ImmGetProperty(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA(param0: super::minwindef::HKL, lpstylebuf: &mut [STYLEBUFA]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::minwindef::HKL, nitem : u32, lpstylebuf : *mut STYLEBUFA) -> u32);
    unsafe { ImmGetRegisterWordStyleA(param0, lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW(param0: super::minwindef::HKL, lpstylebuf: &mut [STYLEBUFW]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::minwindef::HKL, nitem : u32, lpstylebuf : *mut STYLEBUFW) -> u32);
    unsafe { ImmGetRegisterWordStyleW(param0, lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr())) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetStatusWindowPos(param0: HIMC, lpptpos: *mut super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : HIMC, lpptpos : *mut super::windef::POINT) -> windows_core::BOOL);
    unsafe { ImmGetStatusWindowPos(param0, lpptpos as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetVirtualKey(param0: super::windef::HWND) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::windef::HWND) -> u32);
    unsafe { ImmGetVirtualKey(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmInstallIMEA<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::minwindef::HKL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : windows_core::PCSTR, lpszlayouttext : windows_core::PCSTR) -> super::minwindef::HKL);
    unsafe { ImmInstallIMEA(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmInstallIMEW<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::minwindef::HKL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : windows_core::PCWSTR, lpszlayouttext : windows_core::PCWSTR) -> super::minwindef::HKL);
    unsafe { ImmInstallIMEW(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmIsIME(param0: super::minwindef::HKL) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::minwindef::HKL) -> windows_core::BOOL);
    unsafe { ImmIsIME(param0) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmIsUIMessageA(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::windef::HWND, param1 : u32, param2 : super::minwindef::WPARAM, param3 : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageA(param0, param1, param2, param3) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmIsUIMessageW(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::windef::HWND, param1 : u32, param2 : super::minwindef::WPARAM, param3 : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageW(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn ImmNotifyIME(param0: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : HIMC, dwaction : u32, dwindex : u32, dwvalue : u32) -> windows_core::BOOL);
    unsafe { ImmNotifyIME(param0, dwaction, dwindex, dwvalue) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmRegisterWordA<P1, P3>(param0: super::minwindef::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::minwindef::HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordA(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmRegisterWordW<P1, P3>(param0: super::minwindef::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::minwindef::HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordW(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmReleaseContext(param0: super::windef::HWND, param1: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::windef::HWND, param1 : HIMC) -> windows_core::BOOL);
    unsafe { ImmReleaseContext(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmSetCandidateWindow(param0: HIMC, lpcandidate: *const CANDIDATEFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCandidateWindow(param0 : HIMC, lpcandidate : *const CANDIDATEFORM) -> windows_core::BOOL);
    unsafe { ImmSetCandidateWindow(param0, lpcandidate) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontA(param0: HIMC, lplf: *const super::wingdi::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : HIMC, lplf : *const super::wingdi::LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmSetCompositionFontA(param0, lplf) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontW(param0: HIMC, lplf: *const super::wingdi::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : HIMC, lplf : *const super::wingdi::LOGFONTW) -> windows_core::BOOL);
    unsafe { ImmSetCompositionFontW(param0, lplf) }
}
#[inline]
pub unsafe fn ImmSetCompositionStringA(param0: HIMC, dwindex: u32, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionStringA(param0 : HIMC, dwindex : u32, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> windows_core::BOOL);
    unsafe { ImmSetCompositionStringA(param0, dwindex, lpcomp.unwrap_or(core::mem::zeroed()) as _, dwcomplen, lpread.unwrap_or(core::mem::zeroed()) as _, dwreadlen) }
}
#[inline]
pub unsafe fn ImmSetCompositionStringW(param0: HIMC, dwindex: u32, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionStringW(param0 : HIMC, dwindex : u32, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> windows_core::BOOL);
    unsafe { ImmSetCompositionStringW(param0, dwindex, lpcomp.unwrap_or(core::mem::zeroed()) as _, dwcomplen, lpread.unwrap_or(core::mem::zeroed()) as _, dwreadlen) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmSetCompositionWindow(param0: HIMC, lpcompform: *const COMPOSITIONFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionWindow(param0 : HIMC, lpcompform : *const COMPOSITIONFORM) -> windows_core::BOOL);
    unsafe { ImmSetCompositionWindow(param0, lpcompform) }
}
#[inline]
pub unsafe fn ImmSetConversionStatus(param0: HIMC, param1: u32, param2: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetConversionStatus(param0 : HIMC, param1 : u32, param2 : u32) -> windows_core::BOOL);
    unsafe { ImmSetConversionStatus(param0, param1, param2) }
}
#[inline]
pub unsafe fn ImmSetOpenStatus(param0: HIMC, param1: bool) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetOpenStatus(param0 : HIMC, param1 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ImmSetOpenStatus(param0, param1.into()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmSetStatusWindowPos(param0: HIMC, lpptpos: *const super::windef::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : HIMC, lpptpos : *const super::windef::POINT) -> windows_core::BOOL);
    unsafe { ImmSetStatusWindowPos(param0, lpptpos) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmSimulateHotKey(param0: super::windef::HWND, param1: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::windef::HWND, param1 : u32) -> windows_core::BOOL);
    unsafe { ImmSimulateHotKey(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmUnregisterWordA<P1, P3>(param0: super::minwindef::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::minwindef::HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszunregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordA(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmUnregisterWordW<P1, P3>(param0: super::minwindef::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::minwindef::HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszunregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordW(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
pub const ATTR_CONVERTED: u32 = 2;
pub const ATTR_FIXEDCONVERTED: u32 = 5;
pub const ATTR_INPUT: u32 = 0;
pub const ATTR_INPUT_ERROR: u32 = 4;
pub const ATTR_TARGET_CONVERTED: u32 = 1;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::windef::POINT,
    pub rcArea: super::windef::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CFS_CANDIDATEPOS: u32 = 64;
pub const CFS_DEFAULT: u32 = 0;
pub const CFS_EXCLUDE: u32 = 128;
pub const CFS_FORCE_POSITION: u32 = 32;
pub const CFS_POINT: u32 = 2;
pub const CFS_RECT: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::windef::POINT,
    pub rcArea: super::windef::RECT,
}
pub const CPS_CANCEL: u32 = 4;
pub const CPS_COMPLETE: u32 = 1;
pub const CPS_CONVERT: u32 = 2;
pub const CPS_REVERT: u32 = 3;
pub const CS_INSERTCHAR: u32 = 8192;
pub const CS_NOMOVECARET: u32 = 16384;
pub const GCL_CONVERSION: u32 = 1;
pub const GCL_REVERSECONVERSION: u32 = 2;
pub const GCL_REVERSE_LENGTH: u32 = 3;
pub const GCS_COMPATTR: u32 = 16;
pub const GCS_COMPCLAUSE: u32 = 32;
pub const GCS_COMPREADATTR: u32 = 2;
pub const GCS_COMPREADCLAUSE: u32 = 4;
pub const GCS_COMPREADSTR: u32 = 1;
pub const GCS_COMPSTR: u32 = 8;
pub const GCS_CURSORPOS: u32 = 128;
pub const GCS_DELTASTART: u32 = 256;
pub const GCS_RESULTCLAUSE: u32 = 4096;
pub const GCS_RESULTREADCLAUSE: u32 = 1024;
pub const GCS_RESULTREADSTR: u32 = 512;
pub const GCS_RESULTSTR: u32 = 2048;
pub const GGL_INDEX: u32 = 2;
pub const GGL_LEVEL: u32 = 1;
pub const GGL_PRIVATE: u32 = 4;
pub const GGL_STRING: u32 = 3;
pub const GL_ID_CANNOTSAVE: u32 = 17;
pub const GL_ID_CHOOSECANDIDATE: u32 = 40;
pub const GL_ID_INPUTCODE: u32 = 38;
pub const GL_ID_INPUTRADICAL: u32 = 37;
pub const GL_ID_INPUTREADING: u32 = 36;
pub const GL_ID_INPUTSYMBOL: u32 = 39;
pub const GL_ID_NOCONVERT: u32 = 32;
pub const GL_ID_NODICTIONARY: u32 = 16;
pub const GL_ID_NOMODULE: u32 = 1;
pub const GL_ID_PRIVATE_FIRST: u32 = 32768;
pub const GL_ID_PRIVATE_LAST: u32 = 65535;
pub const GL_ID_READINGCONFLICT: u32 = 35;
pub const GL_ID_REVERSECONVERSION: u32 = 41;
pub const GL_ID_TOOMANYSTROKE: u32 = 34;
pub const GL_ID_TYPINGERROR: u32 = 33;
pub const GL_ID_UNKNOWN: u32 = 0;
pub const GL_LEVEL_ERROR: u32 = 2;
pub const GL_LEVEL_FATAL: u32 = 1;
pub const GL_LEVEL_INFORMATION: u32 = 4;
pub const GL_LEVEL_NOGUIDELINE: u32 = 0;
pub const GL_LEVEL_WARNING: u32 = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMC(pub *mut core::ffi::c_void);
impl HIMC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HIMC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMCC(pub *mut core::ffi::c_void);
impl HIMCC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HIMCC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IACE_CHILDREN: u32 = 1;
pub const IACE_DEFAULT: u32 = 16;
pub const IACE_IGNORENOCONTEXT: u32 = 32;
pub const IGIMIF_RIGHTMENU: u32 = 1;
pub const IGIMII_CMODE: u32 = 1;
pub const IGIMII_CONFIGURE: u32 = 4;
pub const IGIMII_HELP: u32 = 16;
pub const IGIMII_INPUTTOOLS: u32 = 64;
pub const IGIMII_OTHER: u32 = 32;
pub const IGIMII_SMODE: u32 = 2;
pub const IGIMII_TOOLS: u32 = 8;
pub const IGP_CONVERSION: u32 = 8;
pub const IGP_GETIMEVERSION: i32 = -4;
pub const IGP_PROPERTY: u32 = 4;
pub const IGP_SELECT: u32 = 24;
pub const IGP_SENTENCE: u32 = 12;
pub const IGP_SETCOMPSTR: u32 = 20;
pub const IGP_UI: u32 = 16;
#[cfg(feature = "minwindef")]
pub type IMCENUMPROC = Option<unsafe extern "system" fn(param0: HIMC, param1: super::minwindef::LPARAM) -> windows_core::BOOL>;
pub const IMC_CLOSESTATUSWINDOW: u32 = 33;
pub const IMC_GETCANDIDATEPOS: u32 = 7;
pub const IMC_GETCOMPOSITIONFONT: u32 = 9;
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11;
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15;
pub const IMC_OPENSTATUSWINDOW: u32 = 34;
pub const IMC_SETCANDIDATEPOS: u32 = 8;
pub const IMC_SETCOMPOSITIONFONT: u32 = 10;
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12;
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::windef::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::windef::RECT,
}
#[cfg(feature = "windef")]
pub type IMEMENUITEMINFO = IMEMENUITEMINFOA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::windef::HBITMAP,
    pub hbmpUnchecked: super::windef::HBITMAP,
    pub dwItemData: u32,
    pub szString: [i8; 80],
    pub hbmpItem: super::windef::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::windef::HBITMAP,
    pub hbmpUnchecked: super::windef::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::windef::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEMENUITEM_STRING_SIZE: u32 = 80;
pub const IMEVER_0310: u32 = 196618;
pub const IMEVER_0400: u32 = 262144;
pub const IME_CAND_CODE: u32 = 2;
pub const IME_CAND_MEANING: u32 = 3;
pub const IME_CAND_RADICAL: u32 = 4;
pub const IME_CAND_READ: u32 = 1;
pub const IME_CAND_STROKE: u32 = 5;
pub const IME_CAND_UNKNOWN: u32 = 0;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: u32 = 16;
pub const IME_CHOTKEY_SHAPE_TOGGLE: u32 = 17;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: u32 = 18;
pub const IME_CMODE_EUDC: u32 = 512;
pub const IME_CMODE_FIXED: u32 = 2048;
pub const IME_CMODE_HANGEUL: u32 = 1;
pub const IME_CMODE_NOCONVERSION: u32 = 256;
pub const IME_CMODE_RESERVED: u32 = 4026531840;
pub const IME_CMODE_SOFTKBD: u32 = 128;
pub const IME_CMODE_SYMBOL: u32 = 1024;
pub const IME_CONFIG_GENERAL: u32 = 1;
pub const IME_CONFIG_REGISTERWORD: u32 = 2;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3;
pub const IME_ESC_AUTOMATA: u32 = 4105;
pub const IME_ESC_GETHELPFILENAME: u32 = 4107;
pub const IME_ESC_GET_EUDC_DICTIONARY: u32 = 4099;
pub const IME_ESC_HANJA_MODE: u32 = 4104;
pub const IME_ESC_IME_NAME: u32 = 4102;
pub const IME_ESC_MAX_KEY: u32 = 4101;
pub const IME_ESC_PRIVATE_FIRST: u32 = 2048;
pub const IME_ESC_PRIVATE_HOTKEY: u32 = 4106;
pub const IME_ESC_PRIVATE_LAST: u32 = 4095;
pub const IME_ESC_QUERY_SUPPORT: u32 = 3;
pub const IME_ESC_RESERVED_FIRST: u32 = 4;
pub const IME_ESC_RESERVED_LAST: u32 = 2047;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: u32 = 4097;
pub const IME_ESC_SET_EUDC_DICTIONARY: u32 = 4100;
pub const IME_ESC_SYNC_HOTKEY: u32 = 4103;
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256;
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287;
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512;
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: u32 = 513;
pub const IME_ITHOTKEY_RECONVERTSTRING: u32 = 515;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: u32 = 512;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: u32 = 514;
pub const IME_JHOTKEY_CLOSE_OPEN: u32 = 48;
pub const IME_KHOTKEY_ENGLISH: u32 = 82;
pub const IME_KHOTKEY_HANJACONVERT: u32 = 81;
pub const IME_KHOTKEY_SHAPE_TOGGLE: u32 = 80;
pub const IME_PROP_AT_CARET: u32 = 65536;
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144;
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576;
pub const IME_PROP_SPECIAL_UI: u32 = 131072;
pub const IME_PROP_UNICODE: u32 = 524288;
pub const IME_REGWORD_STYLE_EUDC: u32 = 1;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295;
pub const IME_SMODE_AUTOMATIC: u32 = 4;
pub const IME_SMODE_CONVERSATION: u32 = 16;
pub const IME_SMODE_NONE: u32 = 0;
pub const IME_SMODE_PHRASEPREDICT: u32 = 8;
pub const IME_SMODE_PLAURALCLAUSE: u32 = 1;
pub const IME_SMODE_RESERVED: u32 = 61440;
pub const IME_SMODE_SINGLECONVERT: u32 = 2;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: u32 = 112;
pub const IME_THOTKEY_SHAPE_TOGGLE: u32 = 113;
pub const IME_THOTKEY_SYMBOL_TOGGLE: u32 = 114;
pub const IMFS_CHECKED: u32 = 8;
pub const IMFS_DEFAULT: u32 = 4096;
pub const IMFS_DISABLED: u32 = 3;
pub const IMFS_ENABLED: u32 = 0;
pub const IMFS_GRAYED: u32 = 3;
pub const IMFS_HILITE: u32 = 128;
pub const IMFS_UNCHECKED: u32 = 0;
pub const IMFS_UNHILITE: u32 = 0;
pub const IMFT_RADIOCHECK: u32 = 1;
pub const IMFT_SEPARATOR: u32 = 2;
pub const IMFT_SUBMENU: u32 = 4;
pub const IMM_ERROR_GENERAL: i32 = -2;
pub const IMM_ERROR_NODATA: i32 = -1;
pub const IMN_CHANGECANDIDATE: u32 = 3;
pub const IMN_CLOSECANDIDATE: u32 = 4;
pub const IMN_CLOSESTATUSWINDOW: u32 = 1;
pub const IMN_GUIDELINE: u32 = 13;
pub const IMN_OPENCANDIDATE: u32 = 5;
pub const IMN_OPENSTATUSWINDOW: u32 = 2;
pub const IMN_PRIVATE: u32 = 14;
pub const IMN_SETCANDIDATEPOS: u32 = 9;
pub const IMN_SETCOMPOSITIONFONT: u32 = 10;
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11;
pub const IMN_SETCONVERSIONMODE: u32 = 6;
pub const IMN_SETOPENSTATUS: u32 = 8;
pub const IMN_SETSENTENCEMODE: u32 = 7;
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12;
pub const IMR_CANDIDATEWINDOW: u32 = 2;
pub const IMR_COMPOSITIONFONT: u32 = 3;
pub const IMR_COMPOSITIONWINDOW: u32 = 1;
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5;
pub const IMR_DOCUMENTFEED: u32 = 7;
pub const IMR_QUERYCHARPOSITION: u32 = 6;
pub const IMR_RECONVERTSTRING: u32 = 4;
pub const ISC_SHOWUIALL: u32 = 3221225487;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15;
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648;
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824;
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCANDIDATEFORM(pub *mut CANDIDATEFORM);
#[cfg(feature = "windef")]
impl LPCANDIDATEFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPCANDIDATEFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCANDIDATELIST(pub *mut CANDIDATELIST);
impl LPCANDIDATELIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOMPOSITIONFORM(pub *mut COMPOSITIONFORM);
#[cfg(feature = "windef")]
impl LPCOMPOSITIONFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPCOMPOSITIONFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHKL(pub *mut super::minwindef::HKL);
#[cfg(feature = "minwindef")]
impl LPHKL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for LPHKL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMECHARPOSITION(pub *mut IMECHARPOSITION);
#[cfg(feature = "windef")]
impl LPIMECHARPOSITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPIMECHARPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPIMEMENUITEMINFO(pub LPIMEMENUITEMINFOA);
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMEMENUITEMINFOA(pub *mut IMEMENUITEMINFOA);
#[cfg(feature = "windef")]
impl LPIMEMENUITEMINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPIMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMEMENUITEMINFOW(pub *mut IMEMENUITEMINFOW);
#[cfg(feature = "windef")]
impl LPIMEMENUITEMINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for LPIMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPRECONVERTSTRING(pub *mut RECONVERTSTRING);
impl LPRECONVERTSTRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPRECONVERTSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPREGISTERWORD(pub LPREGISTERWORDA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREGISTERWORDA(pub *mut REGISTERWORDA);
impl LPREGISTERWORDA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREGISTERWORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREGISTERWORDW(pub *mut REGISTERWORDW);
impl LPREGISTERWORDW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPREGISTERWORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPSTYLEBUF(pub LPSTYLEBUFA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTYLEBUFA(pub *mut STYLEBUFA);
impl LPSTYLEBUFA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSTYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTYLEBUFW(pub *mut STYLEBUFW);
impl LPSTYLEBUFW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSTYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024;
pub const MOD_LEFT: u32 = 32768;
pub const MOD_ON_KEYUP: u32 = 2048;
pub const MOD_RIGHT: u32 = 16384;
pub const NI_CHANGECANDIDATELIST: u32 = 19;
pub const NI_CLOSECANDIDATE: u32 = 17;
pub const NI_COMPOSITIONSTR: u32 = 21;
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20;
pub const NI_IMEMENUSELECTED: u32 = 24;
pub const NI_OPENCANDIDATE: u32 = 16;
pub const NI_SELECTCANDIDATESTR: u32 = 18;
pub const NI_SETCANDIDATE_PAGESIZE: u32 = 23;
pub const NI_SETCANDIDATE_PAGESTART: u32 = 22;
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCANDIDATEFORM(pub *mut CANDIDATEFORM);
#[cfg(feature = "windef")]
impl NPCANDIDATEFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for NPCANDIDATEFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCANDIDATELIST(pub *mut CANDIDATELIST);
impl NPCANDIDATELIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPCANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCOMPOSITIONFORM(pub *mut COMPOSITIONFORM);
#[cfg(feature = "windef")]
impl NPCOMPOSITIONFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for NPCOMPOSITIONFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPIMECHARPOSITION(pub *mut IMECHARPOSITION);
#[cfg(feature = "windef")]
impl NPIMECHARPOSITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for NPIMECHARPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPIMEMENUITEMINFO(pub NPIMEMENUITEMINFOA);
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPIMEMENUITEMINFOA(pub *mut IMEMENUITEMINFOA);
#[cfg(feature = "windef")]
impl NPIMEMENUITEMINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for NPIMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPIMEMENUITEMINFOW(pub *mut IMEMENUITEMINFOW);
#[cfg(feature = "windef")]
impl NPIMEMENUITEMINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for NPIMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPRECONVERTSTRING(pub *mut RECONVERTSTRING);
impl NPRECONVERTSTRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPRECONVERTSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPREGISTERWORD(pub NPREGISTERWORDA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPREGISTERWORDA(pub *mut REGISTERWORDA);
impl NPREGISTERWORDA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPREGISTERWORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPREGISTERWORDW(pub *mut REGISTERWORDW);
impl NPREGISTERWORDW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPREGISTERWORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NPSTYLEBUF(pub NPSTYLEBUFA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPSTYLEBUFA(pub *mut STYLEBUFA);
impl NPSTYLEBUFA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPSTYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPSTYLEBUFW(pub *mut STYLEBUFW);
impl NPSTYLEBUFW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPSTYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCANDIDATEFORM(pub *mut CANDIDATEFORM);
#[cfg(feature = "windef")]
impl PCANDIDATEFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PCANDIDATEFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCANDIDATELIST(pub *mut CANDIDATELIST);
impl PCANDIDATELIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMPOSITIONFORM(pub *mut COMPOSITIONFORM);
#[cfg(feature = "windef")]
impl PCOMPOSITIONFORM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PCOMPOSITIONFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMECHARPOSITION(pub *mut IMECHARPOSITION);
#[cfg(feature = "windef")]
impl PIMECHARPOSITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PIMECHARPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMEMENUITEMINFO(pub PIMEMENUITEMINFOA);
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMEMENUITEMINFOA(pub *mut IMEMENUITEMINFOA);
#[cfg(feature = "windef")]
impl PIMEMENUITEMINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PIMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMEMENUITEMINFOW(pub *mut IMEMENUITEMINFOW);
#[cfg(feature = "windef")]
impl PIMEMENUITEMINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PIMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRECONVERTSTRING(pub *mut RECONVERTSTRING);
impl PRECONVERTSTRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRECONVERTSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PREGISTERWORD(pub PREGISTERWORDA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREGISTERWORDA(pub *mut REGISTERWORDA);
impl PREGISTERWORDA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREGISTERWORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREGISTERWORDW(pub *mut REGISTERWORDW);
impl PREGISTERWORDW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREGISTERWORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSTYLEBUF(pub PSTYLEBUFA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSTYLEBUFA(pub *mut STYLEBUFA);
impl PSTYLEBUFA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSTYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSTYLEBUFW(pub *mut STYLEBUFW);
impl PSTYLEBUFW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSTYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECONVERTSTRING {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwTargetStrLen: u32,
    pub dwTargetStrOffset: u32,
}
pub type REGISTERWORD = REGISTERWORDA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REGISTERWORDA {
    pub lpReading: windows_core::PSTR,
    pub lpWord: windows_core::PSTR,
}
pub type REGISTERWORDENUMPROCA = Option<unsafe extern "system" fn(lpszreading: windows_core::PCSTR, param1: u32, lpszstring: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = Option<unsafe extern "system" fn(lpszreading: windows_core::PCWSTR, param1: u32, lpszstring: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REGISTERWORDW {
    pub lpReading: windows_core::PWSTR,
    pub lpWord: windows_core::PWSTR,
}
pub const SCS_CAP_COMPSTR: u32 = 1;
pub const SCS_CAP_MAKEREAD: u32 = 2;
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4;
pub const SCS_CHANGEATTR: u32 = 18;
pub const SCS_CHANGECLAUSE: u32 = 36;
pub const SCS_QUERYRECONVERTSTRING: u32 = 131072;
pub const SCS_SETRECONVERTSTRING: u32 = 65536;
pub const SCS_SETSTR: u32 = 9;
pub const SELECT_CAP_CONVERSION: u32 = 1;
pub const SELECT_CAP_SENTENCE: u32 = 2;
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1;
pub type STYLEBUF = STYLEBUFA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [i8; 32],
}
impl Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STYLE_DESCRIPTION_SIZE: u32 = 32;
pub const UI_CAP_2700: u32 = 1;
pub const UI_CAP_ROT90: u32 = 2;
pub const UI_CAP_ROTANY: u32 = 4;
