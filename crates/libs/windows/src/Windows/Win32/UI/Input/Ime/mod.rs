#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContext<P0, P1>(param0: P0, param1: P1) -> super::super::super::Globalization::HIMC
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::super::super::Foundation:: HWND, param1 : super::super::super::Globalization:: HIMC) -> super::super::super::Globalization:: HIMC);
    ImmAssociateContext(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContextEx<P0, P1>(param0: P0, param1: P1, param2: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::super::super::Foundation:: HWND, param1 : super::super::super::Globalization:: HIMC, param2 : u32) -> super::super::super::Foundation:: BOOL);
    ImmAssociateContextEx(param0.into_param().abi(), param1.into_param().abi(), param2)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut ::core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    ImmConfigureIMEA(param0.into_param().abi(), param1.into_param().abi(), param2, param3)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut ::core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    ImmConfigureIMEW(param0.into_param().abi(), param1.into_param().abi(), param2, param3)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateContext() -> super::super::super::Globalization::HIMC {
    ::windows_targets::link!("imm32.dll" "system" fn ImmCreateContext() -> super::super::super::Globalization:: HIMC);
    ImmCreateContext()
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC {
    ::windows_targets::link!("imm32.dll" "system" fn ImmCreateIMCC(param0 : u32) -> super::super::super::Globalization:: HIMCC);
    ImmCreateIMCC(param0)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmCreateSoftKeyboard<P0>(param0: u32, param1: P0, param2: i32, param3: i32) -> super::super::super::Foundation::HWND
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmCreateSoftKeyboard(param0 : u32, param1 : super::super::super::Foundation:: HWND, param2 : i32, param3 : i32) -> super::super::super::Foundation:: HWND);
    ImmCreateSoftKeyboard(param0, param1.into_param().abi(), param2, param3)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmDestroyContext<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : super::super::super::Globalization:: HIMC) -> super::super::super::Foundation:: BOOL);
    ImmDestroyContext(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmDestroyIMCC<P0>(param0: P0) -> super::super::super::Globalization::HIMCC
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmDestroyIMCC(param0 : super::super::super::Globalization:: HIMCC) -> super::super::super::Globalization:: HIMCC);
    ImmDestroyIMCC(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDestroySoftKeyboard<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmDestroySoftKeyboard(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: BOOL);
    ImmDestroySoftKeyboard(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link!("imm32.dll" "system" fn ImmDisableIME(param0 : u32) -> super::super::super::Foundation:: BOOL);
    ImmDisableIME(param0)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL {
    ::windows_targets::link!("imm32.dll" "system" fn ImmDisableLegacyIME() -> super::super::super::Foundation:: BOOL);
    ImmDisableLegacyIME()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link!("imm32.dll" "system" fn ImmDisableTextFrameService(idthread : u32) -> super::super::super::Foundation:: BOOL);
    ImmDisableTextFrameService(idthread)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmEnumInputContext<P0>(idthread: u32, lpfn: IMCENUMPROC, lparam: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : u32, lpfn : IMCENUMPROC, lparam : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmEnumInputContext(idthread, lpfn, lparam.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<P0, P1, P2>(param0: P0, param1: REGISTERWORDENUMPROCA, lpszreading: P1, param3: u32, lpszregister: P2, param5: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::super::TextServices:: HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : ::windows_core::PCSTR, param3 : u32, lpszregister : ::windows_core::PCSTR, param5 : *mut ::core::ffi::c_void) -> u32);
    ImmEnumRegisterWordA(param0.into_param().abi(), param1, lpszreading.into_param().abi(), param3, lpszregister.into_param().abi(), param5)
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<P0, P1, P2>(param0: P0, param1: REGISTERWORDENUMPROCW, lpszreading: P1, param3: u32, lpszregister: P2, param5: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::super::TextServices:: HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : ::windows_core::PCWSTR, param3 : u32, lpszregister : ::windows_core::PCWSTR, param5 : *mut ::core::ffi::c_void) -> u32);
    ImmEnumRegisterWordW(param0.into_param().abi(), param1, lpszreading.into_param().abi(), param3, lpszregister.into_param().abi(), param5)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeA<P0, P1>(param0: P0, param1: P1, param2: IME_ESCAPE, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Globalization:: HIMC, param2 : IME_ESCAPE, param3 : *mut ::core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    ImmEscapeA(param0.into_param().abi(), param1.into_param().abi(), param2, param3)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeW<P0, P1>(param0: P0, param1: P1, param2: IME_ESCAPE, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Globalization:: HIMC, param2 : IME_ESCAPE, param3 : *mut ::core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    ImmEscapeW(param0.into_param().abi(), param1.into_param().abi(), param2, param3)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGenerateMessage<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGenerateMessage(param0 : super::super::super::Globalization:: HIMC) -> super::super::super::Foundation:: BOOL);
    ImmGenerateMessage(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListA<P0>(param0: P0, deindex: u32, lpcandlist: ::core::option::Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListA(param0 : super::super::super::Globalization:: HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    ImmGetCandidateListA(param0.into_param().abi(), deindex, ::core::mem::transmute(lpcandlist.unwrap_or(::std::ptr::null_mut())), dwbuflen)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountA<P0>(param0: P0, lpdwlistcount: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListCountA(param0 : super::super::super::Globalization:: HIMC, lpdwlistcount : *mut u32) -> u32);
    ImmGetCandidateListCountA(param0.into_param().abi(), lpdwlistcount)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountW<P0>(param0: P0, lpdwlistcount: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListCountW(param0 : super::super::super::Globalization:: HIMC, lpdwlistcount : *mut u32) -> u32);
    ImmGetCandidateListCountW(param0.into_param().abi(), lpdwlistcount)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListW<P0>(param0: P0, deindex: u32, lpcandlist: ::core::option::Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListW(param0 : super::super::super::Globalization:: HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    ImmGetCandidateListW(param0.into_param().abi(), deindex, ::core::mem::transmute(lpcandlist.unwrap_or(::std::ptr::null_mut())), dwbuflen)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCandidateWindow<P0>(param0: P0, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateWindow(param0 : super::super::super::Globalization:: HIMC, param1 : u32, lpcandidate : *mut CANDIDATEFORM) -> super::super::super::Foundation:: BOOL);
    ImmGetCandidateWindow(param0.into_param().abi(), param1, lpcandidate)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontA<P0>(param0: P0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : super::super::super::Globalization:: HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTA) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionFontA(param0.into_param().abi(), lplf)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontW<P0>(param0: P0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : super::super::super::Globalization:: HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTW) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionFontW(param0.into_param().abi(), lplf)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringA<P0>(param0: P0, param1: IME_COMPOSITION_STRING, lpbuf: ::core::option::Option<*mut ::core::ffi::c_void>, dwbuflen: u32) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionStringA(param0 : super::super::super::Globalization:: HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut ::core::ffi::c_void, dwbuflen : u32) -> i32);
    ImmGetCompositionStringA(param0.into_param().abi(), param1, ::core::mem::transmute(lpbuf.unwrap_or(::std::ptr::null_mut())), dwbuflen)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringW<P0>(param0: P0, param1: IME_COMPOSITION_STRING, lpbuf: ::core::option::Option<*mut ::core::ffi::c_void>, dwbuflen: u32) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionStringW(param0 : super::super::super::Globalization:: HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut ::core::ffi::c_void, dwbuflen : u32) -> i32);
    ImmGetCompositionStringW(param0.into_param().abi(), param1, ::core::mem::transmute(lpbuf.unwrap_or(::std::ptr::null_mut())), dwbuflen)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCompositionWindow<P0>(param0: P0, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionWindow(param0 : super::super::super::Globalization:: HIMC, lpcompform : *mut COMPOSITIONFORM) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionWindow(param0.into_param().abi(), lpcompform)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetContext<P0>(param0: P0) -> super::super::super::Globalization::HIMC
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Globalization:: HIMC);
    ImmGetContext(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListA<P0, P1, P2>(param0: P0, param1: P1, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Globalization:: HIMC, lpsrc : ::windows_core::PCSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    ImmGetConversionListA(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), lpdst, dwbuflen, uflag)
}
#[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListW<P0, P1, P2>(param0: P0, param1: P1, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::super::TextServices:: HKL, param1 : super::super::super::Globalization:: HIMC, lpsrc : ::windows_core::PCWSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    ImmGetConversionListW(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), lpdst, dwbuflen, uflag)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetConversionStatus<P0>(param0: P0, lpfdwconversion: ::core::option::Option<*mut IME_CONVERSION_MODE>, lpfdwsentence: ::core::option::Option<*mut IME_SENTENCE_MODE>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : super::super::super::Globalization:: HIMC, lpfdwconversion : *mut IME_CONVERSION_MODE, lpfdwsentence : *mut IME_SENTENCE_MODE) -> super::super::super::Foundation:: BOOL);
    ImmGetConversionStatus(param0.into_param().abi(), ::core::mem::transmute(lpfdwconversion.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpfdwsentence.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd<P0>(param0: P0) -> super::super::super::Foundation::HWND
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: HWND);
    ImmGetDefaultIMEWnd(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetDescriptionA<P0>(param0: P0, lpszdescription: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::super::TextServices:: HKL, lpszdescription : ::windows_core::PSTR, ubuflen : u32) -> u32);
    ImmGetDescriptionA(param0.into_param().abi(), ::core::mem::transmute(lpszdescription.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetDescriptionW<P0>(param0: P0, lpszdescription: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::super::TextServices:: HKL, lpszdescription : ::windows_core::PWSTR, ubuflen : u32) -> u32);
    ImmGetDescriptionW(param0.into_param().abi(), ::core::mem::transmute(lpszdescription.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetGuideLineA<P0>(param0: P0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : super::super::super::Globalization:: HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : ::windows_core::PSTR, dwbuflen : u32) -> u32);
    ImmGetGuideLineA(param0.into_param().abi(), dwindex, ::core::mem::transmute(lpbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetGuideLineW<P0>(param0: P0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: ::windows_core::PWSTR, dwbuflen: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : super::super::super::Globalization:: HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : ::windows_core::PWSTR, dwbuflen : u32) -> u32);
    ImmGetGuideLineW(param0.into_param().abi(), dwindex, ::core::mem::transmute(lpbuf), dwbuflen)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetHotKey(param0 : u32, lpumodifiers : *mut u32, lpuvkey : *mut u32, phkl : *mut super::super::TextServices:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmGetHotKey(param0, lpumodifiers, lpuvkey, phkl)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCLockCount<P0>(param0: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetIMCCLockCount(param0 : super::super::super::Globalization:: HIMCC) -> u32);
    ImmGetIMCCLockCount(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCSize<P0>(param0: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetIMCCSize(param0 : super::super::super::Globalization:: HIMCC) -> u32);
    ImmGetIMCCSize(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCLockCount<P0>(param0: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetIMCLockCount(param0 : super::super::super::Globalization:: HIMC) -> u32);
    ImmGetIMCLockCount(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA<P0>(param0: P0, lpszfilename: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::super::TextServices:: HKL, lpszfilename : ::windows_core::PSTR, ubuflen : u32) -> u32);
    ImmGetIMEFileNameA(param0.into_param().abi(), ::core::mem::transmute(lpszfilename.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW<P0>(param0: P0, lpszfilename: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::super::TextServices:: HKL, lpszfilename : ::windows_core::PWSTR, ubuflen : u32) -> u32);
    ImmGetIMEFileNameW(param0.into_param().abi(), ::core::mem::transmute(lpszfilename.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA<P0>(param0: P0, param1: u32, param2: u32, lpimeparentmenu: ::core::option::Option<*mut IMEMENUITEMINFOA>, lpimemenu: ::core::option::Option<*mut IMEMENUITEMINFOA>, dwsize: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetImeMenuItemsA(param0 : super::super::super::Globalization:: HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOA, lpimemenu : *mut IMEMENUITEMINFOA, dwsize : u32) -> u32);
    ImmGetImeMenuItemsA(param0.into_param().abi(), param1, param2, ::core::mem::transmute(lpimeparentmenu.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpimemenu.unwrap_or(::std::ptr::null_mut())), dwsize)
}
#[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW<P0>(param0: P0, param1: u32, param2: u32, lpimeparentmenu: ::core::option::Option<*mut IMEMENUITEMINFOW>, lpimemenu: ::core::option::Option<*mut IMEMENUITEMINFOW>, dwsize: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetImeMenuItemsW(param0 : super::super::super::Globalization:: HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOW, lpimemenu : *mut IMEMENUITEMINFOW, dwsize : u32) -> u32);
    ImmGetImeMenuItemsW(param0.into_param().abi(), param1, param2, ::core::mem::transmute(lpimeparentmenu.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpimemenu.unwrap_or(::std::ptr::null_mut())), dwsize)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetOpenStatus<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetOpenStatus(param0 : super::super::super::Globalization:: HIMC) -> super::super::super::Foundation:: BOOL);
    ImmGetOpenStatus(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetProperty<P0>(param0: P0, param1: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::super::TextServices:: HKL, param1 : u32) -> u32);
    ImmGetProperty(param0.into_param().abi(), param1)
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA<P0>(param0: P0, lpstylebuf: &mut [STYLEBUFA]) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::super::TextServices:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFA) -> u32);
    ImmGetRegisterWordStyleA(param0.into_param().abi(), lpstylebuf.len().try_into().unwrap(), ::core::mem::transmute(lpstylebuf.as_ptr()))
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW<P0>(param0: P0, lpstylebuf: &mut [STYLEBUFW]) -> u32
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::super::TextServices:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFW) -> u32);
    ImmGetRegisterWordStyleW(param0.into_param().abi(), lpstylebuf.len().try_into().unwrap(), ::core::mem::transmute(lpstylebuf.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetStatusWindowPos<P0>(param0: P0, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : super::super::super::Globalization:: HIMC, lpptpos : *mut super::super::super::Foundation:: POINT) -> super::super::super::Foundation:: BOOL);
    ImmGetStatusWindowPos(param0.into_param().abi(), lpptpos)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetVirtualKey<P0>(param0: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::super::super::Foundation:: HWND) -> u32);
    ImmGetVirtualKey(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmInstallIMEA<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::super::TextServices::HKL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : ::windows_core::PCSTR, lpszlayouttext : ::windows_core::PCSTR) -> super::super::TextServices:: HKL);
    ImmInstallIMEA(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi())
}
#[doc = "Required features: `\"Win32_UI_TextServices\"`"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmInstallIMEW<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::super::TextServices::HKL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : ::windows_core::PCWSTR, lpszlayouttext : ::windows_core::PCWSTR) -> super::super::TextServices:: HKL);
    ImmInstallIMEW(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmIsIME<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::super::TextServices:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmIsIME(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageA<P0, P1, P2>(param0: P0, param1: u32, param2: P1, param3: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmIsUIMessageA(param0.into_param().abi(), param1, param2.into_param().abi(), param3.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageW<P0, P1, P2>(param0: P0, param1: u32, param2: P1, param3: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmIsUIMessageW(param0.into_param().abi(), param1, param2.into_param().abi(), param3.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmLockIMC<P0>(param0: P0) -> *mut INPUTCONTEXT
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmLockIMC(param0 : super::super::super::Globalization:: HIMC) -> *mut INPUTCONTEXT);
    ImmLockIMC(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmLockIMCC<P0>(param0: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmLockIMCC(param0 : super::super::super::Globalization:: HIMCC) -> *mut ::core::ffi::c_void);
    ImmLockIMCC(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmNotifyIME<P0>(param0: P0, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : super::super::super::Globalization:: HIMC, dwaction : NOTIFY_IME_ACTION, dwindex : NOTIFY_IME_INDEX, dwvalue : u32) -> super::super::super::Foundation:: BOOL);
    ImmNotifyIME(param0.into_param().abi(), dwaction, dwindex, dwvalue)
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmReSizeIMCC<P0>(param0: P0, param1: u32) -> super::super::super::Globalization::HIMCC
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmReSizeIMCC(param0 : super::super::super::Globalization:: HIMCC, param1 : u32) -> super::super::super::Globalization:: HIMCC);
    ImmReSizeIMCC(param0.into_param().abi(), param1)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordA<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::super::TextServices:: HKL, lpszreading : ::windows_core::PCSTR, param2 : u32, lpszregister : ::windows_core::PCSTR) -> super::super::super::Foundation:: BOOL);
    ImmRegisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), param2, lpszregister.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordW<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::super::TextServices:: HKL, lpszreading : ::windows_core::PCWSTR, param2 : u32, lpszregister : ::windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    ImmRegisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), param2, lpszregister.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmReleaseContext<P0, P1>(param0: P0, param1: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::super::super::Foundation:: HWND, param1 : super::super::super::Globalization:: HIMC) -> super::super::super::Foundation:: BOOL);
    ImmReleaseContext(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageA<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> super::super::super::Foundation::LRESULT
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmRequestMessageA(param0 : super::super::super::Globalization:: HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    ImmRequestMessageA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageW<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> super::super::super::Foundation::LRESULT
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
    P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmRequestMessageW(param0 : super::super::super::Globalization:: HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    ImmRequestMessageW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCandidateWindow<P0>(param0: P0, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCandidateWindow(param0 : super::super::super::Globalization:: HIMC, lpcandidate : *const CANDIDATEFORM) -> super::super::super::Foundation:: BOOL);
    ImmSetCandidateWindow(param0.into_param().abi(), lpcandidate)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontA<P0>(param0: P0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : super::super::super::Globalization:: HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTA) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionFontA(param0.into_param().abi(), lplf)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontW<P0>(param0: P0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : super::super::super::Globalization:: HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTW) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionFontW(param0.into_param().abi(), lplf)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringA<P0>(param0: P0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: ::core::option::Option<*const ::core::ffi::c_void>, dwcomplen: u32, lpread: ::core::option::Option<*const ::core::ffi::c_void>, dwreadlen: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionStringA(param0 : super::super::super::Globalization:: HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const ::core::ffi::c_void, dwcomplen : u32, lpread : *const ::core::ffi::c_void, dwreadlen : u32) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionStringA(param0.into_param().abi(), dwindex, ::core::mem::transmute(lpcomp.unwrap_or(::std::ptr::null())), dwcomplen, ::core::mem::transmute(lpread.unwrap_or(::std::ptr::null())), dwreadlen)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringW<P0>(param0: P0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: ::core::option::Option<*const ::core::ffi::c_void>, dwcomplen: u32, lpread: ::core::option::Option<*const ::core::ffi::c_void>, dwreadlen: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionStringW(param0 : super::super::super::Globalization:: HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const ::core::ffi::c_void, dwcomplen : u32, lpread : *const ::core::ffi::c_void, dwreadlen : u32) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionStringW(param0.into_param().abi(), dwindex, ::core::mem::transmute(lpcomp.unwrap_or(::std::ptr::null())), dwcomplen, ::core::mem::transmute(lpread.unwrap_or(::std::ptr::null())), dwreadlen)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionWindow<P0>(param0: P0, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionWindow(param0 : super::super::super::Globalization:: HIMC, lpcompform : *const COMPOSITIONFORM) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionWindow(param0.into_param().abi(), lpcompform)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetConversionStatus<P0>(param0: P0, param1: IME_CONVERSION_MODE, param2: IME_SENTENCE_MODE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetConversionStatus(param0 : super::super::super::Globalization:: HIMC, param1 : IME_CONVERSION_MODE, param2 : IME_SENTENCE_MODE) -> super::super::super::Foundation:: BOOL);
    ImmSetConversionStatus(param0.into_param().abi(), param1, param2)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmSetHotKey<P0>(param0: u32, param1: u32, param2: u32, param3: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetHotKey(param0 : u32, param1 : u32, param2 : u32, param3 : super::super::TextServices:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmSetHotKey(param0, param1, param2, param3.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetOpenStatus<P0, P1>(param0: P0, param1: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetOpenStatus(param0 : super::super::super::Globalization:: HIMC, param1 : super::super::super::Foundation:: BOOL) -> super::super::super::Foundation:: BOOL);
    ImmSetOpenStatus(param0.into_param().abi(), param1.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetStatusWindowPos<P0>(param0: P0, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : super::super::super::Globalization:: HIMC, lpptpos : *const super::super::super::Foundation:: POINT) -> super::super::super::Foundation:: BOOL);
    ImmSetStatusWindowPos(param0.into_param().abi(), lpptpos)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmShowSoftKeyboard<P0>(param0: P0, param1: i32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmShowSoftKeyboard(param0 : super::super::super::Foundation:: HWND, param1 : i32) -> super::super::super::Foundation:: BOOL);
    ImmShowSoftKeyboard(param0.into_param().abi(), param1)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmSimulateHotKey<P0>(param0: P0, param1: IME_HOTKEY_IDENTIFIER) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::super::super::Foundation:: HWND, param1 : IME_HOTKEY_IDENTIFIER) -> super::super::super::Foundation:: BOOL);
    ImmSimulateHotKey(param0.into_param().abi(), param1)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMC<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmUnlockIMC(param0 : super::super::super::Globalization:: HIMC) -> super::super::super::Foundation:: BOOL);
    ImmUnlockIMC(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMCC<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmUnlockIMCC(param0 : super::super::super::Globalization:: HIMCC) -> super::super::super::Foundation:: BOOL);
    ImmUnlockIMCC(param0.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordA<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszunregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::super::TextServices:: HKL, lpszreading : ::windows_core::PCSTR, param2 : u32, lpszunregister : ::windows_core::PCSTR) -> super::super::super::Foundation:: BOOL);
    ImmUnregisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), param2, lpszunregister.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordW<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszunregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::super::TextServices:: HKL, lpszreading : ::windows_core::PCWSTR, param2 : u32, lpszunregister : ::windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    ImmUnregisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), param2, lpszunregister.into_param().abi())
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIME(::windows_core::IUnknown);
impl IActiveIME {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Inquire)(::windows_core::Interface::as_raw(self), dwsysteminfoflags, pimeinfo, ::core::mem::transmute(szwndclass), pdwprivate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ConversionList<P0, P1>(&self, himc: P0, szsource: P1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ConversionList)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), szsource.into_param().abi(), uflag, ubuflen, pdest, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Configure)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pregisterword).ok()
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Destroy)(::windows_core::Interface::as_raw(self), ureserved).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<P0>(&self, himc: P0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<P0, P1>(&self, himc: P0, fflag: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetActiveContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<P0>(&self, himc: P0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).ProcessKey)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), uvirkey, lparam, pbkeystate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<P0, P1>(&self, himc: P0, fselect: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Select)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionString)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<P0>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: P0, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).ToAsciiEx)(::windows_core::Interface::as_raw(self), uvirkey, uscancode, pbkeystate, fustate, himc.into_param().abi(), pdwtransbuf, pusize).ok()
    }
    pub unsafe fn RegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szstring.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szstring.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyle)(::windows_core::Interface::as_raw(self), nitem, pstylebuf, pubufsize).ok()
    }
    pub unsafe fn EnumRegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szregister: P1, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodePageA(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLangId(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIME, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActiveIME {
    type Vtable = IActiveIME_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIME {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fe20962_d077_11d0_8fe7_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Inquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Globalization")]
    pub ConversionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: ::windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ConversionList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    Configure: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    Escape: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetActiveContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetActiveContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ProcessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ProcessKey: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    Notify: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    Select: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionString: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ToAsciiEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ToAsciiEx: usize,
    pub RegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRegisterWordStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::HRESULT,
    pub EnumRegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIME2(::windows_core::IUnknown);
impl IActiveIME2 {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Inquire)(::windows_core::Interface::as_raw(self), dwsysteminfoflags, pimeinfo, ::core::mem::transmute(szwndclass), pdwprivate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ConversionList<P0, P1>(&self, himc: P0, szsource: P1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ConversionList)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), szsource.into_param().abi(), uflag, ubuflen, pdest, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.Configure)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pregisterword).ok()
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Destroy)(::windows_core::Interface::as_raw(self), ureserved).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<P0>(&self, himc: P0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.Escape)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<P0, P1>(&self, himc: P0, fflag: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetActiveContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<P0>(&self, himc: P0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.ProcessKey)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), uvirkey, lparam, pbkeystate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.Notify)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<P0, P1>(&self, himc: P0, fselect: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Select)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCompositionString)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<P0>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: P0, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.ToAsciiEx)(::windows_core::Interface::as_raw(self), uvirkey, uscancode, pbkeystate, fustate, himc.into_param().abi(), pdwtransbuf, pusize).ok()
    }
    pub unsafe fn RegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.RegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szstring.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.UnregisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szstring.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRegisterWordStyle)(::windows_core::Interface::as_raw(self), nitem, pstylebuf, pubufsize).ok()
    }
    pub unsafe fn EnumRegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szregister: P1, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumRegisterWord)(::windows_core::Interface::as_raw(self), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCodePageA(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCodePageA)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLangId(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLangId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Sleep(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Sleep)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unsleep<P0>(&self, fdead: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Unsleep)(::windows_core::Interface::as_raw(self), fdead.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIME2, ::windows_core::IUnknown, IActiveIME);
unsafe impl ::windows_core::Interface for IActiveIME2 {
    type Vtable = IActiveIME2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIME2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1c4bf0e_2d53_11d2_93e1_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME2_Vtbl {
    pub base__: IActiveIME_Vtbl,
    pub Sleep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Unsleep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unsleep: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIMMApp(::windows_core::IUnknown);
impl IActiveIMMApp {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<P0, P1>(&self, hwnd: P0, hime: P1) -> ::windows_core::Result<super::super::super::Globalization::HIMC>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssociateContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), hime.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).ConfigureIMEA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pdata).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).ConfigureIMEW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pdata).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<P0>(&self, hime: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).DestroyContext)(::windows_core::Interface::as_raw(self), hime.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).EscapeA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).EscapeW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListCountA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListCountW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<P0>(&self, himc: P0, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcandidate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<P0>(&self, himc: P0, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcompform).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<P0>(&self, hwnd: P0) -> ::windows_core::Result<super::super::super::Globalization::HIMC>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).GetConversionListA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetConversionListW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<P0>(&self, himc: P0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pfdwconversion, pfdwsentence).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<P0>(&self, hwnd: P0) -> ::windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultIMEWnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionA<P0>(&self, hkl: P0, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetDescriptionA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szdescription), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionW<P0>(&self, hkl: P0, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetDescriptionW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szdescription), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetGuideLineA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, ::core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetGuideLineW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, ::core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameA<P0>(&self, hkl: P0, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetIMEFileNameA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szfilename), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameW<P0>(&self, hkl: P0, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetIMEFileNameW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szfilename), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<P0>(&self, himc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<P0>(&self, hkl: P0, fdwindex: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), fdwindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleA<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<P0>(&self, himc: P0) -> ::windows_core::Result<super::super::super::Foundation::POINT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<P0>(&self, hwnd: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVirtualKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> ::windows_core::Result<super::super::TextServices::HKL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEA)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> ::windows_core::Result<super::super::TextServices::HKL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEW)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<P0>(&self, hkl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).IsIME)(::windows_core::Interface::as_raw(self), hkl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).IsUIMessageA)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).IsUIMessageW)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).NotifyIME)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<P0, P1>(&self, hwnd: P0, himc: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).ReleaseContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<P0>(&self, himc: P0, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcandidate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<P0>(&self, himc: P0, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcompform).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<P0>(&self, himc: P0, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fdwconversion, fdwsentence).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<P0, P1>(&self, himc: P0, fopen: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<P0>(&self, himc: P0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pptpos).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<P0>(&self, hwnd: P0, dwhotkeyid: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SimulateHotKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), dwhotkeyid).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szunregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szunregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, frestorelayout: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self), frestorelayout.into_param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDefWindowProc<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OnDefWindowProc)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FilterClientWindows)(::windows_core::Interface::as_raw(self), aaclasslist, usize).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<P0>(&self, hkl: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<P0>(&self, hkl: P0) -> ::windows_core::Result<u16>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<P0, P1>(&self, hwnd: P0, himc: P1, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).AssociateContextEx)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableIME)(::windows_core::Interface::as_raw(self), idthread).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumInputContext)(::windows_core::Interface::as_raw(self), idthread, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIMMApp, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActiveIMMApp {
    type Vtable = IActiveIMMApp_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIMMApp {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c0e040_62d1_11d1_9326_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMApp_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyContext: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCompositionWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetContext: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetConversionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultIMEWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultIMEWnd: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetOpenStatus: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVirtualKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub IsIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    IsIME: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub NotifyIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    NotifyIME: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub ReleaseContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    ReleaseContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCompositionWindow: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetConversionStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetOpenStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulateHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulateHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordW: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDefWindowProc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDefWindowProc: usize,
    pub FilterClientWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetLangId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContextEx: usize,
    pub DisableIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIMMIME(::windows_core::IUnknown);
impl IActiveIMMIME {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<P0, P1>(&self, hwnd: P0, hime: P1) -> ::windows_core::Result<super::super::super::Globalization::HIMC>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AssociateContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), hime.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).ConfigureIMEA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pdata).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).ConfigureIMEW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), hwnd.into_param().abi(), dwmode, pdata).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<super::super::super::Globalization::HIMC> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<P0>(&self, hime: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).DestroyContext)(::windows_core::Interface::as_raw(self), hime.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn EnumRegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumRegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi(), pdata, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).EscapeA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).EscapeW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), uescape, pdata, plresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListCountA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateListCountW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<P0>(&self, himc: P0, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcandidate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<P0>(&self, himc: P0, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcompform).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<P0>(&self, hwnd: P0) -> ::windows_core::Result<super::super::super::Globalization::HIMC>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).GetConversionListA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetConversionListW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<P0>(&self, himc: P0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pfdwconversion, pfdwsentence).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<P0>(&self, hwnd: P0) -> ::windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultIMEWnd)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionA<P0>(&self, hkl: P0, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetDescriptionA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szdescription), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetDescriptionW<P0>(&self, hkl: P0, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetDescriptionW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szdescription), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetGuideLineA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, ::core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetGuideLineW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetGuideLineW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, dwbuflen, ::core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameA<P0>(&self, hkl: P0, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetIMEFileNameA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szfilename), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetIMEFileNameW<P0>(&self, hkl: P0, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetIMEFileNameW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), ubuflen, ::core::mem::transmute(szfilename), pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<P0>(&self, himc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<P0>(&self, hkl: P0, fdwindex: u32) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), fdwindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleA<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).GetRegisterWordStyleW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<P0>(&self, himc: P0) -> ::windows_core::Result<super::super::super::Foundation::POINT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<P0>(&self, hwnd: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVirtualKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> ::windows_core::Result<super::super::TextServices::HKL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEA)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> ::windows_core::Result<super::super::TextServices::HKL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallIMEW)(::windows_core::Interface::as_raw(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<P0>(&self, hkl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).IsIME)(::windows_core::Interface::as_raw(self), hkl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).IsUIMessageA)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).IsUIMessageW)(::windows_core::Interface::as_raw(self), hwndime.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).NotifyIME)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn RegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<P0, P1>(&self, hwnd: P0, himc: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).ReleaseContext)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<P0>(&self, himc: P0, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCandidateWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcandidate).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionFontA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionFontW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), plf).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionStringA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionStringW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<P0>(&self, himc: P0, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetCompositionWindow)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pcompform).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<P0>(&self, himc: P0, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetConversionStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fdwconversion, fdwsentence).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<P0, P1>(&self, himc: P0, fopen: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetOpenStatus)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<P0>(&self, himc: P0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).SetStatusWindowPos)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), pptpos).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<P0>(&self, hwnd: P0, dwhotkeyid: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SimulateHotKey)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), dwhotkeyid).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterWordA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szunregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn UnregisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UnregisterWordW)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), szreading.into_param().abi(), dwstyle, szunregister.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GenerateMessage<P0>(&self, himc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GenerateMessage)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn LockIMC<P0>(&self, himc: P0) -> ::windows_core::Result<*mut INPUTCONTEXT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LockIMC)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMC<P0>(&self, himc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).UnlockIMC)(::windows_core::Interface::as_raw(self), himc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCLockCount<P0>(&self, himc: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCLockCount)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateIMCC(&self, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateIMCC)(::windows_core::Interface::as_raw(self), dwsize, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyIMCC<P0>(&self, himcc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        (::windows_core::Interface::vtable(self).DestroyIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn LockIMCC<P0>(&self, himcc: P0, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        (::windows_core::Interface::vtable(self).LockIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), ppv).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMCC<P0>(&self, himcc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        (::windows_core::Interface::vtable(self).UnlockIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ReSizeIMCC<P0>(&self, himcc: P0, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReSizeIMCC)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), dwsize, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCSize<P0>(&self, himcc: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCCSize)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCLockCount<P0>(&self, himcc: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMCC>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIMCCLockCount)(::windows_core::Interface::as_raw(self), himcc.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHotKey)(::windows_core::Interface::as_raw(self), dwhotkeyid, pumodifiers, puvkey, phkl).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn SetHotKey<P0>(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        (::windows_core::Interface::vtable(self).SetHotKey)(::windows_core::Interface::as_raw(self), dwhotkeyid, umodifiers, uvkey, hkl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftKeyboard<P0>(&self, utype: u32, howner: P0, x: i32, y: i32) -> ::windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSoftKeyboard)(::windows_core::Interface::as_raw(self), utype, howner.into_param().abi(), x, y, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroySoftKeyboard<P0>(&self, hsoftkbdwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).DestroySoftKeyboard)(::windows_core::Interface::as_raw(self), hsoftkbdwnd.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowSoftKeyboard<P0>(&self, hsoftkbdwnd: P0, ncmdshow: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).ShowSoftKeyboard)(::windows_core::Interface::as_raw(self), hsoftkbdwnd.into_param().abi(), ncmdshow).ok()
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<P0>(&self, hkl: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCodePageA)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_TextServices\"`"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<P0>(&self, hkl: P0) -> ::windows_core::Result<u16>
    where
        P0: ::windows_core::IntoParam<super::super::TextServices::HKL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLangId)(::windows_core::Interface::as_raw(self), hkl.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).KeybdEvent)(::windows_core::Interface::as_raw(self), lgidime, bvk, bscan, dwflags, dwextrainfo).ok()
    }
    pub unsafe fn LockModal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockModal)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnlockModal(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnlockModal)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<P0, P1>(&self, hwnd: P0, himc: P1, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).AssociateContextEx)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), himc.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableIME)(::windows_core::Interface::as_raw(self), idthread).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).GetImeMenuItemsW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumInputContext)(::windows_core::Interface::as_raw(self), idthread, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageA<P0, P1, P2>(&self, himc: P0, wparam: P1, lparam: P2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestMessageA)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageW<P0, P1, P2>(&self, himc: P0, wparam: P1, lparam: P2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequestMessageW)(::windows_core::Interface::as_raw(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCA<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SendIMCA)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), umsg, wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCW<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SendIMCW)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), umsg, wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsSleeping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSleeping)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIMMIME, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActiveIMMIME {
    type Vtable = IActiveIMMIME_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIMMIME {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c03411_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMIME_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub ConfigureIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))]
    ConfigureIMEW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyContext: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub EnumRegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    EnumRegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub EscapeW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    EscapeW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCandidateListCountW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCandidateListCountW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetCompositionWindow: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetContext: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub GetConversionListW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))]
    GetConversionListW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetConversionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultIMEWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultIMEWnd: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetDescriptionW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetDescriptionW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetGuideLineW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetGuideLineW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetIMEFileNameW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetIMEFileNameW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetOpenStatus: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetRegisterWordStyleW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub GetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    GetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVirtualKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub InstallIMEW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub IsIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    IsIME: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUIMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUIMessageW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub NotifyIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    NotifyIME: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub RegisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    RegisterWordW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub ReleaseContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    ReleaseContext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCandidateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCandidateWindow: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub SetCompositionFontW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    SetCompositionFontW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringA: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetCompositionStringW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetCompositionStringW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetCompositionWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetCompositionWindow: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub SetConversionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    SetConversionStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetOpenStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetOpenStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub SetStatusWindowPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    SetStatusWindowPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulateHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulateHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub UnregisterWordW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    UnregisterWordW: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GenerateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GenerateMessage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub LockIMC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    LockIMC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub UnlockIMC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    UnlockIMC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCLockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCLockCount: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub CreateIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    CreateIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub DestroyIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    DestroyIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub LockIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    LockIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub UnlockIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    UnlockIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub ReSizeIMCC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    ReSizeIMCC: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCCSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCCSize: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetIMCCLockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetIMCCLockCount: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetHotKey: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub SetHotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    SetHotKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSoftKeyboard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestroySoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestroySoftKeyboard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowSoftKeyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowSoftKeyboard: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetCodePageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_TextServices")]
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))]
    GetLangId: usize,
    pub KeybdEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::HRESULT,
    pub LockModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnlockModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub AssociateContextEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    AssociateContextEx: usize,
    pub DisableIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsA: usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub GetImeMenuItemsW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub RequestMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    RequestMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub RequestMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))]
    RequestMessageW: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SendIMCA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendIMCA: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SendIMCW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendIMCW: usize,
    pub IsSleeping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIMMMessagePumpOwner(::windows_core::IUnknown);
impl IActiveIMMMessagePumpOwner {
    pub unsafe fn Start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTranslateMessage)(::windows_core::Interface::as_raw(self), pmsg).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Resume(&self, dwcookie: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIMMMessagePumpOwner, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActiveIMMMessagePumpOwner {
    type Vtable = IActiveIMMMessagePumpOwner_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIMMMessagePumpOwner {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5cf2cfa_8aeb_11d1_9364_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMMessagePumpOwner_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub OnTranslateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    OnTranslateMessage: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActiveIMMRegistrar(::windows_core::IUnknown);
impl IActiveIMMRegistrar {
    pub unsafe fn RegisterIME<P0, P1>(&self, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: P0, pszdesc: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).RegisterIME)(::windows_core::Interface::as_raw(self), rclsid, lgid, psziconfile.into_param().abi(), pszdesc.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterIME(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterIME)(::windows_core::Interface::as_raw(self), rclsid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IActiveIMMRegistrar, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActiveIMMRegistrar {
    type Vtable = IActiveIMMRegistrar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActiveIMMRegistrar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3458082_bd00_11d1_939b_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMRegistrar_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: ::windows_core::PCWSTR, pszdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub UnregisterIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumInputContext(::windows_core::IUnknown);
impl IEnumInputContext {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumInputContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Next(&self, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ulcount, rginputcontext, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEnumInputContext, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumInputContext {
    type Vtable = IEnumInputContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumInputContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09b5eab0_f997_11d1_93d4_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumInputContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Globalization")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumRegisterWordA(::windows_core::IUnknown);
impl IEnumRegisterWordA {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRegisterWordA> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ulcount, rgregisterword, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEnumRegisterWordA, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumRegisterWordA {
    type Vtable = IEnumRegisterWordA_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumRegisterWordA {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c03412_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordA_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumRegisterWordW(::windows_core::IUnknown);
impl IEnumRegisterWordW {
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ulcount, rgregisterword, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IEnumRegisterWordW, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumRegisterWordW {
    type Vtable = IEnumRegisterWordW_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumRegisterWordW {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4955dd31_b159_11d0_8fcf_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordW_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFEClassFactory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFEClassFactory {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).base__.CreateInstance)(::windows_core::Interface::as_raw(self), punkouter.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LockServer<P0>(&self, flock: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.LockServer)(::windows_core::Interface::as_raw(self), flock.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IFEClassFactory, ::windows_core::IUnknown, super::super::super::System::Com::IClassFactory);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IFEClassFactory {
    type Vtable = IFEClassFactory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IFEClassFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFEClassFactory_Vtbl {
    pub base__: super::super::super::System::Com::IClassFactory_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFECommon(::windows_core::IUnknown);
impl IFECommon {
    pub unsafe fn IsDefaultIME(&self, szname: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsDefaultIME)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(szname.as_ptr()), szname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetDefaultIME(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultIME)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeWordRegDialog)(::windows_core::Interface::as_raw(self), pimedlg).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InvokeDictToolDialog)(::windows_core::Interface::as_raw(self), pimedlg).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFECommon, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFECommon {
    type Vtable = IFECommon_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFECommon {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7151_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFECommon_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsDefaultIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCSTR, cszname: i32) -> ::windows_core::HRESULT,
    pub SetDefaultIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeWordRegDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeWordRegDialog: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeDictToolDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeDictToolDialog: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFEDictionary(::windows_core::IUnknown);
impl IFEDictionary {
    pub unsafe fn Open(&self, pchdictpath: ::core::option::Option<&mut [u8; 260]>, pshf: *mut IMESHF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pchdictpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pshf).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHeader(&self, pchdictpath: ::core::option::Option<&mut [u8; 260]>, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHeader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pchdictpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pshf, pjfmt, pultype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayProperty<P0>(&self, hwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).DisplayProperty)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPosTable)(::windows_core::Interface::as_raw(self), prgpostbl, pcpostbl).ok()
    }
    pub unsafe fn GetWords<P0, P1, P2>(&self, pwchfirst: P0, pwchlast: P1, pwchdisplay: P2, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetWords)(::windows_core::Interface::as_raw(self), pwchfirst.into_param().abi(), pwchlast.into_param().abi(), pwchdisplay.into_param().abi(), ulpos, ulselect, ulwordsrc, pchbuffer, cbbuffer, pcwrd).ok()
    }
    pub unsafe fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextWords)(::windows_core::Interface::as_raw(self), pchbuffer, cbbuffer, pcwrd).ok()
    }
    pub unsafe fn Create<P0>(&self, pchdictpath: P0, pshf: *mut IMESHF) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), pchdictpath.into_param().abi(), pshf).ok()
    }
    pub unsafe fn SetHeader(&self, pshf: *mut IMESHF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeader)(::windows_core::Interface::as_raw(self), pshf).ok()
    }
    pub unsafe fn ExistWord(&self, pwrd: *mut IMEWRD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExistWord)(::windows_core::Interface::as_raw(self), pwrd).ok()
    }
    pub unsafe fn ExistDependency(&self, pdp: *mut IMEDP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExistDependency)(::windows_core::Interface::as_raw(self), pdp).ok()
    }
    pub unsafe fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterWord)(::windows_core::Interface::as_raw(self), reg, pwrd).ok()
    }
    pub unsafe fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterDependency)(::windows_core::Interface::as_raw(self), reg, pdp).ok()
    }
    pub unsafe fn GetDependencies<P0, P1, P2, P3>(&self, pwchkakarireading: P0, pwchkakaridisplay: P1, ulkakaripos: u32, pwchukereading: P2, pwchukedisplay: P3, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetDependencies)(::windows_core::Interface::as_raw(self), pwchkakarireading.into_param().abi(), pwchkakaridisplay.into_param().abi(), ulkakaripos, pwchukereading.into_param().abi(), pwchukedisplay.into_param().abi(), ulukepos, jrel, ulwordsrc, pchbuffer, cbbuffer, pcdp).ok()
    }
    pub unsafe fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NextDependencies)(::windows_core::Interface::as_raw(self), pchbuffer, cbbuffer, pcdp).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertFromOldMSIME<P0>(&self, pchdic: P0, pfnlog: PFNLOG, reg: IMEREG) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).ConvertFromOldMSIME)(::windows_core::Interface::as_raw(self), pchdic.into_param().abi(), pfnlog, reg).ok()
    }
    pub unsafe fn ConvertFromUserToSys(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConvertFromUserToSys)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFEDictionary, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFEDictionary {
    type Vtable = IFEDictionary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFEDictionary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7153_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFEDictionary_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayProperty: usize,
    pub GetPosTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchfirst: ::windows_core::PCWSTR, pwchlast: ::windows_core::PCWSTR, pwchdisplay: ::windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT,
    pub NextWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PCSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows_core::HRESULT,
    pub ExistWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT,
    pub ExistDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows_core::HRESULT,
    pub RegisterWord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT,
    pub RegisterDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchkakarireading: ::windows_core::PCWSTR, pwchkakaridisplay: ::windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: ::windows_core::PCWSTR, pwchukedisplay: ::windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT,
    pub NextDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConvertFromOldMSIME: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdic: ::windows_core::PCSTR, pfnlog: PFNLOG, reg: IMEREG) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConvertFromOldMSIME: usize,
    pub ConvertFromUserToSys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFELanguage(::windows_core::IUnknown);
impl IFELanguage {
    pub unsafe fn Open(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJMorphResult<P0>(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: P0, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetJMorphResult)(::windows_core::Interface::as_raw(self), dwrequest, dwcmode, cwchinput, pwchinput.into_param().abi(), pfcinfo, ppresult).ok()
    }
    pub unsafe fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConversionModeCaps)(::windows_core::Interface::as_raw(self), pdwcaps).ok()
    }
    pub unsafe fn GetPhonetic<P0>(&self, string: P0, start: i32, length: i32, phonetic: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetPhonetic)(::windows_core::Interface::as_raw(self), string.into_param().abi(), start, length, ::core::mem::transmute(phonetic)).ok()
    }
    pub unsafe fn GetConversion<P0>(&self, string: P0, start: i32, length: i32, result: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetConversion)(::windows_core::Interface::as_raw(self), string.into_param().abi(), start, length, ::core::mem::transmute(result)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFELanguage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFELanguage {
    type Vtable = IFELanguage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFELanguage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x019f7152_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFELanguage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetJMorphResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: ::windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::HRESULT,
    pub GetConversionModeCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows_core::HRESULT,
    pub GetPhonetic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::std::mem::MaybeUninit<::windows_core::BSTR>, start: i32, length: i32, phonetic: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetConversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::std::mem::MaybeUninit<::windows_core::BSTR>, start: i32, length: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImePad(::windows_core::IUnknown);
impl IImePad {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Request<P0, P1, P2>(&self, piimepadapplet: P0, reqid: IME_PAD_REQUEST_FLAGS, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IImePadApplet>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).Request)(::windows_core::Interface::as_raw(self), piimepadapplet.into_param().abi(), reqid.0 as _, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IImePad, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImePad {
    type Vtable = IImePad_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImePad {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643a_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePad_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piimepadapplet: *mut ::core::ffi::c_void, reqid: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Request: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImePadApplet(::windows_core::IUnknown);
impl IImePadApplet {
    pub unsafe fn Initialize<P0>(&self, lpiimepad: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), lpiimepad.into_param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Terminate)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAppletConfig)(::windows_core::Interface::as_raw(self), lpappletcfg).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateUI<P0>(&self, hwndparent: P0, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).CreateUI)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), lpimeappletui).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpimepad: P0, notify: i32, wparam: P1, lparam: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), lpimepad.into_param().abi(), notify, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IImePadApplet, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImePadApplet {
    type Vtable = IImePadApplet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImePadApplet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643b_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePadApplet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetAppletConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetAppletConfig: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notify: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImePlugInDictDictionaryList(::windows_core::IUnknown);
impl IImePlugInDictDictionaryList {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDictionariesInUse)(::windows_core::Interface::as_raw(self), prgdictionaryguid, prgdatecreated, prgfencrypted).ok()
    }
    pub unsafe fn DeleteDictionary<P0>(&self, bstrdictionaryguid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteDictionary)(::windows_core::Interface::as_raw(self), bstrdictionaryguid.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IImePlugInDictDictionaryList, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImePlugInDictDictionaryList {
    type Vtable = IImePlugInDictDictionaryList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImePlugInDictDictionaryList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98752974_b0a6_489b_8f6f_bff3769c8eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePlugInDictDictionaryList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDictionariesInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDictionariesInUse: usize,
    pub DeleteDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImeSpecifyApplets(::windows_core::IUnknown);
impl IImeSpecifyApplets {
    pub unsafe fn GetAppletIIDList(&self, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAppletIIDList)(::windows_core::Interface::as_raw(self), refiid, lpiidlist).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IImeSpecifyApplets, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImeSpecifyApplets {
    type Vtable = IImeSpecifyApplets_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImeSpecifyApplets {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8e643c_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImeSpecifyApplets_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAppletIIDList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::HRESULT,
}
pub const ATTR_CONVERTED: u32 = 2u32;
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
pub const ATTR_INPUT: u32 = 0u32;
pub const ATTR_INPUT_ERROR: u32 = 4u32;
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
pub const CATID_MSIME_IImePadApplet: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7566cad1_4ec9_4478_9fe9_8ed766619edf);
pub const CATID_MSIME_IImePadApplet1000: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe081e1d6_2389_43cb_b66f_609f823d9f9c);
pub const CATID_MSIME_IImePadApplet1200: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa47fb5fc_7d15_4223_a789_b781bf9ae667);
pub const CATID_MSIME_IImePadApplet900: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaae51bf_5e5b_4a1d_8de1_17c1d9e1728d);
pub const CATID_MSIME_IImePadApplet_VER7: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a0f8e31_c3ee_11d1_afef_00805f0c8b6d);
pub const CATID_MSIME_IImePadApplet_VER80: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56f7a792_fef1_11d3_8463_00c04f7a06e5);
pub const CATID_MSIME_IImePadApplet_VER81: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x656520b0_bb88_11d4_84c0_00c04f7a06e5);
pub const CActiveIMM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4955dd33_b159_11d0_8fcf_00aa006bcc59);
pub const CFS_CANDIDATEPOS: u32 = 64u32;
pub const CFS_DEFAULT: u32 = 0u32;
pub const CFS_EXCLUDE: u32 = 128u32;
pub const CFS_FORCE_POSITION: u32 = 32u32;
pub const CFS_POINT: u32 = 2u32;
pub const CFS_RECT: u32 = 1u32;
pub const CHARINFO_APPLETID_MASK: u32 = 4278190080u32;
pub const CHARINFO_CHARID_MASK: u32 = 65535u32;
pub const CHARINFO_FEID_MASK: u32 = 15728640u32;
pub const CLSID_ImePlugInDictDictionaryList_CHS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bf0129b_5bef_4de4_9b0b_5edb66ac2fa6);
pub const CLSID_ImePlugInDictDictionaryList_JPN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fe2776b_b0f9_4396_b5fc_e9d4cf1ec195);
pub const CLSID_VERSION_DEPENDENT_MSIME_JAPANESE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a91029e_aa49_471b_aee7_7d332785660d);
pub const CPS_CANCEL: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(4u32);
pub const CPS_COMPLETE: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(1u32);
pub const CPS_CONVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(2u32);
pub const CPS_REVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(3u32);
pub const CS_INSERTCHAR: u32 = 8192u32;
pub const CS_NOMOVECARET: u32 = 16384u32;
pub const E_LARGEINPUT: u32 = 51u32;
pub const E_NOCAND: u32 = 48u32;
pub const E_NOTENOUGH_BUFFER: u32 = 49u32;
pub const E_NOTENOUGH_WDD: u32 = 50u32;
pub const FEID_CHINESE_HONGKONG: u32 = 3u32;
pub const FEID_CHINESE_SIMPLIFIED: u32 = 2u32;
pub const FEID_CHINESE_SINGAPORE: u32 = 4u32;
pub const FEID_CHINESE_TRADITIONAL: u32 = 1u32;
pub const FEID_JAPANESE: u32 = 5u32;
pub const FEID_KOREAN: u32 = 6u32;
pub const FEID_KOREAN_JOHAB: u32 = 7u32;
pub const FEID_NONE: u32 = 0u32;
pub const FELANG_CLMN_FIXD: u32 = 32u32;
pub const FELANG_CLMN_FIXR: u32 = 16u32;
pub const FELANG_CLMN_NOPBREAK: u32 = 8u32;
pub const FELANG_CLMN_NOWBREAK: u32 = 2u32;
pub const FELANG_CLMN_PBREAK: u32 = 4u32;
pub const FELANG_CLMN_WBREAK: u32 = 1u32;
pub const FELANG_CMODE_AUTOMATIC: u32 = 134217728u32;
pub const FELANG_CMODE_BESTFIRST: u32 = 16384u32;
pub const FELANG_CMODE_BOPOMOFO: u32 = 64u32;
pub const FELANG_CMODE_CONVERSATION: u32 = 536870912u32;
pub const FELANG_CMODE_FULLWIDTHOUT: u32 = 32u32;
pub const FELANG_CMODE_HALFWIDTHOUT: u32 = 16u32;
pub const FELANG_CMODE_HANGUL: u32 = 128u32;
pub const FELANG_CMODE_HIRAGANAOUT: u32 = 0u32;
pub const FELANG_CMODE_KATAKANAOUT: u32 = 8u32;
pub const FELANG_CMODE_MERGECAND: u32 = 4096u32;
pub const FELANG_CMODE_MONORUBY: u32 = 2u32;
pub const FELANG_CMODE_NAME: u32 = 268435456u32;
pub const FELANG_CMODE_NOINVISIBLECHAR: u32 = 1073741824u32;
pub const FELANG_CMODE_NONE: u32 = 16777216u32;
pub const FELANG_CMODE_NOPRUNING: u32 = 4u32;
pub const FELANG_CMODE_PHRASEPREDICT: u32 = 268435456u32;
pub const FELANG_CMODE_PINYIN: u32 = 256u32;
pub const FELANG_CMODE_PLAURALCLAUSE: u32 = 33554432u32;
pub const FELANG_CMODE_PRECONV: u32 = 512u32;
pub const FELANG_CMODE_RADICAL: u32 = 1024u32;
pub const FELANG_CMODE_ROMAN: u32 = 8192u32;
pub const FELANG_CMODE_SINGLECONVERT: u32 = 67108864u32;
pub const FELANG_CMODE_UNKNOWNREADING: u32 = 2048u32;
pub const FELANG_CMODE_USENOREVWORDS: u32 = 32768u32;
pub const FELANG_INVALD_PO: u32 = 65535u32;
pub const FELANG_REQ_CONV: u32 = 65536u32;
pub const FELANG_REQ_RECONV: u32 = 131072u32;
pub const FELANG_REQ_REV: u32 = 196608u32;
pub const FID_MSIME_KMS_DEL_KEYLIST: u32 = 4u32;
pub const FID_MSIME_KMS_FUNCDESC: u32 = 9u32;
pub const FID_MSIME_KMS_GETMAP: u32 = 6u32;
pub const FID_MSIME_KMS_GETMAPFAST: u32 = 11u32;
pub const FID_MSIME_KMS_GETMAPSEAMLESS: u32 = 10u32;
pub const FID_MSIME_KMS_INIT: u32 = 2u32;
pub const FID_MSIME_KMS_INVOKE: u32 = 7u32;
pub const FID_MSIME_KMS_NOTIFY: u32 = 5u32;
pub const FID_MSIME_KMS_SETMAP: u32 = 8u32;
pub const FID_MSIME_KMS_TERM: u32 = 3u32;
pub const FID_MSIME_KMS_VERSION: u32 = 1u32;
pub const FID_MSIME_VERSION: u32 = 0u32;
pub const FID_RECONVERT_VERSION: u32 = 268435456u32;
pub const GCL_CONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(1u32);
pub const GCL_REVERSECONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(2u32);
pub const GCL_REVERSE_LENGTH: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(3u32);
pub const GCSEX_CANCELRECONVERT: u32 = 268435456u32;
pub const GCS_COMPATTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(16u32);
pub const GCS_COMPCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(32u32);
pub const GCS_COMPREADATTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(2u32);
pub const GCS_COMPREADCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(4u32);
pub const GCS_COMPREADSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(1u32);
pub const GCS_COMPSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(8u32);
pub const GCS_CURSORPOS: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(128u32);
pub const GCS_DELTASTART: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(256u32);
pub const GCS_RESULTCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(4096u32);
pub const GCS_RESULTREADCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(1024u32);
pub const GCS_RESULTREADSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(512u32);
pub const GCS_RESULTSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(2048u32);
pub const GGL_INDEX: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(2u32);
pub const GGL_LEVEL: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(1u32);
pub const GGL_PRIVATE: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(4u32);
pub const GGL_STRING: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(3u32);
pub const GL_ID_CANNOTSAVE: u32 = 17u32;
pub const GL_ID_CHOOSECANDIDATE: u32 = 40u32;
pub const GL_ID_INPUTCODE: u32 = 38u32;
pub const GL_ID_INPUTRADICAL: u32 = 37u32;
pub const GL_ID_INPUTREADING: u32 = 36u32;
pub const GL_ID_INPUTSYMBOL: u32 = 39u32;
pub const GL_ID_NOCONVERT: u32 = 32u32;
pub const GL_ID_NODICTIONARY: u32 = 16u32;
pub const GL_ID_NOMODULE: u32 = 1u32;
pub const GL_ID_PRIVATE_FIRST: u32 = 32768u32;
pub const GL_ID_PRIVATE_LAST: u32 = 65535u32;
pub const GL_ID_READINGCONFLICT: u32 = 35u32;
pub const GL_ID_REVERSECONVERSION: u32 = 41u32;
pub const GL_ID_TOOMANYSTROKE: u32 = 34u32;
pub const GL_ID_TYPINGERROR: u32 = 33u32;
pub const GL_ID_UNKNOWN: u32 = 0u32;
pub const GL_LEVEL_ERROR: u32 = 2u32;
pub const GL_LEVEL_FATAL: u32 = 1u32;
pub const GL_LEVEL_INFORMATION: u32 = 4u32;
pub const GL_LEVEL_NOGUIDELINE: u32 = 0u32;
pub const GL_LEVEL_WARNING: u32 = 3u32;
pub const IACE_CHILDREN: u32 = 1u32;
pub const IACE_DEFAULT: u32 = 16u32;
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
pub const IFEC_S_ALREADY_DEFAULT: ::windows_core::HRESULT = ::windows_core::HRESULT(291840i32);
pub const IFED_ACTIVE_DICT: IMEFMT = IMEFMT(13i32);
pub const IFED_ATOK10: IMEFMT = IMEFMT(15i32);
pub const IFED_ATOK9: IMEFMT = IMEFMT(14i32);
pub const IFED_E_INVALID_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192063i32);
pub const IFED_E_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192064i32);
pub const IFED_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192057i32);
pub const IFED_E_NOT_USER_DIC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192058i32);
pub const IFED_E_NO_ENTRY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192060i32);
pub const IFED_E_OPEN_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192062i32);
pub const IFED_E_REGISTER_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192053i32);
pub const IFED_E_REGISTER_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192059i32);
pub const IFED_E_REGISTER_ILLEGAL_POS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192055i32);
pub const IFED_E_REGISTER_IMPROPER_WORD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192054i32);
pub const IFED_E_USER_COMMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192056i32);
pub const IFED_E_WRITE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147192061i32);
pub const IFED_MSIME2_BIN_SYSTEM: IMEFMT = IMEFMT(1i32);
pub const IFED_MSIME2_BIN_USER: IMEFMT = IMEFMT(2i32);
pub const IFED_MSIME2_TEXT_USER: IMEFMT = IMEFMT(3i32);
pub const IFED_MSIME95_BIN_SYSTEM: IMEFMT = IMEFMT(4i32);
pub const IFED_MSIME95_BIN_USER: IMEFMT = IMEFMT(5i32);
pub const IFED_MSIME95_TEXT_USER: IMEFMT = IMEFMT(6i32);
pub const IFED_MSIME97_BIN_SYSTEM: IMEFMT = IMEFMT(7i32);
pub const IFED_MSIME97_BIN_USER: IMEFMT = IMEFMT(8i32);
pub const IFED_MSIME97_TEXT_USER: IMEFMT = IMEFMT(9i32);
pub const IFED_MSIME98_BIN_SYSTEM: IMEFMT = IMEFMT(10i32);
pub const IFED_MSIME98_BIN_USER: IMEFMT = IMEFMT(11i32);
pub const IFED_MSIME98_SYSTEM_CE: IMEFMT = IMEFMT(20i32);
pub const IFED_MSIME98_TEXT_USER: IMEFMT = IMEFMT(12i32);
pub const IFED_MSIME_BIN_SYSTEM: IMEFMT = IMEFMT(21i32);
pub const IFED_MSIME_BIN_USER: IMEFMT = IMEFMT(22i32);
pub const IFED_MSIME_TEXT_USER: IMEFMT = IMEFMT(23i32);
pub const IFED_NEC_AI_: IMEFMT = IMEFMT(16i32);
pub const IFED_PIME2_BIN_STANDARD_SYSTEM: IMEFMT = IMEFMT(26i32);
pub const IFED_PIME2_BIN_SYSTEM: IMEFMT = IMEFMT(25i32);
pub const IFED_PIME2_BIN_USER: IMEFMT = IMEFMT(24i32);
pub const IFED_POS_ADJECTIVE: u32 = 4u32;
pub const IFED_POS_ADJECTIVE_VERB: u32 = 8u32;
pub const IFED_POS_ADNOUN: u32 = 32u32;
pub const IFED_POS_ADVERB: u32 = 16u32;
pub const IFED_POS_AFFIX: u32 = 1536u32;
pub const IFED_POS_ALL: u32 = 131071u32;
pub const IFED_POS_AUXILIARY_VERB: u32 = 32768u32;
pub const IFED_POS_CONJUNCTION: u32 = 64u32;
pub const IFED_POS_DEPENDENT: u32 = 114688u32;
pub const IFED_POS_IDIOMS: u32 = 4096u32;
pub const IFED_POS_INDEPENDENT: u32 = 255u32;
pub const IFED_POS_INFLECTIONALSUFFIX: u32 = 256u32;
pub const IFED_POS_INTERJECTION: u32 = 128u32;
pub const IFED_POS_NONE: u32 = 0u32;
pub const IFED_POS_NOUN: u32 = 1u32;
pub const IFED_POS_PARTICLE: u32 = 16384u32;
pub const IFED_POS_PREFIX: u32 = 512u32;
pub const IFED_POS_SUB_VERB: u32 = 65536u32;
pub const IFED_POS_SUFFIX: u32 = 1024u32;
pub const IFED_POS_SYMBOLS: u32 = 8192u32;
pub const IFED_POS_TANKANJI: u32 = 2048u32;
pub const IFED_POS_VERB: u32 = 2u32;
pub const IFED_REG_ALL: u32 = 7u32;
pub const IFED_REG_AUTO: u32 = 2u32;
pub const IFED_REG_DEL: IMEREG = IMEREG(2i32);
pub const IFED_REG_GRAMMAR: u32 = 4u32;
pub const IFED_REG_HEAD: IMEREG = IMEREG(0i32);
pub const IFED_REG_NONE: u32 = 0u32;
pub const IFED_REG_TAIL: IMEREG = IMEREG(1i32);
pub const IFED_REG_USER: u32 = 1u32;
pub const IFED_REL_ALL: IMEREL = IMEREL(24i32);
pub const IFED_REL_DE: IMEREL = IMEREL(5i32);
pub const IFED_REL_FUKU_YOUGEN: IMEREL = IMEREL(12i32);
pub const IFED_REL_GA: IMEREL = IMEREL(2i32);
pub const IFED_REL_HE: IMEREL = IMEREL(9i32);
pub const IFED_REL_IDEOM: IMEREL = IMEREL(11i32);
pub const IFED_REL_KARA: IMEREL = IMEREL(7i32);
pub const IFED_REL_KEIDOU1_YOUGEN: IMEREL = IMEREL(14i32);
pub const IFED_REL_KEIDOU2_YOUGEN: IMEREL = IMEREL(15i32);
pub const IFED_REL_KEIYOU_TARU_YOUGEN: IMEREL = IMEREL(21i32);
pub const IFED_REL_KEIYOU_TO_YOUGEN: IMEREL = IMEREL(20i32);
pub const IFED_REL_KEIYOU_YOUGEN: IMEREL = IMEREL(13i32);
pub const IFED_REL_MADE: IMEREL = IMEREL(8i32);
pub const IFED_REL_NI: IMEREL = IMEREL(4i32);
pub const IFED_REL_NO: IMEREL = IMEREL(1i32);
pub const IFED_REL_NONE: IMEREL = IMEREL(0i32);
pub const IFED_REL_RENSOU: IMEREL = IMEREL(19i32);
pub const IFED_REL_RENTAI_MEI: IMEREL = IMEREL(18i32);
pub const IFED_REL_TAIGEN: IMEREL = IMEREL(16i32);
pub const IFED_REL_TO: IMEREL = IMEREL(10i32);
pub const IFED_REL_UNKNOWN1: IMEREL = IMEREL(22i32);
pub const IFED_REL_UNKNOWN2: IMEREL = IMEREL(23i32);
pub const IFED_REL_WO: IMEREL = IMEREL(3i32);
pub const IFED_REL_YORI: IMEREL = IMEREL(6i32);
pub const IFED_REL_YOUGEN: IMEREL = IMEREL(17i32);
pub const IFED_SELECT_ALL: u32 = 15u32;
pub const IFED_SELECT_COMMENT: u32 = 8u32;
pub const IFED_SELECT_DISPLAY: u32 = 2u32;
pub const IFED_SELECT_NONE: u32 = 0u32;
pub const IFED_SELECT_POS: u32 = 4u32;
pub const IFED_SELECT_READING: u32 = 1u32;
pub const IFED_S_COMMENT_CHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(291331i32);
pub const IFED_S_EMPTY_DICTIONARY: ::windows_core::HRESULT = ::windows_core::HRESULT(291329i32);
pub const IFED_S_MORE_ENTRIES: ::windows_core::HRESULT = ::windows_core::HRESULT(291328i32);
pub const IFED_S_WORD_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(291330i32);
pub const IFED_TYPE_ALL: u32 = 31u32;
pub const IFED_TYPE_ENGLISH: u32 = 16u32;
pub const IFED_TYPE_GENERAL: u32 = 1u32;
pub const IFED_TYPE_NAMEPLACE: u32 = 2u32;
pub const IFED_TYPE_NONE: u32 = 0u32;
pub const IFED_TYPE_REVERSE: u32 = 8u32;
pub const IFED_TYPE_SPEECH: u32 = 4u32;
pub const IFED_UCT_MAX: IMEUCT = IMEUCT(4i32);
pub const IFED_UCT_NONE: IMEUCT = IMEUCT(0i32);
pub const IFED_UCT_STRING_SJIS: IMEUCT = IMEUCT(1i32);
pub const IFED_UCT_STRING_UNICODE: IMEUCT = IMEUCT(2i32);
pub const IFED_UCT_USER_DEFINED: IMEUCT = IMEUCT(3i32);
pub const IFED_UNKNOWN: IMEFMT = IMEFMT(0i32);
pub const IFED_VJE_20: IMEFMT = IMEFMT(19i32);
pub const IFED_WX_II: IMEFMT = IMEFMT(17i32);
pub const IFED_WX_III: IMEFMT = IMEFMT(18i32);
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
pub const IGIMII_CMODE: u32 = 1u32;
pub const IGIMII_CONFIGURE: u32 = 4u32;
pub const IGIMII_HELP: u32 = 16u32;
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
pub const IGIMII_OTHER: u32 = 32u32;
pub const IGIMII_SMODE: u32 = 2u32;
pub const IGIMII_TOOLS: u32 = 8u32;
pub const IMC_CLOSESTATUSWINDOW: u32 = 33u32;
pub const IMC_GETCANDIDATEPOS: u32 = 7u32;
pub const IMC_GETCOMPOSITIONFONT: u32 = 9u32;
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMC_GETSOFTKBDFONT: u32 = 17u32;
pub const IMC_GETSOFTKBDPOS: u32 = 19u32;
pub const IMC_GETSOFTKBDSUBTYPE: u32 = 21u32;
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15u32;
pub const IMC_OPENSTATUSWINDOW: u32 = 34u32;
pub const IMC_SETCANDIDATEPOS: u32 = 8u32;
pub const IMC_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12u32;
pub const IMC_SETCONVERSIONMODE: u32 = 2u32;
pub const IMC_SETOPENSTATUS: u32 = 6u32;
pub const IMC_SETSENTENCEMODE: u32 = 4u32;
pub const IMC_SETSOFTKBDDATA: u32 = 24u32;
pub const IMC_SETSOFTKBDFONT: u32 = 18u32;
pub const IMC_SETSOFTKBDPOS: u32 = 20u32;
pub const IMC_SETSOFTKBDSUBTYPE: u32 = 22u32;
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16u32;
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
pub const IMEKEYCTRL_UP: u32 = 1u32;
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
pub const IMEKMS_CANDIDATE: u32 = 6u32;
pub const IMEKMS_COMPOSITION: u32 = 1u32;
pub const IMEKMS_IMEOFF: u32 = 3u32;
pub const IMEKMS_INPTGL: u32 = 5u32;
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
pub const IMEKMS_SELECTION: u32 = 2u32;
pub const IMEKMS_TYPECAND: u32 = 7u32;
pub const IMEMENUITEM_STRING_SIZE: u32 = 80u32;
pub const IMEMOUSERET_NOTHANDLED: i32 = -1i32;
pub const IMEMOUSE_LDOWN: u32 = 1u32;
pub const IMEMOUSE_MDOWN: u32 = 4u32;
pub const IMEMOUSE_NONE: u32 = 0u32;
pub const IMEMOUSE_RDOWN: u32 = 2u32;
pub const IMEMOUSE_VERSION: u32 = 255u32;
pub const IMEMOUSE_WDOWN: u32 = 32u32;
pub const IMEMOUSE_WUP: u32 = 16u32;
pub const IMEPADCTRL_CARETBACKSPACE: u32 = 10u32;
pub const IMEPADCTRL_CARETBOTTOM: u32 = 9u32;
pub const IMEPADCTRL_CARETDELETE: u32 = 11u32;
pub const IMEPADCTRL_CARETLEFT: u32 = 6u32;
pub const IMEPADCTRL_CARETRIGHT: u32 = 7u32;
pub const IMEPADCTRL_CARETSET: u32 = 5u32;
pub const IMEPADCTRL_CARETTOP: u32 = 8u32;
pub const IMEPADCTRL_CLEARALL: u32 = 4u32;
pub const IMEPADCTRL_CONVERTALL: u32 = 1u32;
pub const IMEPADCTRL_DETERMINALL: u32 = 2u32;
pub const IMEPADCTRL_DETERMINCHAR: u32 = 3u32;
pub const IMEPADCTRL_INSERTFULLSPACE: u32 = 14u32;
pub const IMEPADCTRL_INSERTHALFSPACE: u32 = 15u32;
pub const IMEPADCTRL_INSERTSPACE: u32 = 13u32;
pub const IMEPADCTRL_OFFIME: u32 = 17u32;
pub const IMEPADCTRL_OFFPRECONVERSION: u32 = 19u32;
pub const IMEPADCTRL_ONIME: u32 = 16u32;
pub const IMEPADCTRL_ONPRECONVERSION: u32 = 18u32;
pub const IMEPADCTRL_PHONETICCANDIDATE: u32 = 20u32;
pub const IMEPADCTRL_PHRASEDELETE: u32 = 12u32;
pub const IMEPADREQ_CHANGESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4113u32);
pub const IMEPADREQ_CHANGESTRINGCANDIDATEINFO: u32 = 4111u32;
pub const IMEPADREQ_CHANGESTRINGINFO: u32 = 4115u32;
pub const IMEPADREQ_DELETESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4112u32);
pub const IMEPADREQ_FIRST: u32 = 4096u32;
pub const IMEPADREQ_FORCEIMEPADWINDOWSHOW: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4117u32);
pub const IMEPADREQ_GETAPPLETDATA: u32 = 4106u32;
pub const IMEPADREQ_GETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4121u32);
pub const IMEPADREQ_GETAPPLHWND: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4116u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4102u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRINGID: u32 = 4109u32;
pub const IMEPADREQ_GETCOMPOSITIONSTRINGINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4108u32);
pub const IMEPADREQ_GETCONVERSIONSTATUS: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4126u32);
pub const IMEPADREQ_GETCURRENTIMEINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4128u32);
pub const IMEPADREQ_GETCURRENTUILANGID: u32 = 4120u32;
pub const IMEPADREQ_GETDEFAULTUILANGID: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4119u32);
pub const IMEPADREQ_GETSELECTEDSTRING: u32 = 4103u32;
pub const IMEPADREQ_GETVERSION: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4127u32);
pub const IMEPADREQ_INSERTITEMCANDIDATE: u32 = 4099u32;
pub const IMEPADREQ_INSERTSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4097u32);
pub const IMEPADREQ_INSERTSTRINGCANDIDATE: u32 = 4098u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATEINFO: u32 = 4110u32;
pub const IMEPADREQ_INSERTSTRINGINFO: u32 = 4114u32;
pub const IMEPADREQ_ISAPPLETACTIVE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4123u32);
pub const IMEPADREQ_ISIMEPADWINDOWVISIBLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4124u32);
pub const IMEPADREQ_POSTMODALNOTIFY: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4118u32);
pub const IMEPADREQ_SENDCONTROL: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4100u32);
pub const IMEPADREQ_SENDKEYCONTROL: u32 = 4101u32;
pub const IMEPADREQ_SETAPPLETDATA: u32 = 4105u32;
pub const IMEPADREQ_SETAPPLETMINMAXSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4125u32);
pub const IMEPADREQ_SETAPPLETSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4104u32);
pub const IMEPADREQ_SETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4122u32);
pub const IMEPADREQ_SETTITLEFONT: u32 = 4107u32;
pub const IMEPN_ACTIVATE: u32 = 257u32;
pub const IMEPN_APPLYCAND: u32 = 267u32;
pub const IMEPN_APPLYCANDEX: u32 = 268u32;
pub const IMEPN_CONFIG: u32 = 264u32;
pub const IMEPN_FIRST: u32 = 256u32;
pub const IMEPN_HELP: u32 = 265u32;
pub const IMEPN_HIDE: u32 = 261u32;
pub const IMEPN_INACTIVATE: u32 = 258u32;
pub const IMEPN_QUERYCAND: u32 = 266u32;
pub const IMEPN_SETTINGCHANGED: u32 = 269u32;
pub const IMEPN_SHOW: u32 = 260u32;
pub const IMEPN_SIZECHANGED: u32 = 263u32;
pub const IMEPN_SIZECHANGING: u32 = 262u32;
pub const IMEPN_USER: u32 = 356u32;
pub const IMEVER_0310: u32 = 196618u32;
pub const IMEVER_0400: u32 = 262144u32;
pub const IME_CAND_CODE: u32 = 2u32;
pub const IME_CAND_MEANING: u32 = 3u32;
pub const IME_CAND_RADICAL: u32 = 4u32;
pub const IME_CAND_READ: u32 = 1u32;
pub const IME_CAND_STROKE: u32 = 5u32;
pub const IME_CAND_UNKNOWN: u32 = 0u32;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(16u32);
pub const IME_CHOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(17u32);
pub const IME_CHOTKEY_SYMBOL_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(18u32);
pub const IME_CMODE_ALPHANUMERIC: IME_CONVERSION_MODE = IME_CONVERSION_MODE(0u32);
pub const IME_CMODE_CHARCODE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(32u32);
pub const IME_CMODE_CHINESE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_EUDC: IME_CONVERSION_MODE = IME_CONVERSION_MODE(512u32);
pub const IME_CMODE_FIXED: IME_CONVERSION_MODE = IME_CONVERSION_MODE(2048u32);
pub const IME_CMODE_FULLSHAPE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(8u32);
pub const IME_CMODE_HANGEUL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_HANGUL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_HANJACONVERT: IME_CONVERSION_MODE = IME_CONVERSION_MODE(64u32);
pub const IME_CMODE_JAPANESE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_KATAKANA: IME_CONVERSION_MODE = IME_CONVERSION_MODE(2u32);
pub const IME_CMODE_LANGUAGE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(3u32);
pub const IME_CMODE_NATIVE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_NATIVESYMBOL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(128u32);
pub const IME_CMODE_NOCONVERSION: IME_CONVERSION_MODE = IME_CONVERSION_MODE(256u32);
pub const IME_CMODE_RESERVED: IME_CONVERSION_MODE = IME_CONVERSION_MODE(4026531840u32);
pub const IME_CMODE_ROMAN: IME_CONVERSION_MODE = IME_CONVERSION_MODE(16u32);
pub const IME_CMODE_SOFTKBD: IME_CONVERSION_MODE = IME_CONVERSION_MODE(128u32);
pub const IME_CMODE_SYMBOL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1024u32);
pub const IME_CONFIG_GENERAL: u32 = 1u32;
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
pub const IME_ESC_AUTOMATA: IME_ESCAPE = IME_ESCAPE(4105u32);
pub const IME_ESC_GETHELPFILENAME: IME_ESCAPE = IME_ESCAPE(4107u32);
pub const IME_ESC_GET_EUDC_DICTIONARY: IME_ESCAPE = IME_ESCAPE(4099u32);
pub const IME_ESC_HANJA_MODE: IME_ESCAPE = IME_ESCAPE(4104u32);
pub const IME_ESC_IME_NAME: IME_ESCAPE = IME_ESCAPE(4102u32);
pub const IME_ESC_MAX_KEY: IME_ESCAPE = IME_ESCAPE(4101u32);
pub const IME_ESC_PRIVATE_FIRST: IME_ESCAPE = IME_ESCAPE(2048u32);
pub const IME_ESC_PRIVATE_HOTKEY: IME_ESCAPE = IME_ESCAPE(4106u32);
pub const IME_ESC_PRIVATE_LAST: IME_ESCAPE = IME_ESCAPE(4095u32);
pub const IME_ESC_QUERY_SUPPORT: IME_ESCAPE = IME_ESCAPE(3u32);
pub const IME_ESC_RESERVED_FIRST: IME_ESCAPE = IME_ESCAPE(4u32);
pub const IME_ESC_RESERVED_LAST: IME_ESCAPE = IME_ESCAPE(2047u32);
pub const IME_ESC_SEQUENCE_TO_INTERNAL: IME_ESCAPE = IME_ESCAPE(4097u32);
pub const IME_ESC_SET_EUDC_DICTIONARY: IME_ESCAPE = IME_ESCAPE(4100u32);
pub const IME_ESC_STRING_BUFFER_SIZE: u32 = 80u32;
pub const IME_ESC_SYNC_HOTKEY: IME_ESCAPE = IME_ESCAPE(4103u32);
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256u32;
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287u32;
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512u32;
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543u32;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(513u32);
pub const IME_ITHOTKEY_RECONVERTSTRING: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(515u32);
pub const IME_ITHOTKEY_RESEND_RESULTSTR: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(512u32);
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(514u32);
pub const IME_JHOTKEY_CLOSE_OPEN: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(48u32);
pub const IME_KHOTKEY_ENGLISH: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(82u32);
pub const IME_KHOTKEY_HANJACONVERT: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(81u32);
pub const IME_KHOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(80u32);
pub const IME_PROP_ACCEPT_WIDE_VKEY: u32 = 32u32;
pub const IME_PROP_AT_CARET: u32 = 65536u32;
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144u32;
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576u32;
pub const IME_PROP_END_UNLOAD: u32 = 1u32;
pub const IME_PROP_IGNORE_UPKEYS: u32 = 4u32;
pub const IME_PROP_KBD_CHAR_FIRST: u32 = 2u32;
pub const IME_PROP_NEED_ALTKEY: u32 = 8u32;
pub const IME_PROP_NO_KEYS_ON_CLOSE: u32 = 16u32;
pub const IME_PROP_SPECIAL_UI: u32 = 131072u32;
pub const IME_PROP_UNICODE: u32 = 524288u32;
pub const IME_REGWORD_STYLE_EUDC: u32 = 1u32;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648u32;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295u32;
pub const IME_SMODE_AUTOMATIC: IME_SENTENCE_MODE = IME_SENTENCE_MODE(4u32);
pub const IME_SMODE_CONVERSATION: IME_SENTENCE_MODE = IME_SENTENCE_MODE(16u32);
pub const IME_SMODE_NONE: IME_SENTENCE_MODE = IME_SENTENCE_MODE(0u32);
pub const IME_SMODE_PHRASEPREDICT: IME_SENTENCE_MODE = IME_SENTENCE_MODE(8u32);
pub const IME_SMODE_PLAURALCLAUSE: IME_SENTENCE_MODE = IME_SENTENCE_MODE(1u32);
pub const IME_SMODE_RESERVED: IME_SENTENCE_MODE = IME_SENTENCE_MODE(61440u32);
pub const IME_SMODE_SINGLECONVERT: IME_SENTENCE_MODE = IME_SENTENCE_MODE(2u32);
pub const IME_SYSINFO_WINLOGON: u32 = 1u32;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(112u32);
pub const IME_THOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(113u32);
pub const IME_THOTKEY_SYMBOL_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(114u32);
pub const IME_UI_CLASS_NAME_SIZE: u32 = 16u32;
pub const IMFT_RADIOCHECK: u32 = 1u32;
pub const IMFT_SEPARATOR: u32 = 2u32;
pub const IMFT_SUBMENU: u32 = 4u32;
pub const IMMGWLP_IMC: u32 = 0u32;
pub const IMMGWL_IMC: u32 = 0u32;
pub const IMM_ERROR_GENERAL: i32 = -2i32;
pub const IMM_ERROR_NODATA: i32 = -1i32;
pub const IMN_CHANGECANDIDATE: u32 = 3u32;
pub const IMN_CLOSECANDIDATE: u32 = 4u32;
pub const IMN_CLOSESTATUSWINDOW: u32 = 1u32;
pub const IMN_GUIDELINE: u32 = 13u32;
pub const IMN_OPENCANDIDATE: u32 = 5u32;
pub const IMN_OPENSTATUSWINDOW: u32 = 2u32;
pub const IMN_PRIVATE: u32 = 14u32;
pub const IMN_SETCANDIDATEPOS: u32 = 9u32;
pub const IMN_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMN_SETCONVERSIONMODE: u32 = 6u32;
pub const IMN_SETOPENSTATUS: u32 = 8u32;
pub const IMN_SETSENTENCEMODE: u32 = 7u32;
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12u32;
pub const IMN_SOFTKBDDESTROYED: u32 = 17u32;
pub const IMR_CANDIDATEWINDOW: u32 = 2u32;
pub const IMR_COMPOSITIONFONT: u32 = 3u32;
pub const IMR_COMPOSITIONWINDOW: u32 = 1u32;
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5u32;
pub const IMR_DOCUMENTFEED: u32 = 7u32;
pub const IMR_QUERYCHARPOSITION: u32 = 6u32;
pub const IMR_RECONVERTSTRING: u32 = 4u32;
pub const INFOMASK_APPLY_CAND: u32 = 2u32;
pub const INFOMASK_APPLY_CAND_EX: u32 = 4u32;
pub const INFOMASK_BLOCK_CAND: u32 = 262144u32;
pub const INFOMASK_HIDE_CAND: u32 = 131072u32;
pub const INFOMASK_NONE: u32 = 0u32;
pub const INFOMASK_QUERY_CAND: u32 = 1u32;
pub const INFOMASK_STRING_FIX: u32 = 65536u32;
pub const INIT_COMPFORM: u32 = 16u32;
pub const INIT_CONVERSION: u32 = 2u32;
pub const INIT_LOGFONT: u32 = 8u32;
pub const INIT_SENTENCE: u32 = 4u32;
pub const INIT_SOFTKBDPOS: u32 = 32u32;
pub const INIT_STATUSWNDPOS: u32 = 1u32;
pub const IPACFG_CATEGORY: i32 = 262144i32;
pub const IPACFG_HELP: i32 = 2i32;
pub const IPACFG_LANG: i32 = 16i32;
pub const IPACFG_NONE: i32 = 0i32;
pub const IPACFG_PROPERTY: i32 = 1i32;
pub const IPACFG_TITLE: i32 = 65536i32;
pub const IPACFG_TITLEFONTFACE: i32 = 131072i32;
pub const IPACID_CHARLIST: u32 = 9u32;
pub const IPACID_EPWING: u32 = 7u32;
pub const IPACID_HANDWRITING: u32 = 2u32;
pub const IPACID_NONE: u32 = 0u32;
pub const IPACID_OCR: u32 = 8u32;
pub const IPACID_RADICALSEARCH: u32 = 4u32;
pub const IPACID_SOFTKEY: u32 = 1u32;
pub const IPACID_STROKESEARCH: u32 = 3u32;
pub const IPACID_SYMBOLSEARCH: u32 = 5u32;
pub const IPACID_USER: u32 = 256u32;
pub const IPACID_VOICE: u32 = 6u32;
pub const IPAWS_ENABLED: i32 = 1i32;
pub const IPAWS_HORIZONTALFIXED: i32 = 512i32;
pub const IPAWS_MAXHEIGHTFIXED: i32 = 8192i32;
pub const IPAWS_MAXSIZEFIXED: i32 = 12288i32;
pub const IPAWS_MAXWIDTHFIXED: i32 = 4096i32;
pub const IPAWS_MINHEIGHTFIXED: i32 = 131072i32;
pub const IPAWS_MINSIZEFIXED: i32 = 196608i32;
pub const IPAWS_MINWIDTHFIXED: i32 = 65536i32;
pub const IPAWS_SIZEFIXED: i32 = 768i32;
pub const IPAWS_SIZINGNOTIFY: i32 = 4i32;
pub const IPAWS_VERTICALFIXED: i32 = 256i32;
pub const ISC_SHOWUIALL: u32 = 3221225487u32;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15u32;
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1u32;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648u32;
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824u32;
pub const JPOS_1DAN: u32 = 213u32;
pub const JPOS_4DAN_HA: u32 = 212u32;
pub const JPOS_5DAN_AWA: u32 = 200u32;
pub const JPOS_5DAN_AWAUON: u32 = 209u32;
pub const JPOS_5DAN_BA: u32 = 206u32;
pub const JPOS_5DAN_GA: u32 = 202u32;
pub const JPOS_5DAN_KA: u32 = 201u32;
pub const JPOS_5DAN_KASOKUON: u32 = 210u32;
pub const JPOS_5DAN_MA: u32 = 207u32;
pub const JPOS_5DAN_NA: u32 = 205u32;
pub const JPOS_5DAN_RA: u32 = 208u32;
pub const JPOS_5DAN_RAHEN: u32 = 211u32;
pub const JPOS_5DAN_SA: u32 = 203u32;
pub const JPOS_5DAN_TA: u32 = 204u32;
pub const JPOS_BUPPIN: u32 = 122u32;
pub const JPOS_CHIMEI: u32 = 109u32;
pub const JPOS_CHIMEI_EKI: u32 = 117u32;
pub const JPOS_CHIMEI_GUN: u32 = 112u32;
pub const JPOS_CHIMEI_KEN: u32 = 111u32;
pub const JPOS_CHIMEI_KU: u32 = 113u32;
pub const JPOS_CHIMEI_KUNI: u32 = 110u32;
pub const JPOS_CHIMEI_MACHI: u32 = 115u32;
pub const JPOS_CHIMEI_MURA: u32 = 116u32;
pub const JPOS_CHIMEI_SHI: u32 = 114u32;
pub const JPOS_CLOSEBRACE: u32 = 911u32;
pub const JPOS_DAIMEISHI: u32 = 123u32;
pub const JPOS_DAIMEISHI_NINSHOU: u32 = 124u32;
pub const JPOS_DAIMEISHI_SHIJI: u32 = 125u32;
pub const JPOS_DOKURITSUGO: u32 = 903u32;
pub const JPOS_EIJI: u32 = 906u32;
pub const JPOS_FUKUSHI: u32 = 500u32;
pub const JPOS_FUKUSHI_DA: u32 = 504u32;
pub const JPOS_FUKUSHI_NANO: u32 = 503u32;
pub const JPOS_FUKUSHI_NI: u32 = 502u32;
pub const JPOS_FUKUSHI_SAHEN: u32 = 501u32;
pub const JPOS_FUKUSHI_TO: u32 = 505u32;
pub const JPOS_FUKUSHI_TOSURU: u32 = 506u32;
pub const JPOS_FUTEIGO: u32 = 904u32;
pub const JPOS_HUKUSIMEISHI: u32 = 104u32;
pub const JPOS_JINMEI: u32 = 106u32;
pub const JPOS_JINMEI_MEI: u32 = 108u32;
pub const JPOS_JINMEI_SEI: u32 = 107u32;
pub const JPOS_KANDOUSHI: u32 = 670u32;
pub const JPOS_KANJI: u32 = 909u32;
pub const JPOS_KANYOUKU: u32 = 902u32;
pub const JPOS_KAZU: u32 = 126u32;
pub const JPOS_KAZU_SURYOU: u32 = 127u32;
pub const JPOS_KAZU_SUSHI: u32 = 128u32;
pub const JPOS_KEIDOU: u32 = 400u32;
pub const JPOS_KEIDOU_GARU: u32 = 403u32;
pub const JPOS_KEIDOU_NO: u32 = 401u32;
pub const JPOS_KEIDOU_TARU: u32 = 402u32;
pub const JPOS_KEIYOU: u32 = 300u32;
pub const JPOS_KEIYOU_GARU: u32 = 301u32;
pub const JPOS_KEIYOU_GE: u32 = 302u32;
pub const JPOS_KEIYOU_ME: u32 = 303u32;
pub const JPOS_KEIYOU_U: u32 = 305u32;
pub const JPOS_KEIYOU_YUU: u32 = 304u32;
pub const JPOS_KENCHIKU: u32 = 121u32;
pub const JPOS_KIGOU: u32 = 905u32;
pub const JPOS_KURU_KI: u32 = 219u32;
pub const JPOS_KURU_KITA: u32 = 220u32;
pub const JPOS_KURU_KITARA: u32 = 221u32;
pub const JPOS_KURU_KITARI: u32 = 222u32;
pub const JPOS_KURU_KITAROU: u32 = 223u32;
pub const JPOS_KURU_KITE: u32 = 224u32;
pub const JPOS_KURU_KO: u32 = 226u32;
pub const JPOS_KURU_KOI: u32 = 227u32;
pub const JPOS_KURU_KOYOU: u32 = 228u32;
pub const JPOS_KURU_KUREBA: u32 = 225u32;
pub const JPOS_KUTEN: u32 = 907u32;
pub const JPOS_MEISA_KEIDOU: u32 = 105u32;
pub const JPOS_MEISHI_FUTSU: u32 = 100u32;
pub const JPOS_MEISHI_KEIYOUDOUSHI: u32 = 103u32;
pub const JPOS_MEISHI_SAHEN: u32 = 101u32;
pub const JPOS_MEISHI_ZAHEN: u32 = 102u32;
pub const JPOS_OPENBRACE: u32 = 910u32;
pub const JPOS_RENTAISHI: u32 = 600u32;
pub const JPOS_RENTAISHI_SHIJI: u32 = 601u32;
pub const JPOS_RENYOU_SETSUBI: u32 = 826u32;
pub const JPOS_SETSUBI: u32 = 800u32;
pub const JPOS_SETSUBI_CHIMEI: u32 = 811u32;
pub const JPOS_SETSUBI_CHOU: u32 = 818u32;
pub const JPOS_SETSUBI_CHU: u32 = 804u32;
pub const JPOS_SETSUBI_DONO: u32 = 835u32;
pub const JPOS_SETSUBI_EKI: u32 = 821u32;
pub const JPOS_SETSUBI_FU: u32 = 805u32;
pub const JPOS_SETSUBI_FUKUSU: u32 = 836u32;
pub const JPOS_SETSUBI_GUN: u32 = 814u32;
pub const JPOS_SETSUBI_JIKAN: u32 = 829u32;
pub const JPOS_SETSUBI_JIKANPLUS: u32 = 830u32;
pub const JPOS_SETSUBI_JINMEI: u32 = 810u32;
pub const JPOS_SETSUBI_JOSUSHI: u32 = 827u32;
pub const JPOS_SETSUBI_JOSUSHIPLUS: u32 = 828u32;
pub const JPOS_SETSUBI_KA: u32 = 803u32;
pub const JPOS_SETSUBI_KATA: u32 = 808u32;
pub const JPOS_SETSUBI_KEN: u32 = 813u32;
pub const JPOS_SETSUBI_KENCHIKU: u32 = 825u32;
pub const JPOS_SETSUBI_KU: u32 = 815u32;
pub const JPOS_SETSUBI_KUN: u32 = 833u32;
pub const JPOS_SETSUBI_KUNI: u32 = 812u32;
pub const JPOS_SETSUBI_MACHI: u32 = 817u32;
pub const JPOS_SETSUBI_MEISHIRENDAKU: u32 = 809u32;
pub const JPOS_SETSUBI_MURA: u32 = 819u32;
pub const JPOS_SETSUBI_RA: u32 = 838u32;
pub const JPOS_SETSUBI_RYU: u32 = 806u32;
pub const JPOS_SETSUBI_SAMA: u32 = 834u32;
pub const JPOS_SETSUBI_SAN: u32 = 832u32;
pub const JPOS_SETSUBI_SEI: u32 = 802u32;
pub const JPOS_SETSUBI_SHAMEI: u32 = 823u32;
pub const JPOS_SETSUBI_SHI: u32 = 816u32;
pub const JPOS_SETSUBI_SON: u32 = 820u32;
pub const JPOS_SETSUBI_SONOTA: u32 = 822u32;
pub const JPOS_SETSUBI_SOSHIKI: u32 = 824u32;
pub const JPOS_SETSUBI_TACHI: u32 = 837u32;
pub const JPOS_SETSUBI_TEINEI: u32 = 831u32;
pub const JPOS_SETSUBI_TEKI: u32 = 801u32;
pub const JPOS_SETSUBI_YOU: u32 = 807u32;
pub const JPOS_SETSUZOKUSHI: u32 = 650u32;
pub const JPOS_SETTOU: u32 = 700u32;
pub const JPOS_SETTOU_CHIMEI: u32 = 710u32;
pub const JPOS_SETTOU_CHOUTAN: u32 = 707u32;
pub const JPOS_SETTOU_DAISHOU: u32 = 705u32;
pub const JPOS_SETTOU_FUKU: u32 = 703u32;
pub const JPOS_SETTOU_JINMEI: u32 = 709u32;
pub const JPOS_SETTOU_JOSUSHI: u32 = 712u32;
pub const JPOS_SETTOU_KAKU: u32 = 701u32;
pub const JPOS_SETTOU_KOUTEI: u32 = 706u32;
pub const JPOS_SETTOU_MI: u32 = 704u32;
pub const JPOS_SETTOU_SAI: u32 = 702u32;
pub const JPOS_SETTOU_SHINKYU: u32 = 708u32;
pub const JPOS_SETTOU_SONOTA: u32 = 711u32;
pub const JPOS_SETTOU_TEINEI_GO: u32 = 714u32;
pub const JPOS_SETTOU_TEINEI_O: u32 = 713u32;
pub const JPOS_SETTOU_TEINEI_ON: u32 = 715u32;
pub const JPOS_SHAMEI: u32 = 119u32;
pub const JPOS_SONOTA: u32 = 118u32;
pub const JPOS_SOSHIKI: u32 = 120u32;
pub const JPOS_SURU_SA: u32 = 229u32;
pub const JPOS_SURU_SE: u32 = 238u32;
pub const JPOS_SURU_SEYO: u32 = 239u32;
pub const JPOS_SURU_SI: u32 = 230u32;
pub const JPOS_SURU_SIATRI: u32 = 233u32;
pub const JPOS_SURU_SITA: u32 = 231u32;
pub const JPOS_SURU_SITARA: u32 = 232u32;
pub const JPOS_SURU_SITAROU: u32 = 234u32;
pub const JPOS_SURU_SITE: u32 = 235u32;
pub const JPOS_SURU_SIYOU: u32 = 236u32;
pub const JPOS_SURU_SUREBA: u32 = 237u32;
pub const JPOS_TANKANJI: u32 = 900u32;
pub const JPOS_TANKANJI_KAO: u32 = 901u32;
pub const JPOS_TANSHUKU: u32 = 913u32;
pub const JPOS_TOKUSHU_KAHEN: u32 = 214u32;
pub const JPOS_TOKUSHU_NAHEN: u32 = 218u32;
pub const JPOS_TOKUSHU_SAHEN: u32 = 216u32;
pub const JPOS_TOKUSHU_SAHENSURU: u32 = 215u32;
pub const JPOS_TOKUSHU_ZAHEN: u32 = 217u32;
pub const JPOS_TOUTEN: u32 = 908u32;
pub const JPOS_UNDEFINED: u32 = 0u32;
pub const JPOS_YOKUSEI: u32 = 912u32;
pub const MAX_APPLETTITLE: u32 = 64u32;
pub const MAX_FONTFACE: u32 = 32u32;
pub const MODEBIASMODE_DEFAULT: u32 = 0u32;
pub const MODEBIASMODE_DIGIT: u32 = 4u32;
pub const MODEBIASMODE_FILENAME: u32 = 1u32;
pub const MODEBIASMODE_READING: u32 = 2u32;
pub const MODEBIAS_GETVALUE: u32 = 2u32;
pub const MODEBIAS_GETVERSION: u32 = 0u32;
pub const MODEBIAS_SETVALUE: u32 = 1u32;
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const MOD_LEFT: u32 = 32768u32;
pub const MOD_ON_KEYUP: u32 = 2048u32;
pub const MOD_RIGHT: u32 = 16384u32;
pub const NI_CHANGECANDIDATELIST: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(19u32);
pub const NI_CLOSECANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(17u32);
pub const NI_COMPOSITIONSTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(21u32);
pub const NI_CONTEXTUPDATED: u32 = 3u32;
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20u32;
pub const NI_IMEMENUSELECTED: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(24u32);
pub const NI_OPENCANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(16u32);
pub const NI_SELECTCANDIDATESTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(18u32);
pub const NI_SETCANDIDATE_PAGESIZE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(23u32);
pub const NI_SETCANDIDATE_PAGESTART: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(22u32);
pub const POS_UNDEFINED: u32 = 0u32;
pub const RECONVOPT_NONE: u32 = 0u32;
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
pub const RWM_CHGKEYMAP: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEChangeKeyMap");
pub const RWM_DOCUMENTFEED: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEDocumentFeed");
pub const RWM_KEYMAP: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEKeyMap");
pub const RWM_MODEBIAS: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEModeBias");
pub const RWM_MOUSE: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEMouseOperation");
pub const RWM_NTFYKEYMAP: ::windows_core::PCWSTR = ::windows_core::w!("MSIMENotifyKeyMap");
pub const RWM_QUERYPOSITION: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEQueryPosition");
pub const RWM_RECONVERT: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEReconvert");
pub const RWM_RECONVERTOPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEReconvertOptions");
pub const RWM_RECONVERTREQUEST: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEReconvertRequest");
pub const RWM_SERVICE: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEService");
pub const RWM_SHOWIMEPAD: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEShowImePad");
pub const RWM_UIREADY: ::windows_core::PCWSTR = ::windows_core::w!("MSIMEUIReady");
pub const SCS_CAP_COMPSTR: u32 = 1u32;
pub const SCS_CAP_MAKEREAD: u32 = 2u32;
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4u32;
pub const SCS_CHANGEATTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(18u32);
pub const SCS_CHANGECLAUSE: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(36u32);
pub const SCS_QUERYRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(131072u32);
pub const SCS_SETRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(65536u32);
pub const SCS_SETSTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(9u32);
pub const SELECT_CAP_CONVERSION: u32 = 1u32;
pub const SELECT_CAP_SENTENCE: u32 = 2u32;
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
pub const SHOWIMEPAD_GUID: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
pub const UI_CAP_2700: u32 = 1u32;
pub const UI_CAP_ROT90: u32 = 2u32;
pub const UI_CAP_ROTANY: u32 = 4u32;
pub const UI_CAP_SOFTKBD: u32 = 65536u32;
pub const VERSION_DOCUMENTFEED: u32 = 1u32;
pub const VERSION_ID_CHINESE_SIMPLIFIED: u32 = 134217728u32;
pub const VERSION_ID_CHINESE_TRADITIONAL: u32 = 67108864u32;
pub const VERSION_ID_JAPANESE: u32 = 16777216u32;
pub const VERSION_ID_KOREAN: u32 = 33554432u32;
pub const VERSION_MODEBIAS: u32 = 1u32;
pub const VERSION_MOUSE_OPERATION: u32 = 1u32;
pub const VERSION_QUERYPOSITION: u32 = 1u32;
pub const VERSION_RECONVERSION: u32 = 1u32;
pub const cbCommentMax: u32 = 256u32;
pub const szImeChina: ::windows_core::PCWSTR = ::windows_core::w!("MSIME.China");
pub const szImeJapan: ::windows_core::PCWSTR = ::windows_core::w!("MSIME.Japan");
pub const szImeKorea: ::windows_core::PCWSTR = ::windows_core::w!("MSIME.Korea");
pub const szImeTaiwan: ::windows_core::PCWSTR = ::windows_core::w!("MSIME.Taiwan");
pub const wchPrivate1: u32 = 57344u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_CONVERSION_LIST_FLAG(pub u32);
impl ::core::marker::Copy for GET_CONVERSION_LIST_FLAG {}
impl ::core::clone::Clone for GET_CONVERSION_LIST_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CONVERSION_LIST_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GET_CONVERSION_LIST_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GET_CONVERSION_LIST_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CONVERSION_LIST_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_GUIDE_LINE_TYPE(pub u32);
impl ::core::marker::Copy for GET_GUIDE_LINE_TYPE {}
impl ::core::clone::Clone for GET_GUIDE_LINE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GUIDE_LINE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GET_GUIDE_LINE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GET_GUIDE_LINE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUIDE_LINE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMEFMT(pub i32);
impl ::core::marker::Copy for IMEFMT {}
impl ::core::clone::Clone for IMEFMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEFMT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMEFMT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMEFMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEFMT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMEREG(pub i32);
impl ::core::marker::Copy for IMEREG {}
impl ::core::clone::Clone for IMEREG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEREG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMEREG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMEREG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEREG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMEREL(pub i32);
impl ::core::marker::Copy for IMEREL {}
impl ::core::clone::Clone for IMEREL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEREL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMEREL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMEREL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEREL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMEUCT(pub i32);
impl ::core::marker::Copy for IMEUCT {}
impl ::core::clone::Clone for IMEUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMEUCT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMEUCT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMEUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMEUCT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_COMPOSITION_STRING(pub u32);
impl ::core::marker::Copy for IME_COMPOSITION_STRING {}
impl ::core::clone::Clone for IME_COMPOSITION_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_COMPOSITION_STRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_COMPOSITION_STRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_COMPOSITION_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_COMPOSITION_STRING").field(&self.0).finish()
    }
}
impl IME_COMPOSITION_STRING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IME_COMPOSITION_STRING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IME_COMPOSITION_STRING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IME_COMPOSITION_STRING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IME_COMPOSITION_STRING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IME_COMPOSITION_STRING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_CONVERSION_MODE(pub u32);
impl ::core::marker::Copy for IME_CONVERSION_MODE {}
impl ::core::clone::Clone for IME_CONVERSION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_CONVERSION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_CONVERSION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_CONVERSION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_CONVERSION_MODE").field(&self.0).finish()
    }
}
impl IME_CONVERSION_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IME_CONVERSION_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IME_CONVERSION_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IME_CONVERSION_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IME_CONVERSION_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IME_CONVERSION_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_ESCAPE(pub u32);
impl ::core::marker::Copy for IME_ESCAPE {}
impl ::core::clone::Clone for IME_ESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_ESCAPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_ESCAPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_ESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_ESCAPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_HOTKEY_IDENTIFIER(pub u32);
impl ::core::marker::Copy for IME_HOTKEY_IDENTIFIER {}
impl ::core::clone::Clone for IME_HOTKEY_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_HOTKEY_IDENTIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_HOTKEY_IDENTIFIER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_HOTKEY_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_HOTKEY_IDENTIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_PAD_REQUEST_FLAGS(pub u32);
impl ::core::marker::Copy for IME_PAD_REQUEST_FLAGS {}
impl ::core::clone::Clone for IME_PAD_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_PAD_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_PAD_REQUEST_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_PAD_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_PAD_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IME_SENTENCE_MODE(pub u32);
impl ::core::marker::Copy for IME_SENTENCE_MODE {}
impl ::core::clone::Clone for IME_SENTENCE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IME_SENTENCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IME_SENTENCE_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IME_SENTENCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IME_SENTENCE_MODE").field(&self.0).finish()
    }
}
impl IME_SENTENCE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IME_SENTENCE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IME_SENTENCE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IME_SENTENCE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IME_SENTENCE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IME_SENTENCE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NOTIFY_IME_ACTION(pub u32);
impl ::core::marker::Copy for NOTIFY_IME_ACTION {}
impl ::core::clone::Clone for NOTIFY_IME_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NOTIFY_IME_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NOTIFY_IME_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NOTIFY_IME_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NOTIFY_IME_INDEX(pub u32);
impl ::core::marker::Copy for NOTIFY_IME_INDEX {}
impl ::core::clone::Clone for NOTIFY_IME_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NOTIFY_IME_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NOTIFY_IME_INDEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NOTIFY_IME_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SET_COMPOSITION_STRING_TYPE(pub u32);
impl ::core::marker::Copy for SET_COMPOSITION_STRING_TYPE {}
impl ::core::clone::Clone for SET_COMPOSITION_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SET_COMPOSITION_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SET_COMPOSITION_STRING_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SET_COMPOSITION_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_COMPOSITION_STRING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct APPLETIDLIST {
    pub count: i32,
    pub pIIDList: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for APPLETIDLIST {}
impl ::core::clone::Clone for APPLETIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPLETIDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLETIDLIST").field("count", &self.count).field("pIIDList", &self.pIIDList).finish()
    }
}
impl ::windows_core::TypeKind for APPLETIDLIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for APPLETIDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.pIIDList == other.pIIDList
    }
}
impl ::core::cmp::Eq for APPLETIDLIST {}
impl ::core::default::Default for APPLETIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct APPLYCANDEXPARAM {
    pub dwSize: u32,
    pub lpwstrDisplay: ::windows_core::PWSTR,
    pub lpwstrReading: ::windows_core::PWSTR,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for APPLYCANDEXPARAM {}
impl ::core::clone::Clone for APPLYCANDEXPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPLYCANDEXPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLYCANDEXPARAM").field("dwSize", &self.dwSize).field("lpwstrDisplay", &self.lpwstrDisplay).field("lpwstrReading", &self.lpwstrReading).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows_core::TypeKind for APPLYCANDEXPARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for APPLYCANDEXPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpwstrDisplay == other.lpwstrDisplay && self.lpwstrReading == other.lpwstrReading && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for APPLYCANDEXPARAM {}
impl ::core::default::Default for APPLYCANDEXPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CANDIDATEFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CANDIDATEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEFORM").field("dwIndex", &self.dwIndex).field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for CANDIDATEFORM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CANDIDATEFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwIndex == other.dwIndex && self.dwStyle == other.dwStyle && self.ptCurrentPos == other.ptCurrentPos && self.rcArea == other.rcArea
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CANDIDATEFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CANDIDATEINFO {
    pub dwSize: u32,
    pub dwCount: u32,
    pub dwOffset: [u32; 32],
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for CANDIDATEINFO {}
impl ::core::clone::Clone for CANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEINFO").field("dwSize", &self.dwSize).field("dwCount", &self.dwCount).field("dwOffset", &self.dwOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
impl ::windows_core::TypeKind for CANDIDATEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCount == other.dwCount && self.dwOffset == other.dwOffset && self.dwPrivateSize == other.dwPrivateSize && self.dwPrivateOffset == other.dwPrivateOffset
    }
}
impl ::core::cmp::Eq for CANDIDATEINFO {}
impl ::core::default::Default for CANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl ::core::marker::Copy for CANDIDATELIST {}
impl ::core::clone::Clone for CANDIDATELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATELIST").field("dwSize", &self.dwSize).field("dwStyle", &self.dwStyle).field("dwCount", &self.dwCount).field("dwSelection", &self.dwSelection).field("dwPageStart", &self.dwPageStart).field("dwPageSize", &self.dwPageSize).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::windows_core::TypeKind for CANDIDATELIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CANDIDATELIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwStyle == other.dwStyle && self.dwCount == other.dwCount && self.dwSelection == other.dwSelection && self.dwPageStart == other.dwPageStart && self.dwPageSize == other.dwPageSize && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for CANDIDATELIST {}
impl ::core::default::Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITIONFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITIONFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONFORM").field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for COMPOSITIONFORM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITIONFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwStyle == other.dwStyle && self.ptCurrentPos == other.ptCurrentPos && self.rcArea == other.rcArea
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITIONFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITIONSTRING {
    pub dwSize: u32,
    pub dwCompReadAttrLen: u32,
    pub dwCompReadAttrOffset: u32,
    pub dwCompReadClauseLen: u32,
    pub dwCompReadClauseOffset: u32,
    pub dwCompReadStrLen: u32,
    pub dwCompReadStrOffset: u32,
    pub dwCompAttrLen: u32,
    pub dwCompAttrOffset: u32,
    pub dwCompClauseLen: u32,
    pub dwCompClauseOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwCursorPos: u32,
    pub dwDeltaStart: u32,
    pub dwResultReadClauseLen: u32,
    pub dwResultReadClauseOffset: u32,
    pub dwResultReadStrLen: u32,
    pub dwResultReadStrOffset: u32,
    pub dwResultClauseLen: u32,
    pub dwResultClauseOffset: u32,
    pub dwResultStrLen: u32,
    pub dwResultStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for COMPOSITIONSTRING {}
impl ::core::clone::Clone for COMPOSITIONSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITIONSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONSTRING")
            .field("dwSize", &self.dwSize)
            .field("dwCompReadAttrLen", &self.dwCompReadAttrLen)
            .field("dwCompReadAttrOffset", &self.dwCompReadAttrOffset)
            .field("dwCompReadClauseLen", &self.dwCompReadClauseLen)
            .field("dwCompReadClauseOffset", &self.dwCompReadClauseOffset)
            .field("dwCompReadStrLen", &self.dwCompReadStrLen)
            .field("dwCompReadStrOffset", &self.dwCompReadStrOffset)
            .field("dwCompAttrLen", &self.dwCompAttrLen)
            .field("dwCompAttrOffset", &self.dwCompAttrOffset)
            .field("dwCompClauseLen", &self.dwCompClauseLen)
            .field("dwCompClauseOffset", &self.dwCompClauseOffset)
            .field("dwCompStrLen", &self.dwCompStrLen)
            .field("dwCompStrOffset", &self.dwCompStrOffset)
            .field("dwCursorPos", &self.dwCursorPos)
            .field("dwDeltaStart", &self.dwDeltaStart)
            .field("dwResultReadClauseLen", &self.dwResultReadClauseLen)
            .field("dwResultReadClauseOffset", &self.dwResultReadClauseOffset)
            .field("dwResultReadStrLen", &self.dwResultReadStrLen)
            .field("dwResultReadStrOffset", &self.dwResultReadStrOffset)
            .field("dwResultClauseLen", &self.dwResultClauseLen)
            .field("dwResultClauseOffset", &self.dwResultClauseOffset)
            .field("dwResultStrLen", &self.dwResultStrLen)
            .field("dwResultStrOffset", &self.dwResultStrOffset)
            .field("dwPrivateSize", &self.dwPrivateSize)
            .field("dwPrivateOffset", &self.dwPrivateOffset)
            .finish()
    }
}
impl ::windows_core::TypeKind for COMPOSITIONSTRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITIONSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCompReadAttrLen == other.dwCompReadAttrLen
            && self.dwCompReadAttrOffset == other.dwCompReadAttrOffset
            && self.dwCompReadClauseLen == other.dwCompReadClauseLen
            && self.dwCompReadClauseOffset == other.dwCompReadClauseOffset
            && self.dwCompReadStrLen == other.dwCompReadStrLen
            && self.dwCompReadStrOffset == other.dwCompReadStrOffset
            && self.dwCompAttrLen == other.dwCompAttrLen
            && self.dwCompAttrOffset == other.dwCompAttrOffset
            && self.dwCompClauseLen == other.dwCompClauseLen
            && self.dwCompClauseOffset == other.dwCompClauseOffset
            && self.dwCompStrLen == other.dwCompStrLen
            && self.dwCompStrOffset == other.dwCompStrOffset
            && self.dwCursorPos == other.dwCursorPos
            && self.dwDeltaStart == other.dwDeltaStart
            && self.dwResultReadClauseLen == other.dwResultReadClauseLen
            && self.dwResultReadClauseOffset == other.dwResultReadClauseOffset
            && self.dwResultReadStrLen == other.dwResultReadStrLen
            && self.dwResultReadStrOffset == other.dwResultReadStrOffset
            && self.dwResultClauseLen == other.dwResultClauseLen
            && self.dwResultClauseOffset == other.dwResultClauseOffset
            && self.dwResultStrLen == other.dwResultStrLen
            && self.dwResultStrOffset == other.dwResultStrOffset
            && self.dwPrivateSize == other.dwPrivateSize
            && self.dwPrivateOffset == other.dwPrivateOffset
    }
}
impl ::core::cmp::Eq for COMPOSITIONSTRING {}
impl ::core::default::Default for COMPOSITIONSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GUIDELINE {
    pub dwSize: u32,
    pub dwLevel: u32,
    pub dwIndex: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for GUIDELINE {}
impl ::core::clone::Clone for GUIDELINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GUIDELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUIDELINE").field("dwSize", &self.dwSize).field("dwLevel", &self.dwLevel).field("dwIndex", &self.dwIndex).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
impl ::windows_core::TypeKind for GUIDELINE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GUIDELINE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwLevel == other.dwLevel && self.dwIndex == other.dwIndex && self.dwStrLen == other.dwStrLen && self.dwStrOffset == other.dwStrOffset && self.dwPrivateSize == other.dwPrivateSize && self.dwPrivateOffset == other.dwPrivateOffset
    }
}
impl ::core::cmp::Eq for GUIDELINE {}
impl ::core::default::Default for GUIDELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct IMEAPPLETCFG {
    pub dwConfig: u32,
    pub wchTitle: [u16; 64],
    pub wchTitleFontFace: [u16; 32],
    pub dwCharSet: u32,
    pub iCategory: i32,
    pub hIcon: super::super::WindowsAndMessaging::HICON,
    pub langID: u16,
    pub dummy: u16,
    pub lReserved1: super::super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for IMEAPPLETCFG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for IMEAPPLETCFG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETCFG").field("dwConfig", &self.dwConfig).field("wchTitle", &self.wchTitle).field("wchTitleFontFace", &self.wchTitleFontFace).field("dwCharSet", &self.dwCharSet).field("iCategory", &self.iCategory).field("hIcon", &self.hIcon).field("langID", &self.langID).field("dummy", &self.dummy).field("lReserved1", &self.lReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for IMEAPPLETCFG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for IMEAPPLETCFG {
    fn eq(&self, other: &Self) -> bool {
        self.dwConfig == other.dwConfig && self.wchTitle == other.wchTitle && self.wchTitleFontFace == other.wchTitleFontFace && self.dwCharSet == other.dwCharSet && self.iCategory == other.iCategory && self.hIcon == other.hIcon && self.langID == other.langID && self.dummy == other.dummy && self.lReserved1 == other.lReserved1
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for IMEAPPLETCFG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEAPPLETUI {
    pub hwnd: super::super::super::Foundation::HWND,
    pub dwStyle: u32,
    pub width: i32,
    pub height: i32,
    pub minWidth: i32,
    pub minHeight: i32,
    pub maxWidth: i32,
    pub maxHeight: i32,
    pub lReserved1: super::super::super::Foundation::LPARAM,
    pub lReserved2: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEAPPLETUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEAPPLETUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETUI").field("hwnd", &self.hwnd).field("dwStyle", &self.dwStyle).field("width", &self.width).field("height", &self.height).field("minWidth", &self.minWidth).field("minHeight", &self.minHeight).field("maxWidth", &self.maxWidth).field("maxHeight", &self.maxHeight).field("lReserved1", &self.lReserved1).field("lReserved2", &self.lReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for IMEAPPLETUI {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEAPPLETUI {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.dwStyle == other.dwStyle && self.width == other.width && self.height == other.height && self.minWidth == other.minWidth && self.minHeight == other.minHeight && self.maxWidth == other.maxWidth && self.maxHeight == other.maxHeight && self.lReserved1 == other.lReserved1 && self.lReserved2 == other.lReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEAPPLETUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMECHARINFO {
    pub wch: u16,
    pub dwCharInfo: u32,
}
impl ::core::marker::Copy for IMECHARINFO {}
impl ::core::clone::Clone for IMECHARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECHARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARINFO").field("wch", &self.wch).field("dwCharInfo", &self.dwCharInfo).finish()
    }
}
impl ::windows_core::TypeKind for IMECHARINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMECHARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wch == other.wch && self.dwCharInfo == other.dwCharInfo
    }
}
impl ::core::cmp::Eq for IMECHARINFO {}
impl ::core::default::Default for IMECHARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMECHARPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMECHARPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARPOSITION").field("dwSize", &self.dwSize).field("dwCharPos", &self.dwCharPos).field("pt", &self.pt).field("cLineHeight", &self.cLineHeight).field("rcDocument", &self.rcDocument).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for IMECHARPOSITION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMECHARPOSITION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCharPos == other.dwCharPos && self.pt == other.pt && self.cLineHeight == other.cLineHeight && self.rcDocument == other.rcDocument
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMECHARPOSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMECOMPOSITIONSTRINGINFO {
    pub iCompStrLen: i32,
    pub iCaretPos: i32,
    pub iEditStart: i32,
    pub iEditLen: i32,
    pub iTargetStart: i32,
    pub iTargetLen: i32,
}
impl ::core::marker::Copy for IMECOMPOSITIONSTRINGINFO {}
impl ::core::clone::Clone for IMECOMPOSITIONSTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECOMPOSITIONSTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPOSITIONSTRINGINFO").field("iCompStrLen", &self.iCompStrLen).field("iCaretPos", &self.iCaretPos).field("iEditStart", &self.iEditStart).field("iEditLen", &self.iEditLen).field("iTargetStart", &self.iTargetStart).field("iTargetLen", &self.iTargetLen).finish()
    }
}
impl ::windows_core::TypeKind for IMECOMPOSITIONSTRINGINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMECOMPOSITIONSTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.iCompStrLen == other.iCompStrLen && self.iCaretPos == other.iCaretPos && self.iEditStart == other.iEditStart && self.iEditLen == other.iEditLen && self.iTargetStart == other.iTargetStart && self.iTargetLen == other.iTargetLen
    }
}
impl ::core::cmp::Eq for IMECOMPOSITIONSTRINGINFO {}
impl ::core::default::Default for IMECOMPOSITIONSTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDLG {
    pub cbIMEDLG: i32,
    pub hwnd: super::super::super::Foundation::HWND,
    pub lpwstrWord: ::windows_core::PWSTR,
    pub nTabId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEDLG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for IMEDLG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEDLG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEDP {
    pub wrdModifier: IMEWRD,
    pub wrdModifiee: IMEWRD,
    pub relID: IMEREL,
}
impl ::core::marker::Copy for IMEDP {}
impl ::core::clone::Clone for IMEDP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEDP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEDP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEFAREASTINFO {
    pub dwSize: u32,
    pub dwType: u32,
    pub dwData: [u32; 1],
}
impl ::core::marker::Copy for IMEFAREASTINFO {}
impl ::core::clone::Clone for IMEFAREASTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEFAREASTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEFAREASTINFO").field("dwSize", &self.dwSize).field("dwType", &self.dwType).field("dwData", &self.dwData).finish()
    }
}
impl ::windows_core::TypeKind for IMEFAREASTINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMEFAREASTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwType == other.dwType && self.dwData == other.dwData
    }
}
impl ::core::cmp::Eq for IMEFAREASTINFO {}
impl ::core::default::Default for IMEFAREASTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEINFO {
    pub dwPrivateDataSize: u32,
    pub fdwProperty: u32,
    pub fdwConversionCaps: u32,
    pub fdwSentenceCaps: u32,
    pub fdwUICaps: u32,
    pub fdwSCSCaps: u32,
    pub fdwSelectCaps: u32,
}
impl ::core::marker::Copy for IMEINFO {}
impl ::core::clone::Clone for IMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEINFO").field("dwPrivateDataSize", &self.dwPrivateDataSize).field("fdwProperty", &self.fdwProperty).field("fdwConversionCaps", &self.fdwConversionCaps).field("fdwSentenceCaps", &self.fdwSentenceCaps).field("fdwUICaps", &self.fdwUICaps).field("fdwSCSCaps", &self.fdwSCSCaps).field("fdwSelectCaps", &self.fdwSelectCaps).finish()
    }
}
impl ::windows_core::TypeKind for IMEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwPrivateDataSize == other.dwPrivateDataSize && self.fdwProperty == other.fdwProperty && self.fdwConversionCaps == other.fdwConversionCaps && self.fdwSentenceCaps == other.fdwSentenceCaps && self.fdwUICaps == other.fdwUICaps && self.fdwSCSCaps == other.fdwSCSCaps && self.fdwSelectCaps == other.fdwSelectCaps
    }
}
impl ::core::cmp::Eq for IMEINFO {}
impl ::core::default::Default for IMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEITEM {
    pub cbSize: i32,
    pub iType: i32,
    pub lpItemData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMEITEM {}
impl ::core::clone::Clone for IMEITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEM").field("cbSize", &self.cbSize).field("iType", &self.iType).field("lpItemData", &self.lpItemData).finish()
    }
}
impl ::windows_core::TypeKind for IMEITEM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMEITEM {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iType == other.iType && self.lpItemData == other.lpItemData
    }
}
impl ::core::cmp::Eq for IMEITEM {}
impl ::core::default::Default for IMEITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMEITEMCANDIDATE {
    pub uCount: u32,
    pub imeItem: [IMEITEM; 1],
}
impl ::core::marker::Copy for IMEITEMCANDIDATE {}
impl ::core::clone::Clone for IMEITEMCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEMCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEMCANDIDATE").field("uCount", &self.uCount).field("imeItem", &self.imeItem).finish()
    }
}
impl ::windows_core::TypeKind for IMEITEMCANDIDATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMEITEMCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.imeItem == other.imeItem
    }
}
impl ::core::cmp::Eq for IMEITEMCANDIDATE {}
impl ::core::default::Default for IMEITEMCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMS {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub cKeyList: u32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMS {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::windows_core::TypeKind for IMEKMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEKMSFUNCDESC {
    pub cbSize: i32,
    pub idLang: u16,
    pub dwControl: u32,
    pub pwszDescription: [u16; 128],
}
impl ::core::marker::Copy for IMEKMSFUNCDESC {}
impl ::core::clone::Clone for IMEKMSFUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEKMSFUNCDESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEKMSFUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEKMSINIT {
    pub cbSize: i32,
    pub hWnd: super::super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEKMSINIT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEKMSINIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for IMEKMSINIT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEKMSINIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSINVK {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub dwControl: u32,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSINVK {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSINVK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::windows_core::TypeKind for IMEKMSINVK {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSINVK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEKMSKEY {
    pub dwStatus: u32,
    pub dwCompStatus: u32,
    pub dwVKEY: u32,
    pub Anonymous1: IMEKMSKEY_0,
    pub Anonymous2: IMEKMSKEY_1,
}
impl ::core::marker::Copy for IMEKMSKEY {}
impl ::core::clone::Clone for IMEKMSKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEKMSKEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEKMSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEKMSKEY_0 {
    pub dwControl: u32,
    pub dwNotUsed: u32,
}
impl ::core::marker::Copy for IMEKMSKEY_0 {}
impl ::core::clone::Clone for IMEKMSKEY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEKMSKEY_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEKMSKEY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEKMSKEY_1 {
    pub pwszDscr: [u16; 31],
    pub pwszNoUse: [u16; 31],
}
impl ::core::marker::Copy for IMEKMSKEY_1 {}
impl ::core::clone::Clone for IMEKMSKEY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEKMSKEY_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEKMSKEY_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSKMP {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub idLang: u16,
    pub wVKStart: u16,
    pub wVKEnd: u16,
    pub cKeyList: i32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSKMP {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSKMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::windows_core::TypeKind for IMEKMSKMP {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSKMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub struct IMEKMSNTFY {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub fSelect: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::marker::Copy for IMEKMSNTFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::clone::Clone for IMEKMSNTFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::windows_core::TypeKind for IMEKMSNTFY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::default::Default for IMEKMSNTFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u8; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMEMENUITEMINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMEMENUITEMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMEMENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOA").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::TypeKind for IMEMENUITEMINFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fType == other.fType && self.fState == other.fState && self.wID == other.wID && self.hbmpChecked == other.hbmpChecked && self.hbmpUnchecked == other.hbmpUnchecked && self.dwItemData == other.dwItemData && self.szString == other.szString && self.hbmpItem == other.hbmpItem
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMEMENUITEMINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMEMENUITEMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMEMENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOW").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::TypeKind for IMEMENUITEMINFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fType == other.fType && self.fState == other.fState && self.wID == other.wID && self.hbmpChecked == other.hbmpChecked && self.hbmpUnchecked == other.hbmpUnchecked && self.dwItemData == other.dwItemData && self.szString == other.szString && self.hbmpItem == other.hbmpItem
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMESHF {
    pub cbShf: u16,
    pub verDic: u16,
    pub szTitle: [u8; 48],
    pub szDescription: [u8; 256],
    pub szCopyright: [u8; 128],
}
impl ::core::marker::Copy for IMESHF {}
impl ::core::clone::Clone for IMESHF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMESHF {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMESHF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMESTRINGCANDIDATE {
    pub uCount: u32,
    pub lpwstr: [::windows_core::PWSTR; 1],
}
impl ::core::marker::Copy for IMESTRINGCANDIDATE {}
impl ::core::clone::Clone for IMESTRINGCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRINGCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATE").field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
impl ::windows_core::TypeKind for IMESTRINGCANDIDATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.lpwstr == other.lpwstr
    }
}
impl ::core::cmp::Eq for IMESTRINGCANDIDATE {}
impl ::core::default::Default for IMESTRINGCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMESTRINGCANDIDATEINFO {
    pub dwFarEastId: u32,
    pub lpFarEastInfo: *mut IMEFAREASTINFO,
    pub fInfoMask: u32,
    pub iSelIndex: i32,
    pub uCount: u32,
    pub lpwstr: [::windows_core::PWSTR; 1],
}
impl ::core::marker::Copy for IMESTRINGCANDIDATEINFO {}
impl ::core::clone::Clone for IMESTRINGCANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRINGCANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATEINFO").field("dwFarEastId", &self.dwFarEastId).field("lpFarEastInfo", &self.lpFarEastInfo).field("fInfoMask", &self.fInfoMask).field("iSelIndex", &self.iSelIndex).field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
impl ::windows_core::TypeKind for IMESTRINGCANDIDATEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFarEastId == other.dwFarEastId && self.lpFarEastInfo == other.lpFarEastInfo && self.fInfoMask == other.fInfoMask && self.iSelIndex == other.iSelIndex && self.uCount == other.uCount && self.lpwstr == other.lpwstr
    }
}
impl ::core::cmp::Eq for IMESTRINGCANDIDATEINFO {}
impl ::core::default::Default for IMESTRINGCANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMESTRINGINFO {
    pub dwFarEastId: u32,
    pub lpwstr: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for IMESTRINGINFO {}
impl ::core::clone::Clone for IMESTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMESTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGINFO").field("dwFarEastId", &self.dwFarEastId).field("lpwstr", &self.lpwstr).finish()
    }
}
impl ::windows_core::TypeKind for IMESTRINGINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMESTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFarEastId == other.dwFarEastId && self.lpwstr == other.lpwstr
    }
}
impl ::core::cmp::Eq for IMESTRINGINFO {}
impl ::core::default::Default for IMESTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEWRD {
    pub pwchReading: ::windows_core::PWSTR,
    pub pwchDisplay: ::windows_core::PWSTR,
    pub Anonymous: IMEWRD_0,
    pub rgulAttrs: [u32; 2],
    pub cbComment: i32,
    pub uct: IMEUCT,
    pub pvComment: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMEWRD {}
impl ::core::clone::Clone for IMEWRD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEWRD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEWRD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union IMEWRD_0 {
    pub ulPos: u32,
    pub Anonymous: IMEWRD_0_0,
}
impl ::core::marker::Copy for IMEWRD_0 {}
impl ::core::clone::Clone for IMEWRD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEWRD_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEWRD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct IMEWRD_0_0 {
    pub nPos1: u16,
    pub nPos2: u16,
}
impl ::core::marker::Copy for IMEWRD_0_0 {}
impl ::core::clone::Clone for IMEWRD_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for IMEWRD_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for IMEWRD_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub struct INPUTCONTEXT {
    pub hWnd: super::super::super::Foundation::HWND,
    pub fOpen: super::super::super::Foundation::BOOL,
    pub ptStatusWndPos: super::super::super::Foundation::POINT,
    pub ptSoftKbdPos: super::super::super::Foundation::POINT,
    pub fdwConversion: u32,
    pub fdwSentence: u32,
    pub lfFont: INPUTCONTEXT_0,
    pub cfCompForm: COMPOSITIONFORM,
    pub cfCandForm: [CANDIDATEFORM; 4],
    pub hCompStr: super::super::super::Globalization::HIMCC,
    pub hCandInfo: super::super::super::Globalization::HIMCC,
    pub hGuideLine: super::super::super::Globalization::HIMCC,
    pub hPrivate: super::super::super::Globalization::HIMCC,
    pub dwNumMsgBuf: u32,
    pub hMsgBuf: super::super::super::Globalization::HIMCC,
    pub fdwInit: u32,
    pub dwReserve: [u32; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for INPUTCONTEXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub union INPUTCONTEXT_0 {
    pub A: super::super::super::Graphics::Gdi::LOGFONTA,
    pub W: super::super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for INPUTCONTEXT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct MORRSLT {
    pub dwSize: u32,
    pub pwchOutput: ::windows_core::PWSTR,
    pub cchOutput: u16,
    pub Anonymous1: MORRSLT_0,
    pub Anonymous2: MORRSLT_1,
    pub pchInputPos: *mut u16,
    pub pchOutputIdxWDD: *mut u16,
    pub Anonymous3: MORRSLT_2,
    pub paMonoRubyPos: *mut u16,
    pub pWDD: *mut WDD,
    pub cWDD: i32,
    pub pPrivate: *mut ::core::ffi::c_void,
    pub BLKBuff: [u16; 1],
}
impl ::core::marker::Copy for MORRSLT {}
impl ::core::clone::Clone for MORRSLT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MORRSLT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MORRSLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_0 {
    pub pwchRead: ::windows_core::PWSTR,
    pub pwchComp: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for MORRSLT_0 {}
impl ::core::clone::Clone for MORRSLT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MORRSLT_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MORRSLT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl ::core::marker::Copy for MORRSLT_1 {}
impl ::core::clone::Clone for MORRSLT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MORRSLT_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MORRSLT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union MORRSLT_2 {
    pub pchReadIdxWDD: *mut u16,
    pub pchCompIdxWDD: *mut u16,
}
impl ::core::marker::Copy for MORRSLT_2 {}
impl ::core::clone::Clone for MORRSLT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MORRSLT_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MORRSLT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct POSTBL {
    pub nPos: u16,
    pub szName: *mut u8,
}
impl ::core::marker::Copy for POSTBL {}
impl ::core::clone::Clone for POSTBL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for POSTBL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for POSTBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for RECONVERTSTRING {}
impl ::core::clone::Clone for RECONVERTSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECONVERTSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECONVERTSTRING").field("dwSize", &self.dwSize).field("dwVersion", &self.dwVersion).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwCompStrLen", &self.dwCompStrLen).field("dwCompStrOffset", &self.dwCompStrOffset).field("dwTargetStrLen", &self.dwTargetStrLen).field("dwTargetStrOffset", &self.dwTargetStrOffset).finish()
    }
}
impl ::windows_core::TypeKind for RECONVERTSTRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for RECONVERTSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVersion == other.dwVersion && self.dwStrLen == other.dwStrLen && self.dwStrOffset == other.dwStrOffset && self.dwCompStrLen == other.dwCompStrLen && self.dwCompStrOffset == other.dwCompStrOffset && self.dwTargetStrLen == other.dwTargetStrLen && self.dwTargetStrOffset == other.dwTargetStrOffset
    }
}
impl ::core::cmp::Eq for RECONVERTSTRING {}
impl ::core::default::Default for RECONVERTSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REGISTERWORDA {
    pub lpReading: ::windows_core::PSTR,
    pub lpWord: ::windows_core::PSTR,
}
impl ::core::marker::Copy for REGISTERWORDA {}
impl ::core::clone::Clone for REGISTERWORDA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REGISTERWORDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDA").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
impl ::windows_core::TypeKind for REGISTERWORDA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REGISTERWORDA {
    fn eq(&self, other: &Self) -> bool {
        self.lpReading == other.lpReading && self.lpWord == other.lpWord
    }
}
impl ::core::cmp::Eq for REGISTERWORDA {}
impl ::core::default::Default for REGISTERWORDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REGISTERWORDW {
    pub lpReading: ::windows_core::PWSTR,
    pub lpWord: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for REGISTERWORDW {}
impl ::core::clone::Clone for REGISTERWORDW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REGISTERWORDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDW").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
impl ::windows_core::TypeKind for REGISTERWORDW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REGISTERWORDW {
    fn eq(&self, other: &Self) -> bool {
        self.lpReading == other.lpReading && self.lpWord == other.lpWord
    }
}
impl ::core::cmp::Eq for REGISTERWORDW {}
impl ::core::default::Default for REGISTERWORDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SOFTKBDDATA {
    pub uCount: u32,
    pub wCode: [u16; 256],
}
impl ::core::marker::Copy for SOFTKBDDATA {}
impl ::core::clone::Clone for SOFTKBDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOFTKBDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOFTKBDDATA").field("uCount", &self.uCount).field("wCode", &self.wCode).finish()
    }
}
impl ::windows_core::TypeKind for SOFTKBDDATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SOFTKBDDATA {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.wCode == other.wCode
    }
}
impl ::core::cmp::Eq for SOFTKBDDATA {}
impl ::core::default::Default for SOFTKBDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [u8; 32],
}
impl ::core::marker::Copy for STYLEBUFA {}
impl ::core::clone::Clone for STYLEBUFA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLEBUFA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFA").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
impl ::windows_core::TypeKind for STYLEBUFA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STYLEBUFA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStyle == other.dwStyle && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for STYLEBUFA {}
impl ::core::default::Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for STYLEBUFW {}
impl ::core::clone::Clone for STYLEBUFW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLEBUFW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFW").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
impl ::windows_core::TypeKind for STYLEBUFW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for STYLEBUFW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStyle == other.dwStyle && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for STYLEBUFW {}
impl ::core::default::Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSG {
    pub message: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSG").field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for TRANSMSG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSG {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message && self.wParam == other.wParam && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSGLIST {
    pub uMsgCount: u32,
    pub TransMsg: [TRANSMSG; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSGLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSGLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSGLIST").field("uMsgCount", &self.uMsgCount).field("TransMsg", &self.TransMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for TRANSMSGLIST {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSGLIST {
    fn eq(&self, other: &Self) -> bool {
        self.uMsgCount == other.uMsgCount && self.TransMsg == other.TransMsg
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSGLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WDD {
    pub wDispPos: u16,
    pub Anonymous1: WDD_0,
    pub cchDisp: u16,
    pub Anonymous2: WDD_1,
    pub WDD_nReserve1: u32,
    pub nPos: u16,
    pub _bitfield: u16,
    pub pReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WDD {}
impl ::core::clone::Clone for WDD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WDD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WDD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union WDD_0 {
    pub wReadPos: u16,
    pub wCompPos: u16,
}
impl ::core::marker::Copy for WDD_0 {}
impl ::core::clone::Clone for WDD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WDD_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WDD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub union WDD_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl ::core::marker::Copy for WDD_1 {}
impl ::core::clone::Clone for WDD_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WDD_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WDD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub type IMCENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNLOG = ::core::option::Option<unsafe extern "system" fn(param0: *mut IMEDP, param1: ::windows_core::HRESULT) -> super::super::super::Foundation::BOOL>;
pub type REGISTERWORDENUMPROCA = ::core::option::Option<unsafe extern "system" fn(lpszreading: ::windows_core::PCSTR, param1: u32, lpszstring: ::windows_core::PCSTR, param3: *mut ::core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = ::core::option::Option<unsafe extern "system" fn(lpszreading: ::windows_core::PCWSTR, param1: u32, lpszstring: ::windows_core::PCWSTR, param3: *mut ::core::ffi::c_void) -> i32>;
pub type fpCreateIFECommonInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type fpCreateIFEDictionaryInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type fpCreateIFELanguageInstanceType = ::core::option::Option<unsafe extern "system" fn(clsid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
