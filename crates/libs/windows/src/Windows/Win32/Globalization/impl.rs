#[cfg(feature = "Win32_Foundation")]
pub trait IComprehensiveSpellCheckProviderImpl: Sized {
    fn ComprehensiveCheck();
}
#[cfg(feature = "Win32_Foundation")]
impl IComprehensiveSpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComprehensiveSpellCheckProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComprehensiveSpellCheckProviderVtbl {
        unsafe extern "system" fn ComprehensiveCheck<Impl: IComprehensiveSpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ComprehensiveCheck: ComprehensiveCheck::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComprehensiveSpellCheckProvider as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCodePageImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl IEnumCodePageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCodePageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCodePageVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCodePage as ::windows::core::Interface>::IID
    }
}
pub trait IEnumRfc1766Impl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl IEnumRfc1766Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRfc1766Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRfc1766Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumRfc1766 as ::windows::core::Interface>::IID
    }
}
pub trait IEnumScriptImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl IEnumScriptVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumScriptImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumScriptVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumScript as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSpellingErrorImpl: Sized {
    fn Next();
}
impl IEnumSpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpellingErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSpellingErrorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSpellingError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangCodePagesImpl: Sized {
    fn GetCharCodePages();
    fn GetStrCodePages();
    fn CodePageToCodePages();
    fn CodePagesToCodePage();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangCodePagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangCodePagesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangCodePagesVtbl {
        unsafe extern "system" fn GetCharCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CodePageToCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CodePagesToCodePage<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCharCodePages: GetCharCodePages::<Impl, IMPL_OFFSET>,
            GetStrCodePages: GetStrCodePages::<Impl, IMPL_OFFSET>,
            CodePageToCodePages: CodePageToCodePages::<Impl, IMPL_OFFSET>,
            CodePagesToCodePage: CodePagesToCodePage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangCodePages as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangConvertCharsetImpl: Sized {
    fn Initialize();
    fn GetSourceCodePage();
    fn GetDestinationCodePage();
    fn GetProperty();
    fn DoConversion();
    fn DoConversionToUnicode();
    fn DoConversionFromUnicode();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangConvertCharsetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangConvertCharsetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangConvertCharsetVtbl {
        unsafe extern "system" fn Initialize<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puisrccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestinationCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puidstcodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoConversion<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoConversionToUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoConversionFromUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetSourceCodePage: GetSourceCodePage::<Impl, IMPL_OFFSET>,
            GetDestinationCodePage: GetDestinationCodePage::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            DoConversion: DoConversion::<Impl, IMPL_OFFSET>,
            DoConversionToUnicode: DoConversionToUnicode::<Impl, IMPL_OFFSET>,
            DoConversionFromUnicode: DoConversionFromUnicode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangConvertCharset as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IMLangFontLinkImpl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn MapFont();
    fn ReleaseFont();
    fn ResetFontMapping();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMLangFontLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangFontLinkVtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMLangCodePagesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Impl, IMPL_OFFSET>,
            MapFont: MapFont::<Impl, IMPL_OFFSET>,
            ReleaseFont: ReleaseFont::<Impl, IMPL_OFFSET>,
            ResetFontMapping: ResetFontMapping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangFontLink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IMLangFontLink2Impl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn ReleaseFont();
    fn ResetFontMapping();
    fn MapFont();
    fn GetFontUnicodeRanges();
    fn GetScriptFontInfo();
    fn CodePageToScriptID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMLangFontLink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangFontLink2Vtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScriptFontInfo<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut tagSCRIPFONTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CodePageToScriptID<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMLangCodePagesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Impl, IMPL_OFFSET>,
            ReleaseFont: ReleaseFont::<Impl, IMPL_OFFSET>,
            ResetFontMapping: ResetFontMapping::<Impl, IMPL_OFFSET>,
            MapFont: MapFont::<Impl, IMPL_OFFSET>,
            GetFontUnicodeRanges: GetFontUnicodeRanges::<Impl, IMPL_OFFSET>,
            GetScriptFontInfo: GetScriptFontInfo::<Impl, IMPL_OFFSET>,
            CodePageToScriptID: CodePageToScriptID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangFontLink2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangLineBreakConsoleImpl: Sized {
    fn BreakLineML();
    fn BreakLineW();
    fn BreakLineA();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangLineBreakConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangLineBreakConsoleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangLineBreakConsoleVtbl {
        unsafe extern "system" fn BreakLineML<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcmlstr: ::windows::core::RawPtr, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BreakLineW<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BreakLineA<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BreakLineML: BreakLineML::<Impl, IMPL_OFFSET>,
            BreakLineW: BreakLineW::<Impl, IMPL_OFFSET>,
            BreakLineA: BreakLineA::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangLineBreakConsole as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringImpl: Sized {
    fn Sync();
    fn GetLength();
    fn SetMLStr();
    fn GetMLStr();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringVtbl {
        unsafe extern "system" fn Sync<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoaccess: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLength<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut *mut ::core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Sync: Sync::<Impl, IMPL_OFFSET>,
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            SetMLStr: SetMLStr::<Impl, IMPL_OFFSET>,
            GetMLStr: GetMLStr::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangString as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringAStrImpl: Sized + IMLangStringImpl {
    fn SetAStr();
    fn SetStrBufA();
    fn GetAStr();
    fn GetStrBufA();
    fn LockAStr();
    fn UnlockAStr();
    fn SetLocale();
    fn GetLocale();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringAStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringAStrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringAStrVtbl {
        unsafe extern "system" fn SetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *mut u32, pszdest: super::Foundation::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut super::Foundation::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMLangStringVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAStr: SetAStr::<Impl, IMPL_OFFSET>,
            SetStrBufA: SetStrBufA::<Impl, IMPL_OFFSET>,
            GetAStr: GetAStr::<Impl, IMPL_OFFSET>,
            GetStrBufA: GetStrBufA::<Impl, IMPL_OFFSET>,
            LockAStr: LockAStr::<Impl, IMPL_OFFSET>,
            UnlockAStr: UnlockAStr::<Impl, IMPL_OFFSET>,
            SetLocale: SetLocale::<Impl, IMPL_OFFSET>,
            GetLocale: GetLocale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangStringAStr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringBufAImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringBufAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringBufAVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut super::Foundation::CHAR, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            LockBuf: LockBuf::<Impl, IMPL_OFFSET>,
            UnlockBuf: UnlockBuf::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangStringBufA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringBufWImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringBufWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringBufWVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PWSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            LockBuf: LockBuf::<Impl, IMPL_OFFSET>,
            UnlockBuf: UnlockBuf::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangStringBufW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringWStrImpl: Sized + IMLangStringImpl {
    fn SetWStr();
    fn SetStrBufW();
    fn GetWStr();
    fn GetStrBufW();
    fn LockWStr();
    fn UnlockWStr();
    fn SetLocale();
    fn GetLocale();
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringWStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringWStrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringWStrVtbl {
        unsafe extern "system" fn SetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: super::Foundation::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut super::Foundation::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMLangStringVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetWStr: SetWStr::<Impl, IMPL_OFFSET>,
            SetStrBufW: SetStrBufW::<Impl, IMPL_OFFSET>,
            GetWStr: GetWStr::<Impl, IMPL_OFFSET>,
            GetStrBufW: GetStrBufW::<Impl, IMPL_OFFSET>,
            LockWStr: LockWStr::<Impl, IMPL_OFFSET>,
            UnlockWStr: UnlockWStr::<Impl, IMPL_OFFSET>,
            SetLocale: SetLocale::<Impl, IMPL_OFFSET>,
            GetLocale: GetLocale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMLangStringWStr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiLanguageImpl: Sized {
    fn GetNumberOfCodePageInfo();
    fn GetCodePageInfo();
    fn GetFamilyCodePage();
    fn EnumCodePages();
    fn GetCharsetInfo();
    fn IsConvertible();
    fn ConvertString();
    fn ConvertStringToUnicode();
    fn ConvertStringFromUnicode();
    fn ConvertStringReset();
    fn GetRfc1766FromLcid();
    fn GetLcidFromRfc1766();
    fn EnumRfc1766();
    fn GetRfc1766Info();
    fn CreateConvertCharset();
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguageVtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Impl, IMPL_OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Impl, IMPL_OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Impl, IMPL_OFFSET>,
            EnumCodePages: EnumCodePages::<Impl, IMPL_OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Impl, IMPL_OFFSET>,
            IsConvertible: IsConvertible::<Impl, IMPL_OFFSET>,
            ConvertString: ConvertString::<Impl, IMPL_OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Impl, IMPL_OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Impl, IMPL_OFFSET>,
            ConvertStringReset: ConvertStringReset::<Impl, IMPL_OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Impl, IMPL_OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Impl, IMPL_OFFSET>,
            EnumRfc1766: EnumRfc1766::<Impl, IMPL_OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Impl, IMPL_OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiLanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMultiLanguage2Impl: Sized {
    fn GetNumberOfCodePageInfo();
    fn GetCodePageInfo();
    fn GetFamilyCodePage();
    fn EnumCodePages();
    fn GetCharsetInfo();
    fn IsConvertible();
    fn ConvertString();
    fn ConvertStringToUnicode();
    fn ConvertStringFromUnicode();
    fn ConvertStringReset();
    fn GetRfc1766FromLcid();
    fn GetLcidFromRfc1766();
    fn EnumRfc1766();
    fn GetRfc1766Info();
    fn CreateConvertCharset();
    fn ConvertStringInIStream();
    fn ConvertStringToUnicodeEx();
    fn ConvertStringFromUnicodeEx();
    fn DetectCodepageInIStream();
    fn DetectInputCodepage();
    fn ValidateCodePage();
    fn GetCodePageDescription();
    fn IsCodePageInstallable();
    fn SetMimeDBSource();
    fn GetNumberOfScripts();
    fn EnumScripts();
    fn ValidateCodePageEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultiLanguage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguage2Vtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: ::windows::core::RawPtr, pstmout: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectCodepageInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: ::windows::core::RawPtr, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectInputCodepage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodePageDescription<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCodePageInstallable<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMimeDBSource<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsource: MIMECONTF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumberOfScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnscripts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateCodePageEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Impl, IMPL_OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Impl, IMPL_OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Impl, IMPL_OFFSET>,
            EnumCodePages: EnumCodePages::<Impl, IMPL_OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Impl, IMPL_OFFSET>,
            IsConvertible: IsConvertible::<Impl, IMPL_OFFSET>,
            ConvertString: ConvertString::<Impl, IMPL_OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Impl, IMPL_OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Impl, IMPL_OFFSET>,
            ConvertStringReset: ConvertStringReset::<Impl, IMPL_OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Impl, IMPL_OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Impl, IMPL_OFFSET>,
            EnumRfc1766: EnumRfc1766::<Impl, IMPL_OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Impl, IMPL_OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Impl, IMPL_OFFSET>,
            ConvertStringInIStream: ConvertStringInIStream::<Impl, IMPL_OFFSET>,
            ConvertStringToUnicodeEx: ConvertStringToUnicodeEx::<Impl, IMPL_OFFSET>,
            ConvertStringFromUnicodeEx: ConvertStringFromUnicodeEx::<Impl, IMPL_OFFSET>,
            DetectCodepageInIStream: DetectCodepageInIStream::<Impl, IMPL_OFFSET>,
            DetectInputCodepage: DetectInputCodepage::<Impl, IMPL_OFFSET>,
            ValidateCodePage: ValidateCodePage::<Impl, IMPL_OFFSET>,
            GetCodePageDescription: GetCodePageDescription::<Impl, IMPL_OFFSET>,
            IsCodePageInstallable: IsCodePageInstallable::<Impl, IMPL_OFFSET>,
            SetMimeDBSource: SetMimeDBSource::<Impl, IMPL_OFFSET>,
            GetNumberOfScripts: GetNumberOfScripts::<Impl, IMPL_OFFSET>,
            EnumScripts: EnumScripts::<Impl, IMPL_OFFSET>,
            ValidateCodePageEx: ValidateCodePageEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiLanguage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMultiLanguage3Impl: Sized + IMultiLanguage2Impl {
    fn DetectOutboundCodePage();
    fn DetectOutboundCodePageInIStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultiLanguage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguage3Vtbl {
        unsafe extern "system" fn DetectOutboundCodePage<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pstrin: ::windows::core::RawPtr, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMultiLanguage2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DetectOutboundCodePage: DetectOutboundCodePage::<Impl, IMPL_OFFSET>,
            DetectOutboundCodePageInIStream: DetectOutboundCodePageInIStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiLanguage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOptionDescriptionImpl: Sized {
    fn Id();
    fn Heading();
    fn Description();
    fn Labels();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOptionDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOptionDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOptionDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Heading<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Labels<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Heading: Heading::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Labels: Labels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOptionDescription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckProviderImpl: Sized {
    fn LanguageTag();
    fn Check();
    fn Suggest();
    fn GetOptionValue();
    fn SetOptionValue();
    fn OptionIds();
    fn Id();
    fn LocalizedName();
    fn GetOptionDescription();
    fn InitializeWordlist();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckProviderVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeWordlist<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LanguageTag: LanguageTag::<Impl, IMPL_OFFSET>,
            Check: Check::<Impl, IMPL_OFFSET>,
            Suggest: Suggest::<Impl, IMPL_OFFSET>,
            GetOptionValue: GetOptionValue::<Impl, IMPL_OFFSET>,
            SetOptionValue: SetOptionValue::<Impl, IMPL_OFFSET>,
            OptionIds: OptionIds::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            LocalizedName: LocalizedName::<Impl, IMPL_OFFSET>,
            GetOptionDescription: GetOptionDescription::<Impl, IMPL_OFFSET>,
            InitializeWordlist: InitializeWordlist::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellCheckProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckProviderFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellCheckProvider();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckProviderFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            CreateSpellCheckProvider: CreateSpellCheckProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellCheckProviderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckerImpl: Sized {
    fn LanguageTag();
    fn Check();
    fn Suggest();
    fn Add();
    fn Ignore();
    fn AutoCorrect();
    fn GetOptionValue();
    fn OptionIds();
    fn Id();
    fn LocalizedName();
    fn SpellCheckerChanged();
    fn RemoveSpellCheckerChanged();
    fn GetOptionDescription();
    fn ComprehensiveCheck();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ignore<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoCorrect<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::Foundation::PWSTR, to: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComprehensiveCheck<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LanguageTag: LanguageTag::<Impl, IMPL_OFFSET>,
            Check: Check::<Impl, IMPL_OFFSET>,
            Suggest: Suggest::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Ignore: Ignore::<Impl, IMPL_OFFSET>,
            AutoCorrect: AutoCorrect::<Impl, IMPL_OFFSET>,
            GetOptionValue: GetOptionValue::<Impl, IMPL_OFFSET>,
            OptionIds: OptionIds::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            LocalizedName: LocalizedName::<Impl, IMPL_OFFSET>,
            SpellCheckerChanged: SpellCheckerChanged::<Impl, IMPL_OFFSET>,
            RemoveSpellCheckerChanged: RemoveSpellCheckerChanged::<Impl, IMPL_OFFSET>,
            GetOptionDescription: GetOptionDescription::<Impl, IMPL_OFFSET>,
            ComprehensiveCheck: ComprehensiveCheck::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellChecker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellChecker2Impl: Sized + ISpellCheckerImpl {
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellChecker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellChecker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellChecker2Vtbl {
        unsafe extern "system" fn Remove<Impl: ISpellChecker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ISpellCheckerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Remove: Remove::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellChecker2 as ::windows::core::Interface>::IID
    }
}
pub trait ISpellCheckerChangedEventHandlerImpl: Sized {
    fn Invoke();
}
impl ISpellCheckerChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerChangedEventHandlerVtbl {
        unsafe extern "system" fn Invoke<Impl: ISpellCheckerChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellCheckerChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckerFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellChecker();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSpellChecker<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Impl, IMPL_OFFSET>,
            IsSupported: IsSupported::<Impl, IMPL_OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpellingErrorImpl: Sized {
    fn StartIndex();
    fn Length();
    fn CorrectiveAction();
    fn Replacement();
}
#[cfg(feature = "Win32_Foundation")]
impl ISpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellingErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellingErrorVtbl {
        unsafe extern "system" fn StartIndex<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Length<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrectiveAction<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Replacement<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartIndex: StartIndex::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            CorrectiveAction: CorrectiveAction::<Impl, IMPL_OFFSET>,
            Replacement: Replacement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellingError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUserDictionariesRegistrarImpl: Sized {
    fn RegisterUserDictionary();
    fn UnregisterUserDictionary();
}
#[cfg(feature = "Win32_Foundation")]
impl IUserDictionariesRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDictionariesRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDictionariesRegistrarVtbl {
        unsafe extern "system" fn RegisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterUserDictionary: RegisterUserDictionary::<Impl, IMPL_OFFSET>,
            UnregisterUserDictionary: UnregisterUserDictionary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDictionariesRegistrar as ::windows::core::Interface>::IID
    }
}
