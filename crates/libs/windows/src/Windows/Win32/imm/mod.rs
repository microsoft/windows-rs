#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmAssociateContext(param0: super::HWND, param1: HIMC) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::HWND, param1 : HIMC) -> HIMC);
    unsafe { ImmAssociateContext(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmAssociateContextEx(param0: super::HWND, param1: HIMC, param2: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::HWND, param1 : HIMC, param2 : u32) -> windows_core::BOOL);
    unsafe { ImmAssociateContextEx(param0, param1, param2) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmConfigureIMEA(param0: super::HKL, param1: super::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::HKL, param1 : super::HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImmConfigureIMEA(param0, param1, param2, param3 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmConfigureIMEW(param0: super::HKL, param1: super::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::HKL, param1 : super::HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
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
pub unsafe fn ImmEnumInputContext(idthread: u32, lpfn: IMCENUMPROC, lparam: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : u32, lpfn : IMCENUMPROC, lparam : super::LPARAM) -> windows_core::BOOL);
    unsafe { ImmEnumInputContext(idthread, lpfn, lparam) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<P2, P4>(param0: super::HKL, param1: REGISTERWORDENUMPROCA, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : windows_core::PCSTR, param3 : u32, lpszregister : windows_core::PCSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordA(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<P2, P4>(param0: super::HKL, param1: REGISTERWORDENUMPROCW, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : windows_core::PCWSTR, param3 : u32, lpszregister : windows_core::PCWSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordW(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEscapeA(param0: super::HKL, param1: HIMC, param2: u32, param3: *mut core::ffi::c_void) -> super::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::HKL, param1 : HIMC, param2 : u32, param3 : *mut core::ffi::c_void) -> super::LRESULT);
    unsafe { ImmEscapeA(param0, param1, param2, param3 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmEscapeW(param0: super::HKL, param1: HIMC, param2: u32, param3: *mut core::ffi::c_void) -> super::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::HKL, param1 : HIMC, param2 : u32, param3 : *mut core::ffi::c_void) -> super::LRESULT);
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
pub unsafe fn ImmGetCompositionFontA(param0: HIMC, lplf: *mut super::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : HIMC, lplf : *mut super::LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmGetCompositionFontA(param0, lplf as _) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontW(param0: HIMC, lplf: *mut super::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : HIMC, lplf : *mut super::LOGFONTW) -> windows_core::BOOL);
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
pub unsafe fn ImmGetContext(param0: super::HWND) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::HWND) -> HIMC);
    unsafe { ImmGetContext(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetConversionListA<P2>(param0: super::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::HKL, param1 : HIMC, lpsrc : windows_core::PCSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : u32) -> u32);
    unsafe { ImmGetConversionListA(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetConversionListW<P2>(param0: super::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::HKL, param1 : HIMC, lpsrc : windows_core::PCWSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : u32) -> u32);
    unsafe { ImmGetConversionListW(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[inline]
pub unsafe fn ImmGetConversionStatus(param0: HIMC, lpfdwconversion: Option<*mut u32>, lpfdwsentence: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : HIMC, lpfdwconversion : *mut u32, lpfdwsentence : *mut u32) -> windows_core::BOOL);
    unsafe { ImmGetConversionStatus(param0, lpfdwconversion.unwrap_or(core::mem::zeroed()) as _, lpfdwsentence.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd(param0: super::HWND) -> super::HWND {
    windows_core::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::HWND) -> super::HWND);
    unsafe { ImmGetDefaultIMEWnd(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetDescriptionA(param0: super::HKL, lpszdescription: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::HKL, lpszdescription : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionA(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetDescriptionW(param0: super::HKL, lpszdescription: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::HKL, lpszdescription : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionW(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineA(param0: HIMC, dwindex: u32, lpbuf: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : HIMC, dwindex : u32, lpbuf : windows_core::PSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineA(param0, dwindex, core::mem::transmute(lpbuf.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineW(param0: HIMC, dwindex: u32, lpbuf: Option<windows_core::PWSTR>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : HIMC, dwindex : u32, lpbuf : windows_core::PWSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineW(param0, dwindex, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA(param0: super::HKL, lpszfilename: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::HKL, lpszfilename : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameA(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW(param0: super::HKL, lpszfilename: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::HKL, lpszfilename : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameW(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
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
pub unsafe fn ImmGetProperty(param0: super::HKL, param1: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::HKL, param1 : u32) -> u32);
    unsafe { ImmGetProperty(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA(param0: super::HKL, lpstylebuf: &mut [STYLEBUFA]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::HKL, nitem : u32, lpstylebuf : *mut STYLEBUFA) -> u32);
    unsafe { ImmGetRegisterWordStyleA(param0, lpstylebuf.len().try_into().unwrap(), lpstylebuf.as_mut_ptr()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW(param0: super::HKL, lpstylebuf: &mut [STYLEBUFW]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::HKL, nitem : u32, lpstylebuf : *mut STYLEBUFW) -> u32);
    unsafe { ImmGetRegisterWordStyleW(param0, lpstylebuf.len().try_into().unwrap(), lpstylebuf.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetStatusWindowPos(param0: HIMC, lpptpos: *mut super::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : HIMC, lpptpos : *mut super::POINT) -> windows_core::BOOL);
    unsafe { ImmGetStatusWindowPos(param0, lpptpos as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmGetVirtualKey(param0: super::HWND) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::HWND) -> u32);
    unsafe { ImmGetVirtualKey(param0) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmInstallIMEA<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::HKL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : windows_core::PCSTR, lpszlayouttext : windows_core::PCSTR) -> super::HKL);
    unsafe { ImmInstallIMEA(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmInstallIMEW<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::HKL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : windows_core::PCWSTR, lpszlayouttext : windows_core::PCWSTR) -> super::HKL);
    unsafe { ImmInstallIMEW(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmIsIME(param0: super::HKL) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::HKL) -> windows_core::BOOL);
    unsafe { ImmIsIME(param0) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmIsUIMessageA(param0: super::HWND, param1: u32, param2: super::WPARAM, param3: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::HWND, param1 : u32, param2 : super::WPARAM, param3 : super::LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageA(param0, param1, param2, param3) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ImmIsUIMessageW(param0: super::HWND, param1: u32, param2: super::WPARAM, param3: super::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::HWND, param1 : u32, param2 : super::WPARAM, param3 : super::LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageW(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn ImmNotifyIME(param0: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : HIMC, dwaction : u32, dwindex : u32, dwvalue : u32) -> windows_core::BOOL);
    unsafe { ImmNotifyIME(param0, dwaction, dwindex, dwvalue) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmRegisterWordA<P1, P3>(param0: super::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordA(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmRegisterWordW<P1, P3>(param0: super::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordW(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmReleaseContext(param0: super::HWND, param1: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::HWND, param1 : HIMC) -> windows_core::BOOL);
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
pub unsafe fn ImmSetCompositionFontA(param0: HIMC, lplf: *const super::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : HIMC, lplf : *const super::LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmSetCompositionFontA(param0, lplf) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontW(param0: HIMC, lplf: *const super::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : HIMC, lplf : *const super::LOGFONTW) -> windows_core::BOOL);
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
pub unsafe fn ImmSetStatusWindowPos(param0: HIMC, lpptpos: *const super::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : HIMC, lpptpos : *const super::POINT) -> windows_core::BOOL);
    unsafe { ImmSetStatusWindowPos(param0, lpptpos) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn ImmSimulateHotKey(param0: super::HWND, param1: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::HWND, param1 : u32) -> windows_core::BOOL);
    unsafe { ImmSimulateHotKey(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmUnregisterWordA<P1, P3>(param0: super::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszunregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordA(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ImmUnregisterWordW<P1, P3>(param0: super::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszunregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordW(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
pub const ATTR_CONVERTED: i32 = 2;
pub const ATTR_FIXEDCONVERTED: i32 = 5;
pub const ATTR_INPUT: i32 = 0;
pub const ATTR_INPUT_ERROR: i32 = 4;
pub const ATTR_TARGET_CONVERTED: i32 = 1;
pub const ATTR_TARGET_NOTCONVERTED: i32 = 3;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::POINT,
    pub rcArea: super::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const CFS_CANDIDATEPOS: i32 = 64;
pub const CFS_DEFAULT: i32 = 0;
pub const CFS_EXCLUDE: i32 = 128;
pub const CFS_FORCE_POSITION: i32 = 32;
pub const CFS_POINT: i32 = 2;
pub const CFS_RECT: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::POINT,
    pub rcArea: super::RECT,
}
pub const CPS_CANCEL: i32 = 4;
pub const CPS_COMPLETE: i32 = 1;
pub const CPS_CONVERT: i32 = 2;
pub const CPS_REVERT: i32 = 3;
pub const CS_INSERTCHAR: i32 = 8192;
pub const CS_NOMOVECARET: i32 = 16384;
pub const GCL_CONVERSION: i32 = 1;
pub const GCL_REVERSECONVERSION: i32 = 2;
pub const GCL_REVERSE_LENGTH: i32 = 3;
pub const GCS_COMPATTR: i32 = 16;
pub const GCS_COMPCLAUSE: i32 = 32;
pub const GCS_COMPREADATTR: i32 = 2;
pub const GCS_COMPREADCLAUSE: i32 = 4;
pub const GCS_COMPREADSTR: i32 = 1;
pub const GCS_COMPSTR: i32 = 8;
pub const GCS_CURSORPOS: i32 = 128;
pub const GCS_DELTASTART: i32 = 256;
pub const GCS_RESULTCLAUSE: i32 = 4096;
pub const GCS_RESULTREADCLAUSE: i32 = 1024;
pub const GCS_RESULTREADSTR: i32 = 512;
pub const GCS_RESULTSTR: i32 = 2048;
pub const GGL_INDEX: i32 = 2;
pub const GGL_LEVEL: i32 = 1;
pub const GGL_PRIVATE: i32 = 4;
pub const GGL_STRING: i32 = 3;
pub const GL_ID_CANNOTSAVE: i32 = 17;
pub const GL_ID_CHOOSECANDIDATE: i32 = 40;
pub const GL_ID_INPUTCODE: i32 = 38;
pub const GL_ID_INPUTRADICAL: i32 = 37;
pub const GL_ID_INPUTREADING: i32 = 36;
pub const GL_ID_INPUTSYMBOL: i32 = 39;
pub const GL_ID_NOCONVERT: i32 = 32;
pub const GL_ID_NODICTIONARY: i32 = 16;
pub const GL_ID_NOMODULE: i32 = 1;
pub const GL_ID_PRIVATE_FIRST: i32 = 32768;
pub const GL_ID_PRIVATE_LAST: i32 = 65535;
pub const GL_ID_READINGCONFLICT: i32 = 35;
pub const GL_ID_REVERSECONVERSION: i32 = 41;
pub const GL_ID_TOOMANYSTROKE: i32 = 34;
pub const GL_ID_TYPINGERROR: i32 = 33;
pub const GL_ID_UNKNOWN: i32 = 0;
pub const GL_LEVEL_ERROR: i32 = 2;
pub const GL_LEVEL_FATAL: i32 = 1;
pub const GL_LEVEL_INFORMATION: i32 = 4;
pub const GL_LEVEL_NOGUIDELINE: i32 = 0;
pub const GL_LEVEL_WARNING: i32 = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMC(pub *mut core::ffi::c_void);
impl Default for HIMC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMCC(pub *mut core::ffi::c_void);
impl Default for HIMCC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IACE_CHILDREN: i32 = 1;
pub const IACE_DEFAULT: i32 = 16;
pub const IACE_IGNORENOCONTEXT: i32 = 32;
pub const IGIMIF_RIGHTMENU: i32 = 1;
pub const IGIMII_CMODE: i32 = 1;
pub const IGIMII_CONFIGURE: i32 = 4;
pub const IGIMII_HELP: i32 = 16;
pub const IGIMII_INPUTTOOLS: i32 = 64;
pub const IGIMII_OTHER: i32 = 32;
pub const IGIMII_SMODE: i32 = 2;
pub const IGIMII_TOOLS: i32 = 8;
pub const IGP_CONVERSION: i32 = 8;
pub const IGP_GETIMEVERSION: u32 = 4294967292;
pub const IGP_PROPERTY: i32 = 4;
pub const IGP_SELECT: i32 = 24;
pub const IGP_SENTENCE: i32 = 12;
pub const IGP_SETCOMPSTR: i32 = 20;
pub const IGP_UI: i32 = 16;
#[cfg(feature = "minwindef")]
pub type IMCENUMPROC = Option<unsafe extern "system" fn(param0: HIMC, param1: super::LPARAM) -> windows_core::BOOL>;
pub const IMC_CLOSESTATUSWINDOW: i32 = 33;
pub const IMC_GETCANDIDATEPOS: i32 = 7;
pub const IMC_GETCOMPOSITIONFONT: i32 = 9;
pub const IMC_GETCOMPOSITIONWINDOW: i32 = 11;
pub const IMC_GETSTATUSWINDOWPOS: i32 = 15;
pub const IMC_OPENSTATUSWINDOW: i32 = 34;
pub const IMC_SETCANDIDATEPOS: i32 = 8;
pub const IMC_SETCOMPOSITIONFONT: i32 = 10;
pub const IMC_SETCOMPOSITIONWINDOW: i32 = 12;
pub const IMC_SETSTATUSWINDOWPOS: i32 = 16;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::RECT,
}
#[cfg(feature = "windef")]
pub type IMEMENUITEMINFO = IMEMENUITEMINFOA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: u32,
    pub szString: [i8; 80],
    pub hbmpItem: super::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::HBITMAP,
    pub hbmpUnchecked: super::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::HBITMAP,
}
#[cfg(feature = "windef")]
impl Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEMENUITEM_STRING_SIZE: i32 = 80;
pub const IMEVER_0310: i32 = 196618;
pub const IMEVER_0400: i32 = 262144;
pub const IME_CAND_CODE: i32 = 2;
pub const IME_CAND_MEANING: i32 = 3;
pub const IME_CAND_RADICAL: i32 = 4;
pub const IME_CAND_READ: i32 = 1;
pub const IME_CAND_STROKE: i32 = 5;
pub const IME_CAND_UNKNOWN: i32 = 0;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: i32 = 16;
pub const IME_CHOTKEY_SHAPE_TOGGLE: i32 = 17;
pub const IME_CHOTKEY_SYMBOL_TOGGLE: i32 = 18;
pub const IME_CMODE_EUDC: i32 = 512;
pub const IME_CMODE_FIXED: i32 = 2048;
pub const IME_CMODE_HANGEUL: i32 = 1;
pub const IME_CMODE_NOCONVERSION: i32 = 256;
pub const IME_CMODE_RESERVED: u32 = 4026531840;
pub const IME_CMODE_SOFTKBD: i32 = 128;
pub const IME_CMODE_SYMBOL: i32 = 1024;
pub const IME_CONFIG_GENERAL: i32 = 1;
pub const IME_CONFIG_REGISTERWORD: i32 = 2;
pub const IME_CONFIG_SELECTDICTIONARY: i32 = 3;
pub const IME_ESC_AUTOMATA: i32 = 4105;
pub const IME_ESC_GETHELPFILENAME: i32 = 4107;
pub const IME_ESC_GET_EUDC_DICTIONARY: i32 = 4099;
pub const IME_ESC_HANJA_MODE: i32 = 4104;
pub const IME_ESC_IME_NAME: i32 = 4102;
pub const IME_ESC_MAX_KEY: i32 = 4101;
pub const IME_ESC_PRIVATE_FIRST: i32 = 2048;
pub const IME_ESC_PRIVATE_HOTKEY: i32 = 4106;
pub const IME_ESC_PRIVATE_LAST: i32 = 4095;
pub const IME_ESC_QUERY_SUPPORT: i32 = 3;
pub const IME_ESC_RESERVED_FIRST: i32 = 4;
pub const IME_ESC_RESERVED_LAST: i32 = 2047;
pub const IME_ESC_SEQUENCE_TO_INTERNAL: i32 = 4097;
pub const IME_ESC_SET_EUDC_DICTIONARY: i32 = 4100;
pub const IME_ESC_SYNC_HOTKEY: i32 = 4103;
pub const IME_HOTKEY_DSWITCH_FIRST: i32 = 256;
pub const IME_HOTKEY_DSWITCH_LAST: i32 = 287;
pub const IME_HOTKEY_PRIVATE_FIRST: i32 = 512;
pub const IME_HOTKEY_PRIVATE_LAST: i32 = 543;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: i32 = 513;
pub const IME_ITHOTKEY_RECONVERTSTRING: i32 = 515;
pub const IME_ITHOTKEY_RESEND_RESULTSTR: i32 = 512;
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: i32 = 514;
pub const IME_JHOTKEY_CLOSE_OPEN: i32 = 48;
pub const IME_KHOTKEY_ENGLISH: i32 = 82;
pub const IME_KHOTKEY_HANJACONVERT: i32 = 81;
pub const IME_KHOTKEY_SHAPE_TOGGLE: i32 = 80;
pub const IME_PROP_AT_CARET: i32 = 65536;
pub const IME_PROP_CANDLIST_START_FROM_1: i32 = 262144;
pub const IME_PROP_COMPLETE_ON_UNSELECT: i32 = 1048576;
pub const IME_PROP_SPECIAL_UI: i32 = 131072;
pub const IME_PROP_UNICODE: i32 = 524288;
pub const IME_REGWORD_STYLE_EUDC: i32 = 1;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295;
pub const IME_SMODE_AUTOMATIC: i32 = 4;
pub const IME_SMODE_CONVERSATION: i32 = 16;
pub const IME_SMODE_NONE: i32 = 0;
pub const IME_SMODE_PHRASEPREDICT: i32 = 8;
pub const IME_SMODE_PLAURALCLAUSE: i32 = 1;
pub const IME_SMODE_RESERVED: i32 = 61440;
pub const IME_SMODE_SINGLECONVERT: i32 = 2;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: i32 = 112;
pub const IME_THOTKEY_SHAPE_TOGGLE: i32 = 113;
pub const IME_THOTKEY_SYMBOL_TOGGLE: i32 = 114;
pub const IMFS_CHECKED: i32 = 8;
pub const IMFS_DEFAULT: i32 = 4096;
pub const IMFS_DISABLED: i32 = 3;
pub const IMFS_ENABLED: i32 = 0;
pub const IMFS_GRAYED: i32 = 3;
pub const IMFS_HILITE: i32 = 128;
pub const IMFS_UNCHECKED: i32 = 0;
pub const IMFS_UNHILITE: i32 = 0;
pub const IMFT_RADIOCHECK: i32 = 1;
pub const IMFT_SEPARATOR: i32 = 2;
pub const IMFT_SUBMENU: i32 = 4;
pub const IMM_ERROR_GENERAL: i32 = -2;
pub const IMM_ERROR_NODATA: i32 = -1;
pub const IMN_CHANGECANDIDATE: i32 = 3;
pub const IMN_CLOSECANDIDATE: i32 = 4;
pub const IMN_CLOSESTATUSWINDOW: i32 = 1;
pub const IMN_GUIDELINE: i32 = 13;
pub const IMN_OPENCANDIDATE: i32 = 5;
pub const IMN_OPENSTATUSWINDOW: i32 = 2;
pub const IMN_PRIVATE: i32 = 14;
pub const IMN_SETCANDIDATEPOS: i32 = 9;
pub const IMN_SETCOMPOSITIONFONT: i32 = 10;
pub const IMN_SETCOMPOSITIONWINDOW: i32 = 11;
pub const IMN_SETCONVERSIONMODE: i32 = 6;
pub const IMN_SETOPENSTATUS: i32 = 8;
pub const IMN_SETSENTENCEMODE: i32 = 7;
pub const IMN_SETSTATUSWINDOWPOS: i32 = 12;
pub const IMR_CANDIDATEWINDOW: i32 = 2;
pub const IMR_COMPOSITIONFONT: i32 = 3;
pub const IMR_COMPOSITIONWINDOW: i32 = 1;
pub const IMR_CONFIRMRECONVERTSTRING: i32 = 5;
pub const IMR_DOCUMENTFEED: i32 = 7;
pub const IMR_QUERYCHARPOSITION: i32 = 6;
pub const IMR_RECONVERTSTRING: i32 = 4;
pub const ISC_SHOWUIALL: u32 = 3221225487;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: i32 = 15;
pub const ISC_SHOWUICANDIDATEWINDOW: i32 = 1;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648;
pub const ISC_SHOWUIGUIDELINE: i32 = 1073741824;
#[cfg(feature = "windef")]
pub type LPCANDIDATEFORM = *mut CANDIDATEFORM;
pub type LPCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(feature = "windef")]
pub type LPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(feature = "minwindef")]
pub type LPHKL = *mut super::HKL;
#[cfg(feature = "windef")]
pub type LPIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(feature = "windef")]
pub type LPIMEMENUITEMINFO = LPIMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type LPIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
pub type LPRECONVERTSTRING = *mut RECONVERTSTRING;
pub type LPREGISTERWORD = LPREGISTERWORDA;
pub type LPREGISTERWORDA = *mut REGISTERWORDA;
pub type LPREGISTERWORDW = *mut REGISTERWORDW;
pub type LPSTYLEBUF = LPSTYLEBUFA;
pub type LPSTYLEBUFA = *mut STYLEBUFA;
pub type LPSTYLEBUFW = *mut STYLEBUFW;
pub const MOD_IGNORE_ALL_MODIFIER: i32 = 1024;
pub const MOD_LEFT: i32 = 32768;
pub const MOD_ON_KEYUP: i32 = 2048;
pub const MOD_RIGHT: i32 = 16384;
pub const NI_CHANGECANDIDATELIST: i32 = 19;
pub const NI_CLOSECANDIDATE: i32 = 17;
pub const NI_COMPOSITIONSTR: i32 = 21;
pub const NI_FINALIZECONVERSIONRESULT: i32 = 20;
pub const NI_IMEMENUSELECTED: i32 = 24;
pub const NI_OPENCANDIDATE: i32 = 16;
pub const NI_SELECTCANDIDATESTR: i32 = 18;
pub const NI_SETCANDIDATE_PAGESIZE: i32 = 23;
pub const NI_SETCANDIDATE_PAGESTART: i32 = 22;
#[cfg(feature = "windef")]
pub type NPCANDIDATEFORM = *mut CANDIDATEFORM;
pub type NPCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(feature = "windef")]
pub type NPCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(feature = "windef")]
pub type NPIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(feature = "windef")]
pub type NPIMEMENUITEMINFO = NPIMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type NPIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type NPIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
pub type NPRECONVERTSTRING = *mut RECONVERTSTRING;
pub type NPREGISTERWORD = NPREGISTERWORDA;
pub type NPREGISTERWORDA = *mut REGISTERWORDA;
pub type NPREGISTERWORDW = *mut REGISTERWORDW;
pub type NPSTYLEBUF = NPSTYLEBUFA;
pub type NPSTYLEBUFA = *mut STYLEBUFA;
pub type NPSTYLEBUFW = *mut STYLEBUFW;
#[cfg(feature = "windef")]
pub type PCANDIDATEFORM = *mut CANDIDATEFORM;
pub type PCANDIDATELIST = *mut CANDIDATELIST;
#[cfg(feature = "windef")]
pub type PCOMPOSITIONFORM = *mut COMPOSITIONFORM;
#[cfg(feature = "windef")]
pub type PIMECHARPOSITION = *mut IMECHARPOSITION;
#[cfg(feature = "windef")]
pub type PIMEMENUITEMINFO = PIMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type PIMEMENUITEMINFOA = *mut IMEMENUITEMINFOA;
#[cfg(feature = "windef")]
pub type PIMEMENUITEMINFOW = *mut IMEMENUITEMINFOW;
pub type PRECONVERTSTRING = *mut RECONVERTSTRING;
pub type PREGISTERWORD = PREGISTERWORDA;
pub type PREGISTERWORDA = *mut REGISTERWORDA;
pub type PREGISTERWORDW = *mut REGISTERWORDW;
pub type PSTYLEBUF = PSTYLEBUFA;
pub type PSTYLEBUFA = *mut STYLEBUFA;
pub type PSTYLEBUFW = *mut STYLEBUFW;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REGISTERWORDA {
    pub lpReading: windows_core::PSTR,
    pub lpWord: windows_core::PSTR,
}
pub type REGISTERWORDENUMPROCA = Option<unsafe extern "system" fn(lpszreading: windows_core::PCSTR, param1: u32, lpszstring: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = Option<unsafe extern "system" fn(lpszreading: windows_core::PCWSTR, param1: u32, lpszstring: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REGISTERWORDW {
    pub lpReading: windows_core::PWSTR,
    pub lpWord: windows_core::PWSTR,
}
pub const SCS_CAP_COMPSTR: i32 = 1;
pub const SCS_CAP_MAKEREAD: i32 = 2;
pub const SCS_CAP_SETRECONVERTSTRING: i32 = 4;
pub const SCS_CHANGEATTR: i32 = 18;
pub const SCS_CHANGECLAUSE: i32 = 36;
pub const SCS_QUERYRECONVERTSTRING: i32 = 131072;
pub const SCS_SETRECONVERTSTRING: i32 = 65536;
pub const SCS_SETSTR: i32 = 9;
pub const SELECT_CAP_CONVERSION: i32 = 1;
pub const SELECT_CAP_SENTENCE: i32 = 2;
pub const SOFTKEYBOARD_TYPE_C1: i32 = 2;
pub const SOFTKEYBOARD_TYPE_T1: i32 = 1;
pub type STYLEBUF = STYLEBUFA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STYLE_DESCRIPTION_SIZE: i32 = 32;
pub const UI_CAP_2700: i32 = 1;
pub const UI_CAP_ROT90: i32 = 2;
pub const UI_CAP_ROTANY: i32 = 4;
